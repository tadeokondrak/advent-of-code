#!/usr/bin/env python3
from textwrap import wrap

input_line = open('../input', 'r').read()[:-1]

polymer = input_line

last_length = None
while True:
    i = 0
    while True:
        if polymer[i].swapcase() == polymer[i+1]:
            polymer = polymer[:i] + polymer[i+2:]
        i = i + 1
        if i >= len(polymer) - 1:
            break
    if (last_length is not None) and (len(polymer) == last_length):
        break

    last_length = len(polymer)

print(len(polymer))
