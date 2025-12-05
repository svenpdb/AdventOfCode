ranges, ids = open("input","r").read().split("\n\n")
count = 0
ranges = ranges.split("\n")
ids = ids.split("\n")[:-1]
for id in ids:
    for idrange in ranges:
        start, end = idrange.split("-")
        if int(id) < int(start) or int(id) > int(end): 
            continue
        else:
            if int(id) <= int(end):
                count += 1
                break
print(count)
