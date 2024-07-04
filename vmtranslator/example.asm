// Initialize stack pointer
@256
D=A
@SP
M=D

@6
D=A
@SP
A=M
M=D
@SP
M=M+1

@5
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@SP
A=M-1
M=D+M

(end)
@end
0;JMP
