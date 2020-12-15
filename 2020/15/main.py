#!/usr/bin/env python3


def get_solution(ns, final_index):
    n_map = {n: i for i, n in enumerate(ns[:-1])}
    next_n = ns[-1]
    for turn in range(len(ns) - 1, final_index - 1):
        previous_i = n_map.get(next_n)
        n_map[next_n] = turn
        if previous_i is not None:
            next_n = turn - previous_i
        else:
            next_n = 0
    return next_n


# The naive approach
def get_solution_slow(ns, final_index):
    for turn in range(len(ns), final_index):
        for i, n in enumerate(ns[-2::-1]):
            if n == ns[-1]:
                break
        else:
            ns.append(0)
            continue

        i = len(ns) - 2 - i
        ns.append(turn - 1 - i)
    return ns[-1]


if __name__ == '__main__':
    with open('input') as f:
        ns = [int(x) for x in f.read().strip().split(',')]

    print(get_solution(ns, 2020))
    print(get_solution(ns, 30000000))
