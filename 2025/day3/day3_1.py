with open("input3_1", "r") as f:
    banks = f.readlines()

som = 0
for bank in banks:
    bank = list(map(int, bank.strip()))
    largestnum = 0
    largestnum2 = 0
    largestindex = 0
    for index, i in enumerate(bank[:-1]):
        if int(i) > largestnum: 
            largestnum, largestindex = int(i), index
    for i in bank[largestindex+1:]:
        if int(i) > largestnum2: 
            largestnum2 = int(i)
    joltage = str(largestnum) + str(largestnum2)
    som += int(joltage)
    
print(som)