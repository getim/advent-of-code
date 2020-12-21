#!/usr/bin/env python3

import math
import copy

# For each cardinal direction:
#   (neighbor_col_offset, neighbor_row_offset), (side_id, neighbor_side_id)
NEIGHBOR_MAP = [
    [(0, 1), (1, 3)],
    [(1, 0), (2, 0)],
    [(0, -1), (3, 1)],
    [(-1, 0), (0, 2)],
]

MONSTER = [
    '                  # ',
    '#    ##    ##    ###',
    ' #  #  #  #  #  #   ',
]


def read_tiles(lines):
    result = {}
    id = None
    tile = []
    for line in lines:
        line = line.strip()
        if not line:
            result[id] = tile[:]
            tile = []
        elif line.startswith('Tile '):
            id = int(line.split(' ')[1][:-1])
        else:
            tile.append([str(int(c == '#')) for c in line])
    if tile:
        result[id] = tile
    return result


def rotate(l):
    return [[l[i][len(l) - 1 - j] for i in range(len(l))]
            for j in range(len(l))]


def flip_h(l):
    return l[::-1]


def flip_v(l):
    return [row[::-1] for row in l]


def identity(l):
    return l


def get_list_hash(l):
    return int(''.join(l), 2)


def get_side_hashes(tile):
    return [
        get_list_hash(tile[0]),
        get_list_hash([row[-1] for row in tile]),
        get_list_hash(tile[-1]),
        get_list_hash([row[0] for row in tile])
    ]


def get_transformed(tile, i):
    bitmap = bin(i)[2:].rjust(3, '0')
    result = copy.deepcopy(tile)
    if int(bitmap[0]):
        result = rotate(result)
    if int(bitmap[1]):
        result = flip_h(result)
    if int(bitmap[2]):
        result = flip_v(result)
    return result


def get_side_hashes_options(tile):
    result = []
    for r in [identity, rotate]:
        for h in [identity, flip_h]:
            for v in [identity, flip_v]:
                result.append(get_side_hashes(v(h(r(tile)))))
    return result


def tile_fits(tiling, row, col, tile_id, transform, all_tiles):
    side_ids = all_tiles[tile_id][transform]
    for neigh_offset, side_hash_indexes in NEIGHBOR_MAP:
        row_neigh = row + neigh_offset[0]
        col_neigh = col + neigh_offset[1]
        if not 0 <= row_neigh < len(tiling) \
                or not 0 <= col_neigh < len(tiling):
            continue
        neigh_tile = tiling[row_neigh][col_neigh]
        if neigh_tile is None:
            continue
        side_ids_neigh = all_tiles[neigh_tile[0]][neigh_tile[1]]
        side_hash = side_ids[side_hash_indexes[0]]
        side_hash_neigh = side_ids_neigh[side_hash_indexes[1]]
        if side_hash == side_hash_neigh:
            continue
        break
    else:
        return True
    return False


def get_tiling(tiling, tiling_pos, tile_ids, all_tiles):
    if len(tile_ids) == 0:
        return tiling

    row = tiling_pos // len(tiling)
    col = tiling_pos % len(tiling)

    for i in range(len(tile_ids)):
        tile_id = tile_ids.pop(i)
        for transform in range(8):
            if tile_fits(tiling, row, col, tile_id, transform, all_tiles):
                tiling[row][col] = tile_id, transform
                new_tiling = get_tiling(tiling, tiling_pos + 1, tile_ids, all_tiles)
                if new_tiling:
                    return new_tiling
                tiling[row][col] = None
        tile_ids.insert(i, tile_id)
    else:
        return False
    assert False


def corner_product(t):
    return t[0][0][0] * t[0][-1][0] * t[-1][0][0] * t[-1][-1][0]


def glue_image(tiling, tiles):
    tile_size = len(list(tiles.values())[0]) - 2
    image_size = int(math.sqrt(len(tiles))) * tile_size
    result = [[] for _ in range(image_size)]
    for overall_r, row in enumerate(tiling):
        for tile_id, transform in row:
            tile = get_transformed(tiles[tile_id], transform)
            for r, tile_row in enumerate(tile[1:-1]):
                result[overall_r * tile_size + r].extend(tile_row[1:-1])
    return result


def get_product_sum(image, pattern, x, y):
    result = 0
    for i in range(len(pattern)):
        for j in range(len(pattern[i])):
            result += pattern[i][j] * int(image[x + i][y + j])
    return result


def count_pattern_occs(image, pattern, pattern_parts):
    result = 0
    for i in range(len(image) - len(pattern)):
        for j in range(len(image) - len(pattern[0])):
            if get_product_sum(image, pattern, i, j) == pattern_parts:
                result += 1
    return result


def count_char_occs(image, c):
    return sum(row.count(c) for row in image)


def get_solution1(tiles):
    tiles = {id: get_side_hashes_options(tile) for id, tile in tiles.items()}
    side_len = int(math.sqrt(len(tiles)))
    tiling = [[None] * side_len for _ in range(side_len)]
    tiling = get_tiling(tiling, 0, list(tiles.keys()), tiles)
    return corner_product(tiling), tiling


def get_solution2(tiling, tiles):
    image = glue_image(tiling, tiles)
    monster = [[1 if x == '#' else 0 for x in row] for row in MONSTER]
    monster_roughs = count_char_occs(monster, 1)
    image_roughs = count_char_occs(image, '1')
    for r in [identity, rotate]:
        for h in [identity, flip_h]:
            for v in [identity, flip_v]:
                monster_count = count_pattern_occs(v(h(r(image))), monster,
                                                   monster_roughs)
                if monster_count:
                    return image_roughs - (monster_count * monster_roughs)
    assert False


if __name__ == '__main__':
    with open('input') as f:
        tiles = read_tiles(f)

    sol1, tiling = get_solution1(tiles)
    print(sol1)
    print(get_solution2(tiling, tiles))
