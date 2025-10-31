# NOTE - PERIODIC COPY OF https://github.com/TophatHackers/NAND - SEE THAT REPO FOR LIVE UPDATES!


- [Description](#description)
  - [Macros](#macros)
- [Specifications](#specifications)
  - [Registers](#registers)
  - [Instructions](#instructions)
    - [NAND](#nand)
    - [SYS](#sys)
      - [STACK](#stack)
      - [READ](#read)
      - [WRITE](#write)
    - [LOOP](#loop)
      - [START](#start)
      - [END](#end)
    - [BIT](#bit)


# Description
Groundbreaking NAND language based on the universal NAND gate.

## Macros
Compiler utilises define macros to build up an arsenal of functions from the one NAND instruction. 
A macro is on the form
```
.define xxx rs rt rd ...
...
...
...
.end_define
``` 
and is then used like `xxx rs rt rd`. The compiler supports recursive .defines, and the sequencing of definitions doesn't matter. For example, here's a XOR macro:
```
.define XOR rs rt rd 
START rt rd
NAND rt rd
sys STACK PUSH rn
NAND rt rn
sys STACK POP r2
sys STACK PUSH rn
NAND rd r2
sys STACK POP r2
NAND r2 rn
END rs
.end_define
```
the XOR can now be used as a regular instruction, for example `XOR r2 r0 r1` is equivalent to `r2 = r0 ^ r1`. XOR can also be used in other macros, and is in every way equivalent to a regular instruction (except for the space it takes up in the binary).

# Specifications

## Registers

| **Register encoding** | **Register name** | **Description**                                                                 |
| :-------------------- | :---------------- | :------------------------------------------------------------------------------ |
| `0`                   | `pc`              | Program counter, points at which instruction to execute                         |
| `1`                   | `rn`              | Used as intermediate output, for example NAND saves to rn and END loads from rn |
| `2`                   | `r0`              | General use register, used as first argument                                    |
| `3`                   | `r1`              | General use register, used as second argument                                   |
| `4`                   | `r2`              | General use register                                                            |
| `5`                   | `r3`              | General use register                                                            |
| `6`                   | `r4`              | General use register, used by BIT instruction to change specific bit            |
| `7`                   | `r5`              | General use register, used by BIT instruction to load the changed bit           |

## Instructions

| **Encoding** | **OPcode** |
| :----------- | :--------- |
| `0`          | `NAND`     |
| `1`          | `SYS`      |
| `2`          | `LOOP`     |
| `3`          | `BIT`      |

### NAND

| **Encoding** | **Description**             |
| :----------- | :-------------------------- |
| `op<7:6>`    | Identifies NAND instruction |
| `rs<5:3>`    | First register              |
| `rt<2:0>`    | Second register             |


NANDS the first register with the second and saves it in rn.

`rn = rs NAND rt`

Example code:
```
NAND r0 r1
NAND rn rn
```
which is equivalent to `rn = r0 & r1`

### SYS

| **ID Encoding** | **Description** |
| :-------------- | :-------------- |
| `0`             | Handle stack    |
| `1`             | Read from stdin |
| `2`             | Write to stdin  |
| `3`             | TBD             |

Syscall instruction, bypasses "NAND is the only instruction" as a necessary feature since we're not actually working on a hardware level but just emulating.
(Also who can be bothered to read/write from/to stdin using only NANDs).

#### STACK

| **Encoding** | **Description**                                             |
| :----------- | :---------------------------------------------------------- |
| `op<7:6>`    | Identifies SYS instruction                                  |
| `id<5:4>`    | Identifies type of syscall                                  |
| `type<3>`    | Type of stack operation, push or pop. `0 = push`, `1 = pop` |
| `rs<2:0>`    | Register to push/pop                                        |

Handles the (global) stack. `PUSH` pushes the given register to stack, `POP` pops the element at the top of the stack to the given register.

Example code:

```
sys STACK PUSH r0
sys STACK POP r1
```
Pushes r0 to the stack, then pops it to r1. Equivalent to `r1 = r0`


#### READ

| **Encoding** | **Description**                        |
| :----------- | :------------------------------------- |
| `op<7:6>`    | Identifies SYS instruction             |
| `id<5:4>`    | Identifies type of syscall             |
| `rs<3:1>`    | Register to read integer from stdin to |
| `<0>`        | TBD, currently padding.                |

Reads an integer from stdin and saves it to given register. 

Example code: `sys READ r0`, sets r0 to given (integer) input.

#### WRITE

| **Encoding** | **Description**                  |
| :----------- | :------------------------------- |
| `op<7:6>`    | Identifies SYS instruction       |
| `id<5:4>`    | Identifies type of syscall       |
| `rs<3:1>`    | Register to write to stdout from |
| `<0>`        | TBD, currently padding.          |

Writes the given register to stdout as an integer.

Example code `sys WRITE r0`, which writes r0 (as an integer) to stdout.

### LOOP

Loop is in reality 2 operations, START and END. They are differentiated by the last 3 bits, if they're all 0 then the operation is identified as END otherwise as START. Be wary of the bug if you try to pass PC as the 2nd argument in START (since PC = 000), this will result in unintended behaviour!

The loop instructions make up the cornerstones on which the entire language stands. Although called "loop", the word subprocess is used internally, and explains what it does better. The emulator works recursively on each subprocess. In other words each subprocess is run contained in and of itself, with the exception of a global stack that allows infinite arguments and return values.

#### START

| **Encoding** | **Description**                    |
| :----------- | :--------------------------------- |
| `op<7:6>`    | Identifies LOOP instruction        |
| `rs<5:3>`    | Register to use as first argument  |
| `rt<2:0>`    | Register to use as second argument |

Start takes in 2 registers which it uses to spawn a subprocess with those as arguments. In other words, it puts rs and rt into r0 and r1 of subprocess (which is identified by matching END, supports infinite (in theory) depth).

Example code:
```
START r3 r4
NAND r0 r1
NAND rn rn
END r2
```
which is equivalent to `r2 = r3 & r4`. This is really useful for building up something equivalent to functions, you pass the required arguments and get returnvalues without having to think about how it was done (blackbox). 

#### END

| **Encoding** | **Description**                                               |
| :----------- | :------------------------------------------------------------ |
| `op<7:6>`    | Identifies LOOP instruction                                   |
| `rs<5:3>`    | Register to use as return value                               |
| `id<2:0>`    | Padding/Identifier, all being 0 differentiates END from START |

Ends a subprocess, and puts rn into rs of parent process. See previous explanations for more details.

### BIT

| **Encoding** | **Description**                                     |
| :----------- | :-------------------------------------------------- |
| `op<7:6>`    | Identifies BIT instruction                          |
| `type<5>`    | Identifies type of BIT operation, READ or WRITE     |
| `imm<4:0>`   | Immediate that signifies index of bit to READ/WRITE |

Uses the registers r4 and r5.

READ:

Sets r5 to `r4[imm]`

WRITE:

sets `r4[imm]` to the LSB of r5

Example code:

```
sys READ r4 # read "1"
sys READ r5 # read "1"

BIT WRITE 5

sys WRITE r4
```

Outputs 33 (see given inputs), since it sets the 5th bit (the bit controlling 2^5) to 1, meaning `0b000001` -> `0b100001` or 1 -> 33.









