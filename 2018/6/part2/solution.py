#!/usr/bin/env python3

import numpy
import sys


input_lines = open('../input', 'r').read().splitlines()


distance = lambda coords, coords_2 : abs(coords[0] - coords_2[0]) + abs(coords[1] - coords_2[1])

idcount = 1

coords_list = []
for line in input_lines:
    coords = (idcount, tuple([int(x) for x in line.split(', ')])[::-1] )
    coords_list.append(coords)
    idcount += 1

max_x = 0
max_y = 0
for coords in coords_list:
    if coords[1][0] > max_x:
        max_x = coords[1][0] + 1
    if coords[1][1] > max_y:
        max_y = coords[1][1] + 1

master_grid = numpy.zeros((max_x,max_y), dtype=int)

for i in range(0, len(master_grid)):
    for j in range(0, len(master_grid[0])):
        for coords in coords_list:
            master_grid[i, j] += distance(coords[1], (i, j))

count = 0
for i in range(0, len(master_grid)):
    for j in range(0, len(master_grid[0])):
        if 10000 > master_grid[i, j]:
            count += 1

print(count)

