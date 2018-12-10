#!/usr/bin/env python3

input_lines = open('../input', 'r').readlines()

WORKER_COUNT = 5
BASE_STEP_TIME = 60

get_first_alphabetically = lambda steps: sorted(steps)[0]
get_number_for_letter = lambda letter: ord(letter) - 64

def get_satisfied_steps(steps, steps_in_progress):
    satisfied_steps = []
    for step in steps.items():
        satisfied = True
        for dependency in step[1]:
            if dependency in steps:
                satisfied = False
        if satisfied is True and step[0] not in steps_in_progress:
            satisfied_steps.append(step)
    return satisfied_steps

steps = {}
for line in input_lines:
    dependency = line.split(' ')[1]
    step = line.split(' ')[7]
    steps.setdefault(step, []).append(dependency)
    steps.setdefault(dependency, [])

workers = [ (0, None) for i in range(WORKER_COUNT) ]

seconds_elapsed = 0
steps_in_progress = []

while True:
    for index in range(len(workers)):
        if workers[index][0] > 0:
            workers[index] = (workers[index][0] - 1, workers[index][1])
        if workers[index][0] == 0 and workers[index][1] != None:
            steps_in_progress.remove(workers[index][1])
            del steps[workers[index][1]]
            workers[index] = (0, None)

    for index in range(len(workers)):
        satisfied_steps = get_satisfied_steps(steps, steps_in_progress)
        if len(satisfied_steps) > 0 and workers[index][0] == 0:
            step = get_first_alphabetically(satisfied_steps)
            steps_in_progress.append(step[0])
            workers[index] = (BASE_STEP_TIME + get_number_for_letter(step[0]), step[0])

    if len(steps) == 0:
        break
    seconds_elapsed += 1

print(seconds_elapsed)
