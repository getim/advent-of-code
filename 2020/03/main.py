#!/usr/bin/env python3


def get_tree_count(grid, x_slope, y_slope):
    tree_count = 0
    x, y = 0, 0
    width = len(grid[0])

    while y < len(grid):
        if grid[y][x % width] == '#':
            tree_count += 1
        x += x_slope
        y += y_slope
    return tree_count


def get_solution1(grid):
    return get_tree_count(grid, 3, 1)


def get_solution2(grid):
    result = 1
    for slope in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
        result *= get_tree_count(grid, *slope)
    return result


if __name__ == '__main__':
    with open('input') as f:
        grid = [x.strip() for x in f]
    print(get_solution1(grid))
    print(get_solution2(grid))
