input = list(map(str, open("input2_2", "r").read().strip().split(",")))

som = 0
maxlength = 0

for i in input:
    start, end = map(int, i.split("-"))
    length = len(str(end))
    if length > maxlength:
        maxlength = length

primes = [2, 3, 5, 7]

for i in input:
    start, end = map(int, i.split("-"))
    for num in range(start, end + 1):
        string = str(num)
        length = len(string)
        for p in primes:
            if length % p != 0: continue
            div = int(length / p)
            s = set([string[j*div:(j+1)*div] for j in range(0, p)])
            if len(s) == 1: 
                som += int(string)
                break
print(som)