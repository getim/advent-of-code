#!/usr/bin/env python3


DIFF_LIST_START = [0, 1, 1]


def get_solution1(jolts):
    diffs = [jolts[i + 1] - jolts[i] for i in range(len(jolts) - 1)]
    return diffs.count(1) * diffs.count(3)


def get_solution2(jolts):
    result = 1
    factor = 1
    diff_list = DIFF_LIST_START[:]
    for i in range(1, len(jolts) - 2):
        if i < len(jolts) - 1 and jolts[i + 2] <= jolts[i - 1] + 3:
            if len(diff_list) > 3:
                factor = factor * 2 - diff_list[-3]
            else:
                factor *= 4
            diff_list.append(sum(diff_list[-3:]))
        elif len(diff_list) == 3 and jolts[i + 1] <= jolts[i - 1] + 3:
            result *= 2
            diff_list = DIFF_LIST_START[:]
        else:
            result *= factor
            factor = 1
            diff_list = DIFF_LIST_START[:]
    return result


if __name__ == '__main__':
    with open('input') as f:
        jolts = sorted([int(l) for l in f])
    jolts = [0] + jolts + [jolts[-1] + 3]

    print(get_solution1(jolts))
    print(get_solution2(jolts))
