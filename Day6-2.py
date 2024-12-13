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

    while True:
        if row == len(map) - 1:
            end_reached = True
            break
        if map[row + 1][column] != "#":
            row += 1
            _update_map(map, row, column)
        else:
            break

    start_coordinate.clear()
    start_coordinate.extend([row, column])
    return end_reached

def _move_up(map, start_coordinate) -> bool:
    row = start_coordinate[0]
    column = start_coordinate[1]
    end_reached = False
    
    while True:
        if row == 0:
            end_reached = True
            break
        if map[row - 1][column] != "#":
            row -= 1
            _update_map(map, row, column)
        else:
            break
    
    start_coordinate.clear()
    start_coordinate.extend([row, column])
    return end_reached

def _move_right(map, start_coordinate) -> bool:
    row = start_coordinate[0]
    column = start_coordinate[1]
    end_reached = False

    while True:
        if column == len(map[row]) - 1:
            end_reached = True
            break
        if map[row][column + 1] != "#":
            column += 1
            _update_map(map, row, column)
        else:
            break
    
    start_coordinate.clear()
    start_coordinate.extend([row, column])
    return end_reached

def _move_left(map, start_coordinate) -> bool:
    row = start_coordinate[0]
    column = start_coordinate[1]
    end_reached = False

    while True:
        if column == 0:
            end_reached = True
            break
        if map[row][column - 1] != "#":
            column -= 1
            _update_map(map, row, column)
        else:
            break
    
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

def _add_blockage(map, row, column):
    row_list = list(map[row])
    row_list[column] = "#"
    map[row] = "".join(row_list)

def move_with_loop(map, start_position):
    loop_options = 0
    for row in range(len(map)):
        for column in range(len(map[row])):
            temp_map = map.copy()
            temp_start_position = start_position.copy()

            if temp_map[row][column] != "^" and temp_map[row][column] != "#":
                _add_blockage(temp_map, row, column)

                positions = 0
                prev_positions = positions
                while True:
                    if _move_up(temp_map, temp_start_position):
                        break
                    if _move_right(temp_map, temp_start_position):
                        break
                    if _move_down(temp_map, temp_start_position):
                        break
                    if _move_left(temp_map, temp_start_position):
                        break
                    positions = get_distinct_positions(temp_map)
                    if prev_positions == positions:
                        loop_options += 1
                        break
                    else:
                        prev_positions = positions
    return loop_options

def get_locations(map, start_location):
    locations = []
    for row in range(len(map)):
        for column in range(len(map[row])):
            if start_location == [row, column]:
                break
            if map[row][column] == "X":
                locations.append([row, column])
    return locations

def move_along_orig_path(map, start_position, locations):
    loop_options = 0

    for location in locations:
        temp_map = map.copy()
        temp_start_position = start_position.copy()

        _add_blockage(temp_map, location[0], location[1])
        print(location)
        
        positions = 0
        prev_positions = positions
        while True:
            if _move_up(temp_map, temp_start_position):
                break
            if _move_right(temp_map, temp_start_position):
                break
            if _move_down(temp_map, temp_start_position):
                break
            if _move_left(temp_map, temp_start_position):
                break
            positions = get_distinct_positions(temp_map)
            if prev_positions == positions:
                loop_options += 1
                break
            else:
                prev_positions = positions

    print(temp_map)
    return loop_options
                
map = get_map()
start_position = get_start_position(map)
move(map, start_position)
max_positions = get_distinct_positions(map)
print(map)

locations = get_locations(map, start_position)

print(max_positions)
map = get_map()
start_position = get_start_position(map)
# Correct answer
print(move_with_loop(map, start_position))

map = get_map()
start_position = get_start_position(map)
# one too low
print(move_along_orig_path(map, start_position, locations))
