#!/usr/bin/env python3
import numpy

input_lines = open('../input', 'r').read().splitlines()

grid = numpy.zeros((1000,1000))

def parse_line(line: str) -> tuple:
    split = line.split(' ')
    claimid = int(split[0][1:])
    coords = tuple(int(i) for i in split[2][:-1].split(','))
    size = tuple(int(i) for i in split[3].split('x'))

    x1 = coords[0]
    y1 = coords[1]
    x2 = coords[0] + size[0]
    y2 = coords[1] + size[1]

    return ((claimid, ((x1, y1), (x2, y2))))

for i in input_lines:
    parsed = parse_line(i)

    for j in range(parsed[1][0][0], parsed[1][1][0]):
        for k in range(parsed[1][0][1], parsed[1][1][1]):
            grid[j, k] += 1

for i in input_lines:
    uncontested = True
    parsed = parse_line(i)

    for j in range(parsed[1][0][0], parsed[1][1][0]):
        for k in range(parsed[1][0][1], parsed[1][1][1]):
            if grid[j, k] > 1:
               uncontested = False

    if uncontested is True:
        print(parsed[0])

