
def get_map():
    with open('Day8_map.txt', 'r') as map:
        return map.read().splitlines()

def _within_limits(location, max_row, max_column):
    return location[0] >= 0 and location[1] >= 0 and location[0] < max_row and location[1] < max_column

def _calculate_antinodes(antenna1, antenna2, max_row, max_column):
    x1, y1 = antenna1
    x2, y2 = antenna2 # Calculate the difference in x and y

    dx = x2 - x1
    dy = y2 - y1 # Calculate the new locations

    new_loc1 = [x1 - dx, y1 - dy]
    new_loc2 = [x2 + dx, y2 + dy]
    
    antinodes = []
    if _within_limits(new_loc1, max_row, max_column):
        antinodes.append(new_loc1)
    if _within_limits(new_loc2, max_row, max_column):
        antinodes.append(new_loc2)

    return antinodes

def _remove_doubles(arrays):
    # Convert each sub-array to a tuple to make them hashable
    unique_arrays = list(set(tuple(array) for array in arrays)) # Convert each tuple back to a list
    unique_arrays = [list(array) for array in unique_arrays]
    return unique_arrays

def get_antinode(map):
    antinodes = []
    for antenna_row in range(len(map)):
        for antenna_column in range(len(map[antenna_row])):
            if map[antenna_row][antenna_column] != '.':
                antenna = map[antenna_row][antenna_column]
                antenna_antinodes = []
                for row in range(len(map)):
                    for column in range(len(map[row])):
                        if row != antenna_row and column != antenna_column and antenna == map[row][column]:
                            antenna_antinodes.extend(_calculate_antinodes([antenna_row, antenna_column], [row, column], len(map), len(map[0])))
                antinodes.extend(antenna_antinodes)
    antinodes = _remove_doubles(antinodes)
    return antinodes

map = get_map()
print(len(get_antinode(map)))

