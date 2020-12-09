#!/usr/bin/env python3


def can_sum_to(n, numbers):
    for i in range(len(numbers) - 1):
        for j in range(i + 1, len(numbers)):
            if numbers[i] + numbers[j] == n:
                return True
    return False


def get_solution1(msg, preamble_length):
    for i in range(preamble_length, len(msg)):
        if not can_sum_to(msg[i], msg[i - preamble_length:i]):
            return msg[i]


def get_solution2(msg, sum_to_find):
    for i in range(len(msg)):
        s = msg[i]
        smallest = msg[i]
        largest = msg[i]
        for j in range(i + 1, len(msg)):
            s += msg[j]
            if msg[j] > largest:
                largest = msg[j]
            elif msg[j] < smallest:
                smallest = msg[j]
            if s == sum_to_find:
                return largest + smallest
            elif s > sum_to_find:
                break


if __name__ == '__main__':
    with open('input') as f:
        msg = [int(l) for l in f]

    first_bad = get_solution1(msg, 25)
    print(first_bad)
    print(get_solution2(msg, first_bad))
