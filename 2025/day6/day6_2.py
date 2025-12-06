def compose_number(c1, c2, c3, c4, op):
    string = (c1 + c2 + c3 + c4).strip()
    if string == "":
        match op:
            case "+": string = "0"
            case "*": string = "1"
    return int(string)

input = [line for line in open("input", "r").readlines()]

numbers = input[:-1]
operations = input[-1]

nrows = len(input)
ncols = len(input[0])
tot = 0
tmp = 0

for col in range(ncols):
    c1, c2, c3, c4, operation_raw = numbers[0][col], numbers[1][col], numbers[2][col], numbers[3][col], operations[col]
    
    match operation_raw:
        case " ":
            if operation == "+":
                tmp += compose_number(c1,c2,c3,c4,operation)
            else:
                tmp *= compose_number(c1,c2,c3,c4,operation)

        case "*":
            tot += tmp
            operation = "*"
            tmp = compose_number(c1,c2,c3,c4,operation)
        
        case "+":
            tot += tmp
            operation = "+"
            tmp = compose_number(c1,c2,c3,c4, operation)

tot += tmp
print(tot)


