#!/usr/bin/env python3


import re


LINE_PATTERN = re.compile(r'(.*) \(contains (.*)\)')


def read_foods(lines):
    result = []
    for line in lines:
        ingredients, allergens = re.match(LINE_PATTERN, line).groups()
        result.append((ingredients.split(' '), allergens.split(', ')))
    return result


def get_solution1(foods):
    # Map allergens to ingredients that could contain them
    allergen_map = {}
    all_ingredients = []
    for ingredients, allergens in foods:
        all_ingredients.extend(ingredients)
        for allergen in allergens:
            if allergen in allergen_map:
                allergen_map[allergen] = allergen_map[allergen].intersection(ingredients)
            else:
                allergen_map[allergen] = set(ingredients)

    # Map ingredients to the allergen they contain
    ingredient_map = {}
    while allergen_map:
        for allergen, ingredients in allergen_map.items():
            if len(ingredients) == 1:
                break
        ingredient_found = ingredients.pop()
        ingredient_map[ingredient_found] = allergen
        del allergen_map[allergen]
        for _, ingredients in allergen_map.items():
            if ingredient_found in ingredients:
                ingredients.remove(ingredient_found)

    result = 0
    for ingredient in all_ingredients:
        if ingredient not in ingredient_map:
            result += 1
    return result, ingredient_map


def get_solution2(ingredient_map):
    return ','.join(sorted([k for k in ingredient_map],
                           key=lambda k: ingredient_map[k]))


if __name__ == '__main__':
    with open('input') as f:
        foods = read_foods(f)

    sol, ingredient_map = get_solution1(foods)
    print(sol)
    print(get_solution2(ingredient_map))
