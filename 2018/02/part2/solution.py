#!/usr/bin/env python3
with open("../input", "r") as file:
    myInput = file.read().split('\n')
    myInput = myInput[:-1] # remove last line, an empty one
    for i in range(0, len(myInput)):
        for j in myInput[:i]:
            differences = 0
            string1 = myInput[i]
            string2 = j
            for k in range(len(string1)):
                if(string1[k] != string2[k]):
                    differences += 1
            if(differences == 1):
                finalString = ""
                for k in range(len(string1)):
                    if(string1[k] == string2[k]):
                        finalString += string1[k]
                print(finalString)
