#!/usr/bin/env python3

import re


MEM_PATTERN = re.compile(r'^(mem)\[(\d*)\] = (.*)$')


def read_instrs(lines):
    result = []
    for l in lines:
        l = l.strip()
        match = re.match(MEM_PATTERN, l)
        if match:
            groups = match.groups()
            result.append((groups[0], int(groups[1]), int(groups[2])))
        else:
            result.append(tuple(l.split(' = ')))
    return result


def apply_val_mask(mask, n):
    b = format(n, '036b')
    return int(''.join([b[i] if m == 'X' else m for i, m in enumerate(mask)]), 2)


def apply_addr_mask(mask, mem):
    b = format(mem, '036b')
    return ''.join(['X' if m == 'X' else str(int(b[i]) or int(m))
                    for i, m in enumerate(mask)])


def unfloat_addr(addr):
    result = []
    degrees = addr.count('X')
    for patch_dec in range(pow(2, degrees)):
        actual_addr = list(addr)
        for patch_bit in '{:0{d}b}'.format(patch_dec, d=degrees):
            actual_addr[actual_addr.index('X')] = patch_bit
        result.append(''.join(actual_addr))
    return result


def get_solution1(instrs):
    mask = '0' * 36
    mem = {}
    for instr in instrs:
        if instr[0] == 'mask':
            mask = instr[1]
        else:
            mem[instr[1]] = apply_val_mask(mask, instr[2])
    return sum(mem.values())


def get_solution2(instrs):
    mask = '0' * 36
    mem = {}
    for instr in instrs:
        if instr[0] == 'mask':
            mask = instr[1]
        else:
            floating_addr = apply_addr_mask(mask, instr[1])
            for addr in unfloat_addr(floating_addr):
                mem[int(addr, 2)] = instr[2]
    return sum(mem.values())


if __name__ == '__main__':
    with open('input') as f:
        instrs = read_instrs(f)

    print(get_solution1(instrs))
    print(get_solution2(instrs))
