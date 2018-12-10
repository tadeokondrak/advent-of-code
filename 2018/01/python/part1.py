#!/usr/bin/env python3

with open('../input', 'r') as file:
    print(sum([int(num) for num in file.readlines()]))
