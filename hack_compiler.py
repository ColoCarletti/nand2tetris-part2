import sys
asm_file = sys.argv[1]

current_free_address = 16

symbols = {
    "R0": 0,
    "R1": 1,
    "R2": 2,
    "R3": 3,
    "R4": 4,
    "R5": 5,
    "R6": 6,
    "R7": 7,
    "R8": 8,
    "R9": 9,
    "R10": 10,
    "R11": 11,
    "R12": 12,
    "R13": 13,
    "R14": 14,
    "R15": 15,
    "SCREEN": 16384,
    "KBD": 24576,
    "SP": 0,
    "LCL": 1,
    "ARG": 2,
    "THIS": 3,
    "THAT": 4,
}

dest_bin = {
    0: '000',
    'M': '001',
    'D': '010',
    'MD': '011',
    'A': '100',
    'AM': '101',
    'AD': '110',
    'AMD': '111',
}

jump_bin = {
    0: '000',
    'JGT': '001',
    'JEQ': '010',
    'JGE': '011',
    'GLT': '100',
    'JNE': '101',
    'JLE': '110',
    'JMP': '111',
}

comp_bin = {
    '0': '0101010',
    '1': '0111111',
    '-1': '0111010',
    'D': '0001100',
    'A': '0110000',
    '!D': '0001101',
    '!A': '0110001',
    '-D': '0001111',
    '-A': '0110011',
    'D+1': '0011111',
    'A+1': '0110111',
    'D-1': '0001110',
    'A-1': '0110010',
    'D+A': '0000010',
    'D-A': '0010011',
    'A-D': '0000111',
    'D&A': '0000000',
    'D|A': '0010101',
    'M': '1110000',
    '!M': '1110001',
    '-M': '1110011',
    'M+1': '1110111',
    'M-1': '1110010',
    'D+M': '1000010',
    'D-M': '1010011',
    'M-D': '1000111',
    'D&M': '1000000',
    'D|M': '1010101',
}
def next_instruction_line(line_number, assembly):
    return line_number if not assembly[line_number].startswith('(') else next_instruction_line(line_number + 1, assembly)

def translate(instruction):
    if instruction[0] == '@':
        return translate_a_instruction(instruction[1:])
    else:
        return translate_c_instruction(instruction)

def translate_c_instruction(instruction):
    dest, comp, jump = 0, 0, 0
    p = instruction.split(';')
    if len(p) == 2:
        jump = p[1]
    if '=' in p[0]:
        dest, comp = p[0].split("=")
    else:
        comp = p[0]
    return f'111{comp_bin[comp]}{dest_bin[dest]}{jump_bin[jump]}'

def translate_a_instruction(address):
    if address.isnumeric():
        return f'0{int(address):015b}'
    else:
        if address not in symbols:
            global current_free_address
            symbols[address] = current_free_address
            current_free_address += 1
        return f'0{symbols[address]:015b}'

def main():
    with open(asm_file, "r") as f:
        assembly = [line.split('//')[0].strip() for line in f if line.split('//')[0].strip()]

    # FIRST PASS
    filtered_assembly= []

    for line in assembly:
        if line.startswith('('):
            symbols[line[1:-1]] = len(filtered_assembly)
        else:
            filtered_assembly.append(line)

    # SECOND PASS
    machine_code = list(map(translate, filtered_assembly))

    with open(f'{asm_file[:-4]}.hack', 'w') as f:
        f.writelines("%s\n" % l for l in machine_code)

if __name__ == "__main__":
    main()
