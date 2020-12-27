#!/usr/bin/env python3

from collections import Counter


TILE_DIRS = {
    'e': (1, 0),
    'ne': (1, 1),
    'nw': (0, 1),
    'w': (-1, 0),
    'sw': (-1, -1),
    'se': (0, -1),
}


def read_instructions(lines):
    instructions = []
    for line in lines:
        steps = []
        step = ''
        for c in line:
            if step:
                steps.append(step + c)
                step = ''
            elif c in ['e', 'w']:
                steps.append(c)
            else:
                step += c
        instructions.append(steps)
    return instructions


def exec_steps(steps):
    x, y = 0, 0
    for step in steps:
        dx, dy = TILE_DIRS[step]
        x += dx
        y += dy
    return x, y


def count_neighbors(x, y, black_tiles):
    result = 0
    for dx, dy in TILE_DIRS.values():
        if (x + dx, y + dy) in black_tiles:
            result += 1
    return result


def do_day(black_tiles):
    result = set()
    min_x = min([t[0] for t in black_tiles])
    max_x = max([t[0] for t in black_tiles])
    min_y = min([t[1] for t in black_tiles])
    max_y = max([t[1] for t in black_tiles])
    for x in range(min_x - 1, max_x + 2):
        for y in range(min_y - 1, max_y + 2):
            count = count_neighbors(x, y, black_tiles)
            if count == 2 or (count == 1 and (x, y) in black_tiles):
                result.add((x, y))
    return result


def get_solution1(instructions):
    flips = []
    for steps in instructions:
        flips.append(exec_steps(steps))

    black_tiles = set(k for k, v in Counter(flips).items() if v % 2 == 1)
    return len(black_tiles), black_tiles


def get_solution2(black_tiles):
    for i in range(100):
        black_tiles = do_day(black_tiles)
    return len(black_tiles)


if __name__ == '__main__':
    with open('input') as f:
        instructions = read_instructions(f)

    sol, black_tiles = get_solution1(instructions)
    print(sol)
    print(get_solution2(black_tiles))
