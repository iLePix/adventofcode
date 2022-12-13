#!/usr/bin/python
import json
from functools import cmp_to_key

same = False

def check(a, b):

    global same
    for i in range(min(len(a), len(b))):
        same = False
        el1 = a[i]
        el2 = b[i]

        if isinstance(el1, int):
            if isinstance(el2, int):
                if (el1 > el2):
                    return False
                if (el1 < el2):
                    return True
            else:
                if not check([el1], el2):
                    return False
                if not same:
                    return True
        else:
            if isinstance(el2, int):
                if not check(el1, [el2]):
                    return False
                if not same:
                    return True
            else:
                if not check(el1, el2):
                    return False
                if not same:
                    return True

    if len(a) == len(b):
        same = True

    if (len(a) > len(b)):
        return False

    return True

def cmpfunc(a, b):
    if check(a, b):
        return -1
    return 1

with open("input.txt", "r") as file:
    inputs = file.read().split('\n')

    packets = []

    res = 0
    for i in range(0, len(inputs), 3):
        a = json.loads(inputs[i])
        b = json.loads(inputs[i+1])
        packets.append(a)
        packets.append(b)

        if check(a, b):
            res += i//3 + 1

    #print(res)

    packets.append([[2]])
    packets.append([[6]])

    packets = sorted(packets, key=cmp_to_key(cmpfunc))

    res = 1
    for i,k in enumerate(packets):
        if k == [[2]] or k == [[6]]:
            res *= i+1
        print(k)

    print(res)