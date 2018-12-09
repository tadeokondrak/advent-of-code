#!/usr/bin/env python3

from itertools import cycle

with open('../input', 'r') as file:
    frequency = 0
    seen = set()
    found = False
    for num in cycle(file.readlines()):
        frequency += int(num)
        if frequency in seen:
            print(frequency)
            break
        seen.add(frequency)

