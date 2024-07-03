@256
D=A
@SP
M=D


@5
D=A
@SP
A=M
M=D
@SP
M=M+1

@6
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
D=D-M
@j0
D;JEQ

@SP
A=M-1
M=0
@j0end
0;JMP

(j0)
@SP
A=M-1
M=-1

(j0end)


