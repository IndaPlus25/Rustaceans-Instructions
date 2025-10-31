### Version 1
sys READ r0
sys READ r1

START r0 r1
NAND r0 r1
NAND rn rn
END r2

sys PRINT r2

### Version 2

.define AND rs rt ru

START rs rt
NAND r0 r1
NAND rn rn
END ru

.enddefine

sys READ r0
sys READ r1

AND r0 r1 r2

sys PRINT r2


01 01 010 0

01 01 011 0

10 010 011

00 010 011

00 001 001

11 100 000


010101000101011010010011000100110000100111100000