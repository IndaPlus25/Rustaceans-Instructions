## Information about barcode
1. Only legal characters are I and l
2. there are 4 registers (reg0 is II and so on (2 bits)) 
3. Mark comments with **ii** *comment* **ii** (interpreter will still check and remove invalid characters if you are incompitent)

## Usage

| .barcode      | Name          | Usage       | Function                                      |
| ------------- |:-------------:| -----------:| ---------------------------------------------:|
| lIII          | ADD           | Add rd, rt  | Add rt to rd                                  |
| llII          | SUB           | Sub rd, rt  | Sub rd by rt                                  |
| lIlI          | MOVE          | Move rd, rt | Move rd to rt                                 |
| IIl           | LI            | LI imm      | Load immediate to reg0                        |
| lll           | JR            | JR imm      | Jump immediate lines (signed)                 |
| Ill           | BNE           | BNE imm     | Branch on not equal (reg2 and reg 3)          |
| IIIIlI        | READ          | READ rd     | Read console to rd                            |
| IIIIII        | WRITE         | WRITE rd    | Write rd                                      |
| IIIIIl        | ADD1          | ADD1 rd     | Add 1 to rd                                   |
| IIIIllII      | SKIP          | SKIP        | Skip next line if (reg2 and 3 are equal)      |
