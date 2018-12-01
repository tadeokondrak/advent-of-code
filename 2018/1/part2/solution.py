#!/usr/bin/env python3
# SLOOOOOW but i couldn't come up with a better way until someone told me how they did it after
with open("../input", "r") as file:
    myInput = file.read().split('\n')
    myInput = myInput[:len(myInput)-1]
    array = [0]
    frequency = 0
    i = 0
    while True:
        if(i >= len(myInput)):
            myInput = myInput + myInput
        if(myInput[i][0:1] == "+"):
            frequency = frequency + int(myInput[i][1:])
        else:
            frequency = frequency - int(myInput[i][1:])
        array.append(frequency)
        if array[-1:][0] in array[0:-1]:
            print(array[-1:][0])
            break
        i += 1
