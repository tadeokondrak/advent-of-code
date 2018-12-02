#!/usr/bin/env python3
import string

with open("../input", "r") as file:
    myInput = file.read().split('\n')
    myInput = myInput[:-1] # remove last line, an empty one
    letters = list(string.ascii_lowercase) # ['a', 'b', 'c'...]
    boxIdsWithTwo = 0
    boxIdsWithThree = 0
    for i in myInput:
        hasTwo = False
        hasThree = False
        letterCounts = {}
        for k in letters:
            letterCounts[k] = 0
        for j in range(0, len(i)):
            letterCounts[i[j]] += 1
        for l in letterCounts:
            if(letterCounts[l] == 2):
                hasTwo = True
            elif(letterCounts[l] == 3):
                hasThree = True
        if(hasTwo == True):
            boxIdsWithTwo += 1
        if(hasThree == True):
            boxIdsWithThree += 1
    print(boxIdsWithTwo * boxIdsWithThree)
