def calc_area(c1, c2):
    x1, y1 = c1
    x2, y2 = c2

    dy = abs(y2 - y1) + 1
    dx = abs(x2 - x1) + 1
    return dx * dy

def coord_inbetween(c1, c2):
    x1, y1 = c1
    x2, y2 = c2
    if x1 > x2:
        x1, x2 = x2, x1
    if y1 > y2:
        y1, y2 = y2, y1

    for c in input:
        x, y = c
        if x1 < x < x2 and y1 < y < y2:
            print(f"err: coord in the middle: {(x, y)}")
            return True

    return False


def line_through_area(c1, c2):
    x1, y1 = c1
    x2, y2 = c2

    xmin, xmax = min(x1, x2), max(x1, x2)
    ymin, ymax = min(y1, y2), max(y1, y2)
    
    length = len(input)

    for i in range(length):

        x, y = input[i]
        x_next, y_next = input[(i+1)%length]
        px_min, px_max = min(x, x_next), max(x, x_next)
        py_min, py_max = min(y, y_next), max(y, y_next)

        if py_min <= ymin < ymax <= py_max and (xmin < px_min <= px_max < xmax):
            print(f"err1 at coords {x, y} and {x_next, y_next}")
            return True


        if px_min <= xmin < xmax <= px_max and ymin < py_min <= py_max < ymax:
            print(f"err2 at coords {x, y} and {x_next, y_next}")
            return True

    return False



input = [list(map(int, line.strip().split(","))) for line in open("input", "r").readlines()]

areas = list()
for i1, c1 in enumerate(input):
    for i2, c2 in enumerate(input[i1+1:], i1+1):
        area = calc_area(c1, c2)
        areas.append([area, (i1, i2)])
areas.sort()


while True:
    area, (i1, i2) = areas.pop()
    c1 = input[i1]
    c2 = input[i2]
    print("="*25)
    print("c1,c2: ", c1, c2)
    
    if coord_inbetween(c1, c2):
        continue

    if line_through_area(c1, c2):
        continue

    # if no problems:
    break


print(f"answer: {area}")
