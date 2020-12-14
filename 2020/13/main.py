#!/usr/bin/env python3

import math


def lcm(a, b):
    return abs(a * b) // math.gcd(a, b)


def get_solution1(basetime, buses):
    bustimes = [int(x) for x in buses if x != 'x']
    waittimes = [bustime - basetime % bustime for bustime in bustimes]
    best_bus_index = waittimes.index(min(waittimes))
    return bustimes[best_bus_index] * waittimes[best_bus_index]


def get_solution2(buses):
    buses_by_offset = [(int(x), i) for i, x in enumerate(buses) if x != 'x']

    base, step = 1, 1
    for id, pos in buses_by_offset:
        while (base + pos) % id:
            base += step
        step = lcm(step, id)
    return base


if __name__ == '__main__':
    with open('input') as f:
        basetime = int(f.readline().strip())
        buses = f.read().strip().split(',')

    print(get_solution1(basetime, buses))
    print(get_solution2(buses))
