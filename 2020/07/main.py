#!/usr/bin/env python3


import re
from collections import Counter


CONTENT_PATTERN = re.compile(r'(?:(\d*) (\w* \w*) bags?. ?)')


def read_rules(lines):
    result = {}
    for l in lines:
        base, content = l.strip().split(' bags contain ')
        result[base] = {type: int(count) for (count, type) in
                        re.findall(CONTENT_PATTERN, content)}
    return result


def get_possible_wrappers(rules, base_bag, current_bags=set()):
    new_current_bags = set()
    for bag in current_bags or [base_bag]:
        for base, content in rules.items():
            if bag in content and base not in current_bags:
                new_current_bags.add(base)

    if new_current_bags:
        new_current_bags.update(get_possible_wrappers(rules, base_bag, new_current_bags))

    return current_bags.union(new_current_bags)


def get_solution1(rules, base_bag):
    return len(get_possible_wrappers(rules, base_bag))


def get_bags_inside(rules, base_type):
    new_bags = Counter()
    if base_type in rules and rules[base_type]:
        new_bags.update(rules[base_type])
        for bag_type, count in rules[base_type].items():
            new_bags.update({k: count * v
                             for k, v in get_bags_inside(rules, bag_type).items()})
    return new_bags


def get_solution2(rules, base_bag):
    return sum(get_bags_inside(rules, base_bag).values())


if __name__ == '__main__':
    with open('input') as f:
        rules = read_rules(f)
    print(get_solution1(rules, 'shiny gold'))
    print(get_solution2(rules, 'shiny gold'))
