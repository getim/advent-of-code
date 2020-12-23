#!/usr/bin/env python3


# Part 1 using generator to create cycle
def cycle(cups):
    while True:
        for cup in cups:
            yield cup


def do_move_generator(cups, cup_count):
    current = next(cups)
    cups_to_move = [next(cups) for _ in range(3)]
    after_insert = (current - 2) % cup_count + 1
    while after_insert in cups_to_move:
        after_insert = (after_insert - 2) % cup_count + 1

    new_cups = []
    for _ in range(cup_count - 4):
        cup = next(cups)
        new_cups.append(cup)
        if cup == after_insert:
            new_cups.extend(cups_to_move)
    new_cups.append(current)

    return cycle(new_cups)


def get_solution1(cups):
    cup_count = len(cups)
    cups = cycle(cups)
    for _ in range(100):
        cups = do_move_generator(cups, cup_count)

    while next(cups) != 1:
        pass
    return ''.join(str(next(cups)) for _ in range(cup_count - 1))


# Part 2 with linked list, faster
def get_cup(cups, index):
    if index < len(cups):
        return cups[index]
    else:
        return index + 1


def get_index(cups, cup):
    if cup in cups:
        return cups.index(cup)
    else:
        return cup - 1


def do_move(cups, links, current, cup_count):
    start_group_index = links[current]
    end_group_index = start_group_index
    cups_to_move = [get_cup(cups, start_group_index)]
    for _ in range(2):
        end_group_index = links[end_group_index]
        cups_to_move.append(get_cup(cups, end_group_index))
    links[current] = links[end_group_index]

    insert_cup = (get_cup(cups, current) - 2) % cup_count + 1
    while insert_cup in cups_to_move:
        insert_cup = (insert_cup - 2) % cup_count + 1
    insert_index = get_index(cups, insert_cup)
    links[end_group_index] = links[insert_index]
    links[insert_index] = start_group_index
    return cups, links, links[current]


def get_solution2(cups, cup_count):
    links = [(i + 1) % cup_count for i in range(cup_count)]
    current = 0
    for _ in range(10000000):
        cups, indexes, current = do_move(cups, links, current, cup_count)

    base = get_index(cups, 1)
    return get_cup(cups, links[base]) * get_cup(cups, links[links[base]])


if __name__ == '__main__':
    cups = [int(x) for x in '156794823']

    print(get_solution1(cups))
    print(get_solution2(cups, 1000000))
