with open("input", "r") as f:
    lines = f.readlines()


input = [list(line.strip()) for line in lines]

cols = len(input)
rows = len(input[0])
tot = 0

while True:
    res = 0
    for x in range(cols):
        for y in range(rows):
            if input[y][x] == ".": continue
            count = 0
            for i, j in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]:
                xnew = x + i
                ynew = y + j
                if 0 <= xnew < rows and 0 <= ynew < cols:
                    pass
                else:
                    continue
                if input[ynew][xnew] == "@":
                    count += 1

            if count < 4:
                res += 1
                input[y][x] = "."
    if res == 0: break
    tot += res

print(tot)
