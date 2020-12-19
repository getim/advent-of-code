#!/usr/bin/env python3

import re
import sys


def read_input(lines):
    i = 0
    rules = {}
    while lines[i].strip():
        id, rule = lines[i].strip().split(': ')
        rules[int(id)] = rule.split(' ')
        i += 1

    i += 1
    msgs = []
    while i < len(lines):
        msgs.append(lines[i].strip())
        i += 1

    return rules, msgs


def unrecurse_trailing(groups, rule_id):
    """
    Properly create +-regex for trailing recursion
    """
    recs, non_recs = [], []
    for group in groups:
        if group[-1] == str(rule_id):
            recs.append(group[:-1])
        else:
            non_recs.append(group[:])
    if recs == non_recs:
        return [group + ['+'] for group in non_recs]
    else:
        return groups


def duplicate_internal(groups, rule_id):
    """
    Approximate internal recursion by creating options up to a certain depth
    """
    if len(groups) != 2:
        return groups

    groups = sorted(groups, key=len)
    if groups[0] != [x for x in groups[1] if x != str(rule_id)]:
        return groups

    result = []
    insert_index = groups[1].index(str(rule_id))
    for i in range(8):
        new_group = groups[0][:]
        for j in range(i):
            for x in groups[0][::-1]:
                new_group.insert(insert_index + j, x)
        result.append(new_group)
    return result


def unrecurse(rule, rule_id):
    group, groups = [], []
    for i in range(len(rule) + 1):
        if i == len(rule) or rule[i] == '|':
            groups.append(group[:])
            group = []
        else:
            group.append(rule[i])

    groups = unrecurse_trailing(groups, rule_id)
    groups = duplicate_internal(groups, rule_id)

    result = []
    for group in groups:
        if rule_id in group:
            print("Failed to naively remove all recursiveness")
            sys.exit(1)
        result.extend(group + ['|'])
    return result[:-1]


def get_pattern(id, rules):
    pattern = ''
    non_rec_rule = unrecurse(rules[id], id)
    for c in unrecurse(non_rec_rule, id):
        if c.isdigit():
            group = get_pattern(int(c), rules)
            pattern += '(' + group + ')'
        elif c in ['|', '+']:
            pattern += c
        else:
            pattern += c[1:-1]
    return pattern


def get_solution(rules, msgs):
    pattern = get_pattern(0, rules)
    return [bool(re.match(pattern + '$', msg)) for msg in msgs].count(True)


if __name__ == '__main__':
    with open('input') as f:
        rules, msgs = read_input(f.readlines())

    print(get_solution(rules, msgs))
    rules[8] = ['42', '|', '42', '8']
    rules[11] = ['42', '31', '|', '42', '11', '31']
    print(get_solution(rules, msgs))
