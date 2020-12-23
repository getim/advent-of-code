#!/usr/bin/env python3

import copy


def read_decks(lines):
    player_id = 0
    decks = ([], [])
    for line in lines:
        line = line.strip()
        if not line:
            player_id += 1
        elif line.isdigit():
            decks[player_id].append(int(line))
    return decks


def score(deck):
    return sum((len(deck) - i) * card for i, card in enumerate(deck))


def assign_cards(cards, decks):
    sorted_cards = sorted(enumerate(cards), key=lambda x: x[1],
                          reverse=True)
    decks[sorted_cards[0][0]].extend([x[1] for x in sorted_cards])
    return sorted_cards[0][0]


def play1(decks):
    while all(decks):
        cards = [deck.pop(0) for deck in decks]
        winner = assign_cards(cards, decks)
    return winner


def play2(decks):
    visited_hands = [[] for _ in decks]
    while all(decks):
        for i in range(len(decks)):
            if decks[i] in visited_hands[i]:
                return 0
            visited_hands[i].append(decks[i][:])
        cards = [deck.pop(0) for deck in decks]
        if all(len(deck) >= cards[i] for i, deck in enumerate(decks)):
            winner = play2([deck[:cards[i]] for i, deck in enumerate(decks)])
            decks[winner].append(cards.pop(winner))
            decks[winner].extend(cards)
        else:
            winner = assign_cards(cards, decks)
    return winner


def get_solution(decks, play_func):
    decks = copy.deepcopy(decks)
    winner = play_func(decks)
    return score(decks[winner])


if __name__ == '__main__':
    with open('input') as f:
        decks = read_decks(f)

    print(get_solution(decks, play1))
    print(get_solution(decks, play2))
