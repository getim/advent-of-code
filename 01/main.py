#!/usr/bin/env python3


def get_solution1(input):
    length = len(input)
    for i in range(length - 1):
        for j in range(i + 1, length):
            if input[i] + input[j] == 2020:
                return input[i] * input[j]


def get_solution2(input):
    length = len(input)
    for i in range(length - 2):
        for j in range(i + 1, length - 1):
            init_sum = input[i] + input[j]
            if init_sum > 2020:
                continue
            for k in range(j + 1, length):
                sum = init_sum + input[k]
                if sum == 2020:
                    return input[i] * input[j] * input[k]


if __name__ == '__main__':
    with open('input') as f:
        input = [int(x) for x in f]
    print(get_solution1(input))
    print(get_solution2(input))
