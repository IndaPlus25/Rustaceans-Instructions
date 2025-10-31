# DD1337 Week 8 | Architecture Design

_Don't forget to take a look out for potential complementary work from your previous task submissions._

## Assembly Language Design

Time to face it, MIPS, especially MARS, is boring! You, yes YOU, can surely do a better job than that! For this reason, make a show of your programming skills by designing your own assembly language. 

Your language must be an 32-bit assembly language with 8-bit instructions. See the [MIPS reference sheet](https://www.kth.se/social/files/563c63c9f276547044e8695f/mips-ref-sheet.pdf) and take inspiration from the MIPS 32-bit instruction encoding. Your instruction encoding determins:

| **Code** | **Size and Flexibility** |
|----------|--------------------------|
| `op` _(operation)_ | 2-3 bits operation code gives room for 4-8 instructions. |
| `rs`/`rt`/`rd` _(registry)_ | 2 bits registry addresses gives room for 4 registries. |
| `imm` _(immidiate)_ | 1-5 bits immidiate values give room (at most) for the unsigned values 0-31 or the signed values (-16)-15. |
| `label` _(jump address)_ | 4-6 bits jump address gives room for jumps (branching) of max 16-64 instructions (lines) per jump. Remember that jump chaining is tedious. |

Think carefully of how your instructions are encoded. More registries may mean a smaller instruction set or smaller immidiates. The above table is only an example for how you may divide your availiable bits.

Your registries should be 32 bits in size, meaning values ranging between $[0, 2^{32})$ (unsigned) and $[-2^{31}, 2^{31})$ (signed).

Lastly, remember to give your language a killer name!!!

## Assignment

Summary:
- Fulfill one of the assignment levels below.
- Write a factorial calculator as described [below](#language-capabilities).

### Prepare Assignment

This is a group exercise! Please find one or two partners.

1) Create a repository named `task-8-<KTH_ID>-<KTH_ID>`.
2) Clone your regular assignment repository.
```sh
git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-8.git
```
3) Add the upstream for `task-8-<KTH_ID>-<KTH_ID>` to your local repository.
```sh
git remote add plus git@github.com:IndaPlus25/task-8-<KTH_ID>-<KTH_ID>.git
```
4) The repository is your group workspace. If you wish to complete the regular task as well, keep your regular task in a seperate branch which you only push to inda-25.
5) Agree on a common git methodology/workflow to avoid merch conflicts.

For help with inspiration, see `./examples`.

### Assignment Levels

Higher level equals more alpha!

1) Copy the design of _Bacon Borde Vara Vegetariskt_ and implement an interpreter for the language.
2) Implement an emulator to complement the already provided assembler for _Bacon Borde Vara Vegetariskt_. See `./bbvv/rust-assembler/README.md` for specifications.
3) Design your own language and implement an interpreter for your language in whatever programming language you want.
4) Design your own language and write an assembler as well as an emulator for your language/architecture in whatever programming language you want.

An interpretor reads a code file and run it instruction for instruction. An assembler reads a code file and outputs a system specific executable file. An emulator reads an executable file and executes it by interpretation.

**@Rustacians** Good character points for whoever implements their solution using Rust! Tagga Rust! 🦀

**@Everyone** The most outrageous solution will get recognition from the fabled Ric himself! Be creative!

### Language Capabilities

Your language has to have the instructions for you to write a program that takes an integer `n` as user input, then calculates and outputs the value `n!`.

To prove your language's capabilities, write this program and include the file in your repository ready to be interpreted or compiled. In addition, include instructions of how to do this.

### _NOTE_

Your language are not allowed to fit multiplication of factorial logic in single instructions. Multiplication by gate logic (add, or, not, xor ...) and addition is allowed.
