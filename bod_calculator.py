import numpy as np
import csv
import sys

reader = csv.reader(open('./word_index.csv'))

word_map = {}
for row in reader:
    key = row[0]
    word_map[key] = int(row[1])

reader = csv.reader(open('./bag_of_docs.csv'))

mat_py = []

for row in reader:
    if len(row) > 1:
        mat_py.append(row)
    else:
        print(len(row))

mat_np = np.array(mat_py);

if len(sys.argv) < 3:
    print (sys.argv[0]+" [FIRST_WORD] [SECOND_WORD]")
    exit(1)

print (np.shape(mat_np))

vec1 = mat_np[:, word_map[sys.argv[1]]]
vec2 = mat_np[:, word_map[sys.argv[2]]]

result = vec1 * vec2 / (np.linalg.norm(vec1) * np.linalg.norm(vec2))

print ("Result: " + string(result))
