import pandas as pd

# Read
input = pd.read_csv("input", sep=r"\s+", header=None)

list1 = sorted(input[0])
list2 = sorted(input[1])

# Day 1 part 1
sum_abs_diff = sum([abs(b - a) for a, b in zip(list1, list2)])
print(f"1: {sum_abs_diff}")

# Day 1 part 2
similarity = 0
list2_map = {}
for num in list2:
    list2_map[num] = list2_map.get(num, 0) + 1

for num in list1:
    similarity += num * list2_map.get(num, 0)

print(f"2: {similarity}")

# Day 2
