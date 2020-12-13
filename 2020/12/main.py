#!/usr/bin/env python3

CLOCK = ['E', 'N', 'W', 'S']
DIRS = {'E': [1, 0],
        'N': [0, 1],
        'W': [-1, 0],
        'S': [0, -1]}


def add(pos1, pos2, n):
    return [p1 + p2 * n for p1, p2 in zip(pos1, pos2)]


def get_solution2(navs):
    pos = [0, 0]
    way = [10, 1]
    for d, n in navs:
        if d in DIRS:
            way = add(way, DIRS[d], n)
        elif d == 'F':
            pos = add(pos, way, n)
        elif d == 'L':
            for i in range(n // 90):
                way = [-way[1], way[0]]
        elif d == 'R':
            for i in range(n // 90):
                way = [way[1], -way[0]]
    return abs(pos[0]) + abs(pos[1])


def get_solution1(navs):
    pos = [0, 0]
    clock_dir = 0
    for d, n in navs:
        if d == 'F':
            d = CLOCK[clock_dir]

        if d in DIRS:
            pos = add(pos, DIRS[d], n)
        elif d == 'L':
            clock_dir = (clock_dir + n // 90) % 4
        elif d == 'R':
            clock_dir = (clock_dir - n // 90) % 4
    return abs(pos[0]) + abs(pos[1])


if __name__ == '__main__':
    with open('input') as f:
        navs = [(l[0], int(l.strip()[1:])) for l in f]

    print(get_solution1(navs))
    print(get_solution2(navs))
