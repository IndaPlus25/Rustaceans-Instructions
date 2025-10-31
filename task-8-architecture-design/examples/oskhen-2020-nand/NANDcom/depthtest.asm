sys READ r0
sys READ r1
START r0 r1
AND r0 r4 r5
AND r5 r1 r5
END r5
sys WRITE r5