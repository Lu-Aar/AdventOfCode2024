import re

def _update_map(map, row, column):
    row_list = list(map[row])
    row_list[column] = "X"
    map[row] = "".join(row_list)

def get_map():
    with open('Day6_map.txt', 'r') as map:
    # with open('Day6_testmap.txt', 'r') as map:
        return map.read().splitlines()

def get_start_position(map):
    for row in range(len(map)):
        for column in range(len(map[row])):
            if map[row][column] == "^":
                _update_map(map, row, column)
                return [row, column]

def _move_down(map, start_coordinate) -> bool:
    row = start_coordinate[0]
    column = start_coordinate[1]
    end_reached = False

    try:
        while True:
            if map[row + 1][column] != "#":
                row += 1
                _update_map(map, row, column)
            else:
                break
    except:
        end_reached = True

    start_coordinate.clear()
    start_coordinate.extend([row, column])
    return end_reached

def _move_up(map, start_coordinate) -> bool:
    row = start_coordinate[0]
    column = start_coordinate[1]
    end_reached = False
    try:
        while True:
            if map[row - 1][column] != "#":
                row -= 1
                _update_map(map, row, column)
            else:
                break
    except:
        end_reached = True
    
    start_coordinate.clear()
    start_coordinate.extend([row, column])
    return end_reached

def _move_right(map, start_coordinate) -> bool:
    row = start_coordinate[0]
    column = start_coordinate[1]
    end_reached = False

    try:
        while True:
            if map[row][column + 1] != "#":
                column += 1
                _update_map(map, row, column)
            else:
                break
    except:
        end_reached = True
    
    start_coordinate.clear()
    start_coordinate.extend([row, column])
    return end_reached

def _move_left(map, start_coordinate) -> bool:
    row = start_coordinate[0]
    column = start_coordinate[1]
    end_reached = False

    try:
        while True:
            if map[row][column - 1] != "#":
                column -= 1
                _update_map(map, row, column)
            else:
                break
    except:
        end_reached = True
    
    start_coordinate.clear()
    start_coordinate.extend([row, column])
    return end_reached

def move(map, start_coordinate):
    while True:
        if _move_up(map, start_coordinate):
            break
        if _move_right(map, start_coordinate):
            break
        if _move_down(map, start_coordinate):
            break
        if _move_left(map, start_coordinate):
            break

def get_distinct_positions(map) -> int:
    positions = 0
    for row in map:
        positions += len(re.findall(r"X", row))
    return positions

map = get_map()
start_position = get_start_position(map)
move(map, start_position)
print(get_distinct_positions(map))