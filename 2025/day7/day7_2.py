input = list(list(row.strip()) for row in open("input", "r").readlines())


nrows = len(input)
ncols = len(input[0])

for ri, row in enumerate(input):
    if "S" in row:
        start = (ri, row.index("S"))

beams = [int(i == start[1]) for i in range(ncols)]


for row in range(1,nrows):
    if "^" not in input[row]: continue
    newbeams = beams[:]

    for beam in range(ncols):
        if input[row][beam] == "^" and beams[beam]:

            newbeams[beam] = 0
            newbeams[beam-1] += beams[beam]
            newbeams[beam+1] += beams[beam]
    beams = newbeams[:]

print(sum(beams))
