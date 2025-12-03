input = list(map(str, open("input2_2", "r").read().strip().split(",")))

som = 0

for i in input:
    start, end = map(int, i.split("-"))
    for num in range(start, end + 1):
        string = str(num)
        length = len(string)
        half = int(length / 2)
        
        if length % 2 != 0:
            continue
        if string[:half] == string[half:]: som += int(string)
        
print(som)