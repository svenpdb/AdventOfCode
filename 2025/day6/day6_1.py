input = [line.split() for line in open("input", "r").readlines()]

numbers = input[:-1]
operations = input[-1]

nrows = len(input)
ncols = len(input[0])
tot = 0

for col in range(ncols):
    nums = [int(numbers[i][col]) for i in range(nrows - 1)]
    match operations[col]:
        case "+":
            tot += sum(nums)
        case "*":
            tmp = 1
            for num in nums:
                tmp *= num
            tot += tmp

print(tot)
