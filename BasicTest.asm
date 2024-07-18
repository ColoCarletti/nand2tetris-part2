// Initialize stack pointer
@256
D=A
@SP
M=D

// Push Constant 57
@57
D=A
@SP
A=M
M=D
@SP
M=M+1

// Push Constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1

// Push Constant 53
@53
D=A
@SP
A=M
M=D
@SP
M=M+1

// Add
@SP
AM=M-1
D=M
@SP
A=M-1
M=M+D

// Push Constant 112
@112
D=A
@SP
A=M
M=D
@SP
M=M+1

// Sub
@SP
AM=M-1
D=M
@SP
A=M-1
M=M-D

// Neg
@SP
AM=M-1
D=M
M=-D

// And
@SP
AM=M-1
D=M
@SP
A=M-1
M=M&D

// Push Constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1

// Or
@SP
AM=M-1
D=M
@SP
A=M-1
M=M|D

// Not
@SP
AM=M-1
D=M
M=!D

// Final loop
(end)
@end
0;JMP
