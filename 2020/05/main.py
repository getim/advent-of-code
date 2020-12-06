#!/usr/bin/env python3


def parts_to_number(part, upper_char):
    return int(''.join([str(int(c == upper_char)) for c in part]), 2)


def read_seats(lines):
    return [(parts_to_number(l.strip()[:7], 'B'),
             parts_to_number(l.strip()[7:], 'R'))
            for l in lines]


def get_seat_id(seat):
    return seat[0] * 8 + seat[1]


def get_solution2(seat_ids, max_id):
    return (set(range(min(seat_ids), max_id)) - seat_ids).pop()


if __name__ == '__main__':
    with open('input') as f:
        seats = read_seats(f)
    seat_ids = {get_seat_id(s) for s in seats}

    max_id = max(seat_ids)
    print(max_id)
    print(get_solution2(seat_ids, max_id))
