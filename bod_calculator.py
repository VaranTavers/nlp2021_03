import numpy as np
import csv
import sys

def get_word_map():
    reader = csv.reader(open('./word_index.csv'))

    word_map = {}
    for row in reader:
        key = row[0]
        word_map[key] = int(row[1])

    return word_map

def get_bod_mat():
    reader = csv.reader(open('./bag_of_docs.csv'))

    mat_py = []

    for row in reader:
        if len(row) > 1:
            l = list(map(lambda x : int(x), row[0:10000]))
            mat_py.append(l)

    mat_np = np.array(mat_py);
    
    return mat_np

def get_similarity(word1, word2, word_map, mat_np):
    if word_map.get(word1) == None or word_map.get(word2) == None:
        return None
    vec1 = mat_np[:, word_map[word1]]
    vec2 = mat_np[:, word_map[word2]]
    result = np.inner(vec1, vec2) / (np.linalg.norm(vec1) * np.linalg.norm(vec2))

    return (result + 1) / 2


def main():

    word_map = get_word_map()
    mat_np = get_bod_mat()

    sum_sq_err = 0
    count_sq_err = 0

    reader = csv.reader(open('./wordsim353.csv'))

    for row in reader:
        if len(row) > 1:
            ws_value = float(row[2]) / 10
            bod_value = get_similarity(row[0], row[1], word_map, mat_np)
            if bod_value != None:
                sum_sq_err += (ws_value - bod_value)**2
                count_sq_err += 1
    print (count_sq_err)
    print (sum_sq_err / count_sq_err)

if __name__ == "__main__":
    main()
