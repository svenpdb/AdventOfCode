import numpy as np
import matplotlib.pyplot as plt

def calc_area(c1, c2):
    x1, y1 = c1
    x2, y2 = c2

    dy = abs(y2 - y1)+1
    dx = abs(x2 - x1)+1
    return dx * dy


input = [list(map(int, line.strip().split(","))) for line in open("input", "r").readlines()]
maxarea = 0

for i1, c1 in enumerate(input):
    for c2 in input[i1:]:
        area = calc_area(c1, c2)

        if area > maxarea:
            maxarea = area


print(maxarea)
