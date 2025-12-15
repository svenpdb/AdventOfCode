import math

def compute_dist(x1, x2):
    dist_sq = 0
    for d in range(3):
        dist_sq += (x1[d] - x2[d])**2
    return math.sqrt(dist_sq)

def find_circuit(x):
    for i, circ in enumerate(circuits):
        if x in circ:
            return i
    return None
        

path = "input8_1"
# path = "tmp"
input = [tuple(map(int, line.strip().split(","))) for line in open(path).readlines()]

distances = list()

for i1, c1 in enumerate(input):
    for i2, c2 in enumerate(input[i1+1:]):
        distances.append([compute_dist(c1, c2), (c1, c2)])     
distances.sort(reverse=True)

circuits = [{i} for i in input]

count = 0
if path == "tmp": 
    n = 10
else: 
    n = 1000

while count < n:
    
    dist, (c1, c2) = distances.pop()
    
    i1, i2 = find_circuit(c1), find_circuit(c2)
    
    if i1 != i2:
        new_circuit = circuits[i1] | circuits[i2]
        circuits[i1] = new_circuit
        circuits.pop(i2)
        count += 1

    if i1 == i2:
        count += 1
        continue

tot = 1

lengths = [len(i) for i in circuits]
lengths.sort(reverse=True)
tot = lengths[0] * lengths[1] * lengths[2]
print(tot)