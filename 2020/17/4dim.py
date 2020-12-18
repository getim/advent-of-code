#!/usr/bin/env python3


def draw(grid):
    print('-' * 20)
    for w in range(len(grid)):
        for z in range(len(grid[w])):
            print('z={}, w={}'.format(z, w))
            for row in grid[w][z]:
                print(''.join(['#' if x else '.' for x in row]))
            print()


def is_valid_coord(coord, grid):
    return 0 <= coord[3] < len(grid) \
        and 0 <= coord[2] < len(grid[coord[3]]) \
        and 0 <= coord[1] < len(grid[coord[3]][coord[2]]) \
        and 0 <= coord[0] < len(grid[coord[3]][coord[2]][coord[1]])


def count_neighbors(coord, grid):
    count = 0
    for x in range(-1, 2):
        for y in range(-1, 2):
            for z in range(-1, 2):
                for w in range(-1, 2):
                    if x == 0 and y == 0 and z == 0 and w == 0:
                        continue
                    new_x = coord[0] + x
                    new_y = coord[1] + y
                    new_z = coord[2] + z
                    new_w = coord[3] + w
                    if not is_valid_coord((new_x, new_y, new_z, new_w), grid):
                        continue
                    if grid[new_w][new_z][new_y][new_x]:
                        count += 1
    return count


def trim(grid):
    for direction in [0, -1]:
        trimmed = True
        while trimmed:
            trimmed = False
            if any([any([any(y) for y in z]) for z in grid[direction]]):
                continue
            del grid[direction]

    for direction in [0, -1]:
        trimmed = True
        while trimmed:
            trimmed = False
            if any([any([any(y) for y in w[direction]]) for w in grid]):
                continue
            for w in grid:
                del w[direction]
            trimmed = True

    for direction in [0, -1]:
        trimmed = True
        while trimmed:
            trimmed = False
            if any([any([any(z[direction]) for z in w]) for w in grid]):
                continue
            for w in grid:
                for z in w:
                    del z[direction]
            trimmed = True

    for direction in [0, -1]:
        trimmed = True
        while trimmed:
            trimmed = False
            if any([any([any([y[direction] for y in z]) for z in w]) for w in grid]):
                continue
            for w in grid:
                for z in w:
                    for y in z:
                        del y[direction]
            trimmed = True


def step(grid):
    new_grid = []
    for w in range(-1, len(grid) + 1):
        new_grid.append([])
        for z in range(-1, len(grid[0]) + 1):
            new_grid[-1].append([])
            for y in range(-1, len(grid[0][0]) + 1):
                new_grid[-1][-1].append([])
                for x in range(-1, len(grid[0][0][0]) + 1):
                    valid = is_valid_coord((x, y, z, w), grid)
                    count = count_neighbors((x, y, z, w), grid)
                    if valid and grid[w][z][y][x]:
                        new_cell = (count == 2 or count == 3)
                    else:
                        new_cell = count == 3
                    new_grid[-1][-1][-1].append(new_cell)
    trim(new_grid)
    return new_grid


def get_solution(grid):
    for i in range(6):
        grid = step(grid)

    return sum([sum([sum([y.count(True) for y in z]) for z in w]) for w in grid])


if __name__ == '__main__':
    with open('input') as f:
        grid = [[[[x == '#' for x in list(l.strip())] for l in f]]]

    print(get_solution(grid))
