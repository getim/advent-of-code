#!/usr/bin/env python3


def read_info(lines):
    i = 0
    rules = {}
    while lines[i].strip():
        prop, prop_rules = lines[i].strip().split(': ')
        rules[prop] = [[int(x) for x in rule.split('-')]
                       for rule in prop_rules.split(' or ')]
        i += 1

    i += 2
    my_ticket = [int(x) for x in lines[i].strip().split(',')]
    tickets = []
    for l in lines[i + 3:]:
        tickets.append([int(x) for x in l.strip().split(',')])
    return rules, my_ticket, tickets


def satisfies_rules(n, rules):
    for rule in rules:
        if rule[0] <= n <= rule[1]:
            return True
    return False


def get_invalids_sum(ticket, all_rules):
    return sum([n for n in ticket if not satisfies_rules(n, all_rules)])


def get_solution1(rules, tickets):
    all_rules = [rule for prop_rules in rules.values() for rule in prop_rules]
    return sum([get_invalids_sum(ticket, all_rules) for ticket in tickets])


def get_solution2(rules, my_ticket, tickets):
    # Filter out completely invalid tickets
    all_rules = [rule for prop_rules in rules.values() for rule in prop_rules]
    valid_tickets = [ticket for ticket in tickets
                     if get_invalids_sum(ticket, all_rules) == 0]

    # Get all possible valid properties per index on all tickets
    all_valid_props = []
    for id in range(len(my_ticket)):
        valid_props = []
        for prop, prop_rules in rules.items():
            for ticket in valid_tickets:
                if not satisfies_rules(ticket[id], prop_rules):
                    break
            else:
                valid_props.append(prop)
        all_valid_props.append(valid_props)

    # Find indexes with one valid property to eliminate one-by-one
    final_valids = [None for _ in range(len(my_ticket))]
    while any(all_valid_props):
        valid_counts = [len(valids) for valids in all_valid_props]
        unique_index = valid_counts.index(1)
        unique_prop = all_valid_props[unique_index][0]
        final_valids[unique_index] = unique_prop
        for i, valid_props in enumerate(all_valid_props):
            if unique_prop in valid_props:
                valid_props.remove(unique_prop)
            if i == unique_index:
                all_valid_props[i] = []

    # Calculate answer
    result = 1
    for i, prop in enumerate(final_valids):
        if prop.startswith('departure'):
            result *= my_ticket[i]
    return result


if __name__ == '__main__':
    with open('input') as f:
        rules, my_ticket, tickets = read_info(f.readlines())

    print(get_solution1(rules, tickets))
    print(get_solution2(rules, my_ticket, tickets))
