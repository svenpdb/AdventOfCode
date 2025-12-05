def check_overlap(range1, range2):
    x1, x2 = range1
    y1, y2 = range2

    return (x1 <= y1 <= x2 or y1 <= x2 <= y2
            or y1 <= x1 <= y2 or x1 <= y2 <= x2)

ranges, _ = open("input","r").read().split("\n\n")
ranges = ranges.split("\n")
count = 0

idranges = []

for idrange in ranges:
    a, b = idrange.split("-")
    idranges.append((int(a), int(b)))

cont = True
while cont:
    cont = False
    length = len(idranges)
    for r1 in range(length-1):
        for r2 in range(r1+1, length):
            if check_overlap(idranges[r1], idranges[r2]):
                cont = True
                new_overlap = (min(idranges[r1][0], idranges[r2][0])
                             , max(idranges[r1][1], idranges[r2][1]))
                idranges[r1] = new_overlap
                idranges.pop(r2)
                break
        if cont:
            break


print(idranges)
tot = 0
for idrange in idranges:
    start, end = idrange
    tot += end - start + 1
print(tot)



