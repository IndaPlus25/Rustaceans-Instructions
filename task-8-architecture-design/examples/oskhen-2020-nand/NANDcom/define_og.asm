.define MOVE rs rt
START rs rt
AND rt rt
END rs
.end_define

.define AND r0 r1 rt
START r0 r1
NAND r0 r1
NAND rn rn
END rt
.end_define

.define OR r0 r1 rt
START r0 r1 
NAND r0 r0
sys STACK PUSH rn
NAND r1 r1
sys STACK POP r2
NAND rn r2
END rt
.end_define

.define NOR r0 r1 rt
START r0 r1
NAND r0 r0
STACK PUSH rn
NAND r1 r1 
sys STACK POP r2
NAND rn r2
NAND rn rn
.end_define

.define XOR r0 r1 rt
START r0 r1
NAND r0 r1
sys STACK PUSH rn
NAND r0 rn
sys STACK POP r2
sys STACK PUSH rn
NAND r1 r2
sys STACK POP r2
NAND r2 rn
END rt
.end_define

.define NOT r0 r1 rt
START r0 r1
NAND r0 r0
END rt
.end_define

.define XNOR r0 r1 rt
START r0 r1 
NAND r0 r0
sys STACK PUSH rn
NAND r1 r1
sys STACK POP r2
NAND r2 rn
sys STACK PUSH rn
NAND r1 r0 
sys STACK POP r2
NAND r2 rn
END rt
.end_define

.define ADDER r0 r1 r4 rt
START r0 r1
NAND r0 r1
sys STACK PUSH rn
sys STACK PUSH rn
NAND rn r1
sys STACK POP r2
sys STACK PUSH rn
NAND r0 r2
sys STACK POP r2
NAND rn r2
sys STACK PUSH rn
NAND r4 rn
sys STACK POP r2
sys STACK PUSH rn
sys STACK PUSH rn
NAND rn r2
sys STACK POP r2
sys STACK PUSH rn
NAND r4 r2
sys STACK POP r2
NAND rn r2
sys STACK POP r2
sys STACK POP r3
sys STACK PUSH rn
NAND r3 r2
sys STACK POP r2
sys STACK PUSH rn
sys STACK POP r4
sys STACK PUSH r2
sys STACK POP rn
END rt
.end_define

.define 2-BIT-ADDER r0 r1 r2 rt
sys STACK PUSH r0
sys STACK PUSH r1
MOVE r4 r0 
BIT READ 0 
MOVE r0 r5
MOVE r4 r1
BIT READ 0
MOVE r1 r5
ADDER r0 r1 r2 rt
sys STACK POP r2
sys STACK POP r1
sys STACK POP r0

sys STACK PUSH r0
sys STACK PUSH r1
MOVE r4 r0 
BIT READ 1 
MOVE r0 r5
MOVE r4 r1
BIT READ 1
MOVE r1 r5
ADDER r0 r1 r2 rt
END rt
.end_define


