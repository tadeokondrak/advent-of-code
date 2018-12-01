#!/usr/bin/env python3

with open("../input", "r") as file:
    myInput = file.read().split('\n')
    myInput = myInput[:len(myInput)-1]
    frequency = 0
    for i in range(0, len(myInput)):
        if(myInput[i][0:1] == "+"):
            frequency = frequency + int(myInput[i][1:])
        else:
            frequency = frequency - int(myInput[i][1:])
    print(frequency)
