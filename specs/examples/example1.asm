// Computes RAM[1] = 1+...+RAM[RAM[0]
// Usage: put a numbert in RAM[0]

@16 // RAM[16] represents i
M=1 // i = 1
@17 // RAM[17] represents sum
M=0 // sum = 0

@16
D=M 
@0
D=D-M
@17
D;JGT

@16
D=M
@17
M=D+M // sum = sum + i
@16
M=M+1 // i = i + 1
@4
0;JMP

@17
D=M
@1
M=D // RAM[1] = sum
@21 // program's end
0;JMP // infinite loop - This is a best practise to avoid the program to end and cause errors