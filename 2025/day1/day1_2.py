input = list(map(str, open("day1_2.txt", "r").read().strip().split()))
count = 0
position = 50
L = 100


for i in input:
    direction, num = i[0], int(i[1:])

    if num >= 100: 
        tmp = int(num) // L
        count += tmp
        num -= 100 * tmp

    match direction:
        case "L":
            if (position != 0 and position - num != (position - num)%L) or (position-num)%L == 0: 
                count += 1 
            position = (position - num)%L
        case "R":
            if (position!= 0 and position + num != (position + num)%L) or (position+num)%L == 0: 
                count += 1
            position = (position + num)%L

print(count)