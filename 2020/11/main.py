#!/usr/bin/env python3


def is_direct_occupied(seats, row_index, seat_index, row_offset, seat_offset):
    new_row = row_index + row_offset
    new_seat = seat_index + seat_offset
    if not 0 <= new_row < len(seats) or not 0 <= new_seat < len(seats[0]):
        return False
    return seats[new_row][new_seat] == '#'


def is_visible_occupied(seats, row_index, seat_index, row_offset, seat_offset):
    new_row = row_index + row_offset
    new_seat = seat_index + seat_offset
    while 0 <= new_row < len(seats) and 0 <= new_seat < len(seats[0]):
        if not seats[new_row][new_seat] == '.':
            break
        new_row = new_row + row_offset
        new_seat = new_seat + seat_offset
    else:
        return False
    return seats[new_row][new_seat] == '#'


def count_neighbours(seats, occupied_func, row_index, seat_index):
    result = 0
    for row_offset in range(-1, 2):
        for seat_offset in range(-1, 2):
            if (row_offset == 0) and (seat_offset == 0):
                continue
            if occupied_func(seats, row_index, seat_index, row_offset, seat_offset):
                result += 1
    return result


def do_round(seats, max_neighbours, occupied_func):
    result = []
    changed = False
    for i, row in enumerate(seats):
        new_row = ''
        for j, seat in enumerate(row):
            if seat == 'L' and count_neighbours(seats, occupied_func, i, j) == 0:
                new_row += '#'
                changed = True
            elif seat == '#' and count_neighbours(seats, occupied_func, i, j) >= max_neighbours:
                new_row += 'L'
                changed = True
            else:
                new_row += seat
        result.append(new_row)
    return result, changed


def find_equilibrium(seats, occupied_func, max_neighbours):
    changed = True
    while changed:
        seats, changed = do_round(seats, max_neighbours, occupied_func)
    return sum([row.count('#') for row in seats])


def get_solution1(seats):
    return find_equilibrium(seats, is_direct_occupied, 4)


def get_solution2(seats):
    return find_equilibrium(seats, is_visible_occupied, 5)


if __name__ == '__main__':
    with open('input') as f:
        seats = [l.strip() for l in f]

    print(get_solution1(seats))
    print(get_solution2(seats))
