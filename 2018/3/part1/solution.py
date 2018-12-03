#!/usr/bin/env python3
import numpy

input_lines = open('../input', 'r').read().splitlines()

grid = numpy.zeros((1000,1000))

def parse_line(line: str) -> dict:
    split = line.split(' ')
    claimid = int(split[0][1:])
    coords = tuple(int(i) for i in split[2][:-1].split(','))
    size = tuple(int(i) for i in split[3].split('x'))

    return {'claimid': claimid,
            'x1': coords[0],
            'x2': coords[0] + size[0],
            'y1': coords[1],
            'y2': coords[1] + size[1]}

for i in input_lines:
    parsed = parse_line(i)

    for x in range(parsed['x1'], parsed['x2']):
        for y in range(parsed['y1'], parsed['y2']):
            grid[x, y] += 1

multiClaimed = 0

for x in range(0, 1000):
    for y in range(0, 1000):
        if grid[x, y] > 1:
            multiClaimed += 1

print(multiClaimed)
