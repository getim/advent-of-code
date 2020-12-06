#!/usr/bin/env python3


def read_groups(lines):
    result = []
    group = set()
    for l in lines:
        l = l.strip()
        if not l:
            result.append(group.copy())
            group = set()
            continue
        group.add(l)
    if group:
        result.append(group)
    return result


def get_answer_count(groups, combine_func):
    return sum([len(combine_func(*[set(p) for p in people]))
                for people in groups])


def get_solution1(groups):
    return get_answer_count(groups, set.union)


def get_solution2(groups):
    return get_answer_count(groups, set.intersection)


if __name__ == '__main__':
    with open('input') as f:
        groups = read_groups(f)

    print(get_solution1(groups))
    print(get_solution2(groups))
