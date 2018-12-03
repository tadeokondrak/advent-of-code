#!/usr/bin/env python3

input_lines = open('../input', 'r').read().splitlines()

frequency = 0

for i in input_lines:
    if i[:1] == "+":
        frequency = frequency + int(i[1:])
    else:
        frequency = frequency - int(i[1:])

print(frequency)
