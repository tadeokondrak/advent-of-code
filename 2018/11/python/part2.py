#!/usr/bin/env python3

import numpy

def get_level(x, y, serial):
    rack_id = x + 10
    level = rack_id * y
    level += serial
    level *= rack_id
    level = int(str(level)[-3]) - 5
    return level


with open('../input', 'r') as file:
    serial = int(file.read())
    grid = numpy.zeros((300, 300), dtype=int)

    for x in range(300):
      for y in range(300):
          grid[x, y] = get_level(x, y, serial)

    power_grid = numpy.zeros((300, 300, 300), dtype=int)
    for i in range(300):
        for x in range(300-i+1):
          for y in range(300-i+1):
              for x2 in range(x,x+i):
                  for y2 in range(y,y+i):
                      power_grid[x, y, i] += grid[x2, y2]
        print(i)
        print(numpy.unravel_index(power_grid.argmax(), power_grid.shape))


