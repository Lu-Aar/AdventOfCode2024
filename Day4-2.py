import re
import math

pattern = r'MAS'
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

def _search_diag_start_left_top(data, row, character, search_pattern) -> bool:
    letters_till_middle = math.floor(len(search_pattern) / 2)
    use_invert_pattern = (data[row - letters_till_middle][character - letters_till_middle] == search_pattern[-1:])
    for length in range(len(search_pattern)):
        diag_steps_row = row - letters_till_middle + length
        diag_steps_char = character - letters_till_middle + length
        if not use_invert_pattern:
            if data[diag_steps_row][diag_steps_char] != search_pattern[length]:
                return False
        else:
            if data[diag_steps_row][diag_steps_char] != search_pattern[::-1][length]:
                return False
    return True

def _search_diag_start_left_bottom(data, row, character, search_pattern) -> bool:
    letters_till_middle = math.floor(len(search_pattern) / 2)
    use_invert_pattern = (data[row + letters_till_middle][character - letters_till_middle] == search_pattern[-1:])

    for length in range(len(search_pattern)):
        diag_steps_row = row + letters_till_middle - length
        diag_steps_char = character - letters_till_middle + length
        if not use_invert_pattern:
            if data[diag_steps_row][diag_steps_char] != search_pattern[length]:
                return False
        else:
            if data[diag_steps_row][diag_steps_char] != search_pattern[::-1][length]:
                return False
    return True


def search_diagonal(data) -> int:
    instances = 0

    for row in range(len(data)):
        upward_possible = (row >= math.floor(len(pattern) / 2))
        downward_possible = (row <= len(data) - math.ceil(len(pattern) / 2))

        for character in range(len(data[row])):
            leftward_possible = (character >= math.floor(len(pattern) / 2))
            rightward_possible = (character <= len(data[row]) - math.ceil(len(pattern) / 2))

            if data[row][character] == "A":
                if upward_possible and downward_possible and leftward_possible and rightward_possible:
                    instances += int(_search_diag_start_left_top(data, row, character, pattern) and _search_diag_start_left_bottom(data, row, character, pattern))
    return instances

data = get_data()
diagonal = search_diagonal(data)
print(diagonal)