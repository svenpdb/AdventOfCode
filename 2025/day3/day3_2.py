with open("input3_2", "r") as f:
    banks = f.readlines()

som = 0
num_batteries = 12

for bank in banks:
    bank = list(map(int, bank.strip()))

    batterystring = ""
    length = len(bank)
    largest_index = -1
    
    for battery in range(num_batteries):
        largestnum = 0
        
        for index, i in enumerate(bank[largest_index+1:(length - num_batteries + battery + 1)], largest_index+1):
            if int(i) > largestnum: 
                largestnum = i
                largestindex_next = index
        batterystring += str(largestnum)
        largest_index = largestindex_next
    
    som += int(batterystring)
print(som)