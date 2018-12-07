#!/usr/bin/env python3

input_lines = open('../input', 'r').read().splitlines()

get_first_alphabetically = lambda steps: sorted(steps)[0]

def get_satisfied_steps(steps, steps_done):
    satisfied_steps = []
    for step in steps.items():
        satisfied = True
        for dependency in step[1]:
            if dependency in steps:
                satisfied = False
        if satisfied is True:
            satisfied_steps.append(step)
    return satisfied_steps

steps = {}
for line in input_lines:
    dependency = line.split(' ')[1]
    step = line.split(' ')[7]
    steps.setdefault(step, []).append(dependency)
    steps.setdefault(dependency, [])

steps_done = []
while True:
    satisfied_steps = get_satisfied_steps(steps, steps_done)
    if len(satisfied_steps) > 0:
        step = get_first_alphabetically(satisfied_steps)
        print(step[0], end='', flush=True)
        del steps[step[0]]
    else:
        break
print()
