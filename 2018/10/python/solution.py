#!/usr/bin/env python3

import numpy

def graph(points, size):
    grid = numpy.zeros((size[0], size[1]), dtype=int)
    for point in points:
        if abs(point[0][0]) >= size[1]:
            break
        if abs(point[0][1]) >= size[0]:
            break
        grid[point[0][1], point[0][0]] = 1
    return grid

with open('../input', 'r') as file:
    input_lines = file.readlines()

    points = []

    for line in input_lines:
        xpos = int(line.split('<')[1].split('>')[0].split(',')[0].strip())
        ypos = int(line.split('<')[1].split('>')[0].split(',')[1].strip())
        xvel = int(line.split('<')[2].split('>')[0].split(',')[0].strip())
        yvel = int(line.split('<')[2].split('>')[0].split(',')[1].strip())
        points.append(((xpos, ypos), (xvel, yvel)))

    last_spread = 9999999
    last_grid = None
    second = 0
    while True:
        maxx = 0
        maxy = 0
        miny = 0
        minx = 0
        for index in range(len(points)):
            point = points[index]
            points[index] = (( point[0][0] + point[1][0],  point[0][1] + point[1][1]), point[1])
            if points[index][0][0] > maxx:
                maxx = points[index][0][0]
            if points[index][0][1] > maxy:
                maxy = points[index][0][1]
            if points[index][0][0] < minx or minx == 0:
                minx = points[index][0][0]
            if points[index][0][1] < miny or miny == 0:
                miny = points[index][0][1]
        spread = (maxx - minx) + (maxy - miny)

        if last_spread < spread:
            for line in last_grid:
                print(numpy.array2string(line, max_line_width=99999, separator='').replace('0', '.'))

            print(second)
            break

        last_spread = spread

        second += 1
        last_grid = graph(points, (minx+maxx, miny + maxy))
