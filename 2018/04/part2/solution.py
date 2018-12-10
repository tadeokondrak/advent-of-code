#!/usr/bin/env python3

from functools import reduce
input_lines = sorted(open('../input', 'r').read().splitlines())

guard_set = set()
guards = {}

current_guard = None
asleep = False
asleep_since = False
for line in input_lines:
    minute: int = int(line.split(']')[0][1:][14:])
    action: str = line.split(']')[1][1:]

    if action.startswith('Guard'):
        guard: int = int(action.split(' ')[1][1:])
        asleep = False
        asleep_since = None
        current_guard = guard
        guard_set.add(guard)

    for i in range(0, 60):
        if (current_guard, i) not in guards:
            guards[(current_guard, i)] = 0

    if (asleep_since is not None) and (minute > asleep_since):
        for asleep_minute in range(int(asleep_since), int(minute)+1):
            if (current_guard, asleep_minute) not in guards:
                guards[(current_guard, asleep_minute)] = 1
            else:
                guards[(current_guard, asleep_minute)] += 1

    if action.startswith('falls asleep'):
        asleep = True
        asleep_since = minute

    if action.startswith('wakes up'):
        asleep = False
        asleep_since = None

largest = (None, 0)
for minute in range(0, 60):
    for guard in guard_set:
        entry = guards[(guard, minute)]
        if entry > largest[1]:
            largest = (guard, minute)

print(reduce(lambda guard, minute: guard * minute, largest))
