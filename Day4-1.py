import re

pattern = r'XMAS'
WORD_LENGTH = 4

def get_data():
    with open('Day4_data.txt', 'r') as data:
        return data.read().splitlines()

def search_same_row(data) -> int:
    instances = 0
    for row in data:
        instances += len(re.findall(pattern, row))
        instances += len(re.findall(pattern[::-1], row))

    return instances

def _search_left_up(data, row, character, search_pattern) -> bool:
    for length in range(len(search_pattern)):
        if data[row - length][character - length] != search_pattern[length]:
            return False
    return True

def _search_left_down(data, row, character, search_pattern) -> bool:
    for length in range(len(search_pattern)):
        if data[row + length][character - length] != search_pattern[length]:
            return False
    return True

def _search_right_up(data, row, character, search_pattern) -> bool:
    for length in range(len(search_pattern)):
        if data[row - length][character + length] != search_pattern[length]:
            return False
    return True

def _search_right_down(data, row, character, search_pattern) -> bool:
    for length in range(len(search_pattern)):
        if data[row + length][character + length] != search_pattern[length]:
            return False
    return True

def _search_up(data, row, character, search_pattern) -> bool:
    for length in range(len(search_pattern)):
        if data[row - length][character] != search_pattern[length]:
            return False
    return True

def _search_down(data, row, character, search_pattern) -> bool:
    for length in range(len(search_pattern)):
        if data[row + length][character] != search_pattern[length]:
            return False
    return True

def search_diagonal(data) -> int:
    instances = 0

    for row in range(len(data)):
        upward_possible = (row >= WORD_LENGTH - 1)
        downward_possible = (row <= len(data) - WORD_LENGTH)

        for character in range(len(data[row])):
            leftward_possible = (character >= WORD_LENGTH - 1)
            rightward_possible = (character <= len(data[row]) - WORD_LENGTH)

            if data[row][character] == "X":
                if upward_possible and leftward_possible:
                    instances += int(_search_left_up(data, row, character, pattern))
                if downward_possible and leftward_possible:
                    instances += int(_search_left_down(data, row, character, pattern))
                if upward_possible and rightward_possible:
                    instances += int(_search_right_up(data, row, character, pattern))
                if downward_possible and rightward_possible:
                    instances += int(_search_right_down(data, row, character, pattern))
                if upward_possible:
                    instances += int(_search_up(data, row, character, pattern))
                if downward_possible:
                    instances += int(_search_down(data, row, character, pattern))
    return instances

data = get_data()
same_row = search_same_row(data) 
diagonal = search_diagonal(data)
print(same_row)
print(diagonal)
print(same_row + diagonal)