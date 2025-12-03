input = list(map(str, open("day1_1.txt", "r").read().strip().split()))
count = 0
position = 50
for i in input:
    direction, num = i[0], int(i[1:])
    match direction:
        case "L":
            position = (position + num)%100
        case "R":
            position = (position - num)%100
    if position == 0:
        count += 1
print(count)