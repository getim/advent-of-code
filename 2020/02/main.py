#!/usr/bin/env python3

import re

LINE_PATTERN = re.compile(r'(\d*)-(\d*) (\w): (\w*)')


def is_valid1(min_count, max_count, char, password):
    return int(min_count) <= password.count(char) <= int(max_count)


def is_valid2(index1, index2, char, password):
    return (password[int(index1) - 1] == char) ^ (password[int(index2) - 1] == char)


def get_solution(input, valid_func):
    return sum([valid_func(*re.match(LINE_PATTERN, l).groups())
                for l in input])


if __name__ == '__main__':
    with open('input') as f:
        input = f.readlines()

    print(get_solution(input, is_valid1))
    print(get_solution(input, is_valid2))
