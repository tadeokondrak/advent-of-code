#!/usr/bin/env python3

import numpy
import sys


input_lines = open('../input', 'r').read().splitlines()


distance = lambda coords, coords_2: abs(coords[0] - coords_2[0]) + abs(coords[1] - coords_2[1])

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
        max_x = coords[1][0]
    if coords[1][1] > max_y:
        max_y = coords[1][1]


master_grid = numpy.zeros((max_x,max_y), dtype=int)

for i in range(0, len(master_grid)):
    for j in range(0, len(master_grid[0])):
        lowest = (99999, None)
        distances = set()
        for coords in coords_list:
            distance_current = distance(coords[1], (i, j))
            distances.add( distance_current )
            if lowest[0] > distance_current :
                lowest = (distance_current, coords)
        mindistance = lowest[0]
        matchesofmin = 0
        for coords in coords_list:
            distance_current = distance(coords[1], (i, j))
            if distance_current == mindistance:
                matchesofmin += 1
        master_grid[i, j] = lowest[1][0]
        if matchesofmin > 1:
            master_grid[i, j] = 0

infinites = set()
for i in range(0, len(master_grid)):
    for j in range(0, len(master_grid[0])):
        if i == 0:
            infinites.add(master_grid[i,j])
        elif i == len(master_grid)-1:
            infinites.add(master_grid[i,j])
        elif j == 0:
            infinites.add(master_grid[i,j])
        elif j == len(master_grid[0])-1:
            infinites.add(master_grid[i,j])

notinfinites = set()
maximumarea = 0
for i in range(1, idcount):
    if i not in infinites:
        count = 0
        notinfinites.add(i)
        for j in range(0, max_x-1):
            for k in range(0, max_y-1):
                if master_grid[j, k] == i:
                    count += 1
        if count > maximumarea:
            maximumarea = count

print(maximumarea)
