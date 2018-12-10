#!/usr/bin/env python3

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

asleep_days = {}
for guard in guard_set:
    days_asleep = 0
    for minute in range(0, 60):
        days_asleep += guards[(guard, minute)]
    asleep_days[(guard)] = days_asleep

selected_guard = max(asleep_days, key=asleep_days.get)

minutes_sleep = {}
for minute in range(0, 60):
    minutes_sleep[(minute)] = guards[(selected_guard, minute)]

print(selected_guard * max(minutes_sleep, key=minutes_sleep.get))
