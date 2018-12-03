#!/usr/bin/env python3
import numpy

input_lines = open('../input', 'r').read().splitlines()

grid = numpy.zeros((1000,1000))

for i in input_lines:
    split = i.split(' ')
    claimid = int(split[0][1:])
    coords = tuple(int(i) for i in split[2][:-1].split(','))
    size = tuple(int(i) for i in split[3].split('x'))

    x1 = coords[0]
    y1 = coords[1]
    x2 = coords[0] + size[0]
    y2 = coords[1] + size[1]

    for j in range(x1, x2):
        for k in range(y1, y2):
            grid[j, k] += 1

multiClaimed = 0

for i in range(0, 1000):
    for j in range(0, 1000):
        if grid[i, j] > 1:
            multiClaimed += 1

print(multiClaimed)
