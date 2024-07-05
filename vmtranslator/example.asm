// Initialize stack pointer
@256
D=A
@SP
M=D

// Push Constant 6
@6
D=A
@SP
A=M
M=D
@SP
M=M+1

// Push Constant 5
@5
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
M=D+M

// Final loop
(end)
@end
0;JMP
