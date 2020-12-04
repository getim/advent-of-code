#!/usr/bin/env python3

import re

HCL_PATTERN = re.compile(r'^#[\da-f]{6}$')
ECLS = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
PID_PATTERN = re.compile(r'^\d{9}$')


def read_passports(lines):
    result = []
    passport = {}
    for l in lines:
        l = l.strip()
        if not l:
            result.append(passport.copy())
            passport = {}
            continue
        passport.update([prop.split(':') for prop in l.split(' ')])
    if passport:
        result.append(passport)
    return result


def is_valid1(passport):
    return len(passport) == 8 or (len(passport) == 7 and 'cid' not in passport)


def is_valid_height(hgt):
    unit = hgt[-2:]
    number = int(hgt[:-2])
    if unit == 'cm':
        return 150 <= number <= 193
    elif hgt[-2:] == 'in':
        return 59 <= number <= 76
    else:
        return False


def is_valid2(passport):
    if not is_valid1(passport):
        return False

    passport = {k: int(v) if k[-2:] == 'yr' else v
                for k, v in passport.items()}

    return 1920 <= passport['byr'] <= 2002 \
        and 2010 <= passport['iyr'] <= 2020 \
        and 2020 <= passport['eyr'] <= 2030 \
        and is_valid_height(passport['hgt']) \
        and bool(re.match(HCL_PATTERN, passport['hcl'])) \
        and passport['ecl'] in ECLS \
        and bool(re.match(PID_PATTERN, passport['pid']))


def count_valid(passports, valid_func):
    return sum([valid_func(p) for p in passports])


if __name__ == '__main__':
    with open('input') as f:
        passports = read_passports(f)

    print(count_valid(passports, is_valid1))
    print(count_valid(passports, is_valid2))
