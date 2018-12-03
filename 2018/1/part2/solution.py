#!/usr/bin/env python3

input_lines = open('../input', 'r').read().splitlines()

frequency = 0
seen = set()

found = False

while found is False:
    for i in input_lines:
        if i[:1] == "+":
            frequency = frequency + int(i[1:])
        else:
            frequency = frequency - int(i[1:])
        if frequency in seen:
            found = True
            print(frequency)
            break
        seen.add(frequency)

