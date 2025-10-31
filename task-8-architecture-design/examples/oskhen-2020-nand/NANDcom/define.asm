.define MOVE rs rt
sys STACK PUSH rt
sys STACK POP rs
.end_define

.define AND rs rt rd
START rt rd
NAND rt rd
NAND rn rn
END rs
.end_define

.define OR rs rt rd
START rt rd 
NAND rt rt
sys STACK PUSH rn
NAND rd rd
sys STACK POP r2
NAND rn r2
END rs
.end_define

.define NOR rs rt rd 
START rt rd
NAND rt rt
STACK PUSH rn
NAND rd rd 
sys STACK POP r2
NAND rn r2
NAND rn rn
END rs
.end_define

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

.define NOT rs rt rd
START rt rd
NAND rt rt
END rs
.end_define

.define XNOR rs rt rd 
START rt rd 
NAND rt rt
sys STACK PUSH rn
NAND rd rd
sys STACK POP r2
NAND r2 rn
sys STACK PUSH rn
NAND rd rt 
sys STACK POP r2
NAND r2 rn
END rs
.end_define

.define ADDER rt rs0 rs1 rs2
sys STACK PUSH rs2
START rs0 rs1
sys STACK POP r4
NAND r0 r1      # U1
sys STACK PUSH rn
sys STACK PUSH rn
NAND rn r1      # U3
sys STACK POP r2
sys STACK PUSH rn
NAND r0 r2      # U2
sys STACK POP r2
NAND rn r2      # U4
sys STACK PUSH rn
NAND r4 rn      # U5
sys STACK POP r2
sys STACK PUSH rn
sys STACK PUSH rn
NAND rn r2      # U6
sys STACK POP r2
sys STACK PUSH rn
NAND r4 r2      # U7
sys STACK POP r2
NAND rn r2      # U8
sys STACK POP r2
sys STACK POP r3
sys STACK PUSH rn
NAND r3 r2      # U9 rn is carry now 
sys STACK POP r2    # r2 is sum
sys STACK PUSH rn       
sys STACK PUSH r2
sys STACK POP rn
END rt
sys STACK POP rs2
.end_define

.define 2-BIT-ADDER rt rs0 rs1 rs2
sys STACK PUSH rs0
sys STACK PUSH rs1
sys STACK PUSH rs2

START rs0 rs1
sys STACK POP r2
MOVE r4 r0 
BIT READ 0 
MOVE r0 r5
MOVE r4 r1
BIT READ 0
MOVE r1 r5

ADDER r3 r0 r1 r2

MOVE r5 r3
MOVE r4 rn
BIT WRITE 0
MOVE rn r4
sys STACK POP r1
sys STACK POP r0
MOVE r4 r0 
BIT READ 1 
MOVE r0 r5
MOVE r4 r1
BIT READ 1
MOVE r1 r5
ADDER r3 r0 r1 r2
MOVE r5 r3
MOVE r4 rn
BIT WRITE 1
MOVE rn r4
sys STACK PUSH r2

END rt
sys STACK POP r2
.end_define

.define 4-BIT-ADDER rt rs0 rs1 rs2
sys STACK PUSH rs0
sys STACK PUSH rs1
sys STACK PUSH rs2
START rs0 rs1
sys STACK POP r2 #carry


2-BIT-ADDER r3 r0 r1 r2
sys WRITE r3


MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 0
MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 1
MOVE rn r4


MOVE r4 r0
BIT READ 2  # 3rd bit in r5
BIT WRITE 0 # 1rd bit of r4 = r5
BIT READ 3
BIT WRITE 1
MOVE r0 r4

MOVE r4 r1
BIT READ 2  # 3rd bit in r5
BIT WRITE 0 # 1rd bit of r4 = r5
BIT READ 3
BIT WRITE 1
MOVE r1 r4

2-BIT-ADDER r3 r0 r1 r2
sys WRITE r3

MOVE r4 r3
BIT READ 0
MOVE r4 rn
BIT WRITE 2
MOVE r4 r3
BIT READ 1
MOVE r4 rn
BIT WRITE 3
MOVE rn r4

sys WRITE r4

sys STACK PUSH r2

END rt
sys STACK POP rs2
.end_define