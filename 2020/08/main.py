#!/usr/bin/env python3


def read_program(lines):
    result = []
    for l in lines:
        op, arg = l.strip().split(' ')
        result.append((op, int(arg)))
    return result


def step(program, pc, acc):
    op, arg = program[pc]

    if op == 'nop':
        return pc + 1, acc
    elif op == 'acc':
        return pc + 1, acc + arg
    elif op == 'jmp':
        return pc + arg, acc


def execute(program):
    visited = set()
    pc = 0
    acc = 0

    while pc not in visited and 0 <= pc < len(program):
        visited.add(pc)
        pc, acc = step(program, pc, acc)
    return pc, acc


def corrupt(program, i):
    op, arg = program[i]
    if op == 'nop':
        program[i] = ('jmp', arg)
    elif op == 'jmp':
        program[i] = ('nop', arg)


def get_solution1(program):
    _, acc = execute(program)
    return acc


def get_solution2(program):
    for i, (op, arg) in enumerate(program):
        if op == 'acc':
            continue
        corrupt(program, i)
        pc, acc = execute(program)
        corrupt(program, i)
        if pc == len(program):
            return acc


if __name__ == '__main__':
    with open('input') as f:
        program = read_program(f)

    print(get_solution1(program))
    print(get_solution2(program))
