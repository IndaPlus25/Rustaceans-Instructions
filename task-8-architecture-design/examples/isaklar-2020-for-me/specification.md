#  FOR ME 👉👈 language specification

Assembly language for the `Y(eet)L(mao)69-series` chipset, designed by Malte Blomqvist (maltebl) and Isak Larsson (isaklar).

Instructions, registers and syscall codes are written exclusively in Unicode emojis, while immidiate values are in regular text. Labels can, but should not unless absolutely necessary, be in text.

Comments are prefaced with a `#`.

The file endings are `.forme` for code and  `.formexe` for the excecutable.

## Instructions

| **Type** | **Encoding** |
|:---------|:-------------|
| Arithmatic | `op<7:5>, rt<4:3>, rs<2:1>, imm<0>` |
| Immidiate | `op<7:5>, rt<4:3>, imm<2:0>` |
| Jump     | `op<7:5>, addr<4:0>` |
| Special  | `op<7:5>, code<4:0>` |

## Arithmetic Instructions
| **Instruction** | **Description** |
|:----------------|:----------------|
| `💘` | `rt = rt + rs + imm` |
| `💔` | `rt = rt - rs - imm` |
| `🤔` | skip next line if `rt != rs` |

## Immidiate instructions

| **Instruction** | **Description** |
|:----------------|:----------------|
| `👆` | `rt = rt + imm` |
| `👇` | `rt = rt - imm` |
| `👈` | `rt = imm` |

## Jump instructions 

| **Instruction** | **Description** |
|:----------------|:----------------|
| `♿` | jump to label |

## Special instructions
| **Instruction** | **Description** |
|:----------------|:----------------|
| `🤖` | syscall with code |
| **Call code**| |
| `📢` | writes integer value of `👀` to output |
| `📜` | gets integer value from standard I/O stream and writes to `👀` |
| `🔪` | terminates program |

## Registers
| **Symbol** | **Description** |
|:----------------|:----------------|
| `🍩` | `i32` |
| `👀` | `i32` used by I/O |
| `🍎` | `i32` |
| `🍊` | `i32` |

## Declarations
| **Symbol** | **Description** |
|:----------------|:----------------|
| 🔓 | start new macro declaration |
| 🔒 | end macro declaration |
| 💬 | indicates where the code starts |
