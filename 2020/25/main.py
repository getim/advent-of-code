#!/usr/bin/env python3


MULTIPLIER = 7
MODULO = 20201227


def get_solution1(card_public, door_public):
    i = 0
    current_key = 1
    while True:
        if current_key == card_public:
            loop_size = i
            public_key = door_public
            break
        if current_key == door_public:
            loop_size = i
            public_key = card_public
            break
        current_key = current_key * MULTIPLIER % MODULO
        i += 1

    return public_key ** loop_size % MODULO


if __name__ == '__main__':
    with open('input') as f:
        card_public, door_public = [int(l.strip()) for l in f]

    print(get_solution1(card_public, door_public))
