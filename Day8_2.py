
def get_map():
    with open('Day8_map.txt', 'r') as map:
        return map.read().splitlines()

def _within_limits(location, max_row, max_column):
    return location[0] >= 0 and location[1] >= 0 and location[0] < max_row and location[1] < max_column

def _merge_and_remove_duplicates(arrays):
    merged_dict = {}
    for array in arrays:
        key = array[0]
        if key not in merged_dict:
            merged_dict[key] = []
        merged_dict[key].extend(array[1:]) # Remove duplicate sub-arrays
    
    for key in merged_dict:
        merged_dict[key] = [list(x) for x in set(tuple(x) for x in merged_dict[key])]
        
    merged_arrays = [[key] + value for key, value in merged_dict.items()]

    all_antinodes = []
    for antenna_group in range(len(merged_arrays)):
        for antinode in range(len(merged_arrays[antenna_group])):
            if antinode != 0:
                all_antinodes.append(merged_arrays[antenna_group][antinode])

    return all_antinodes

def _calculate_antinodes(antenna1, antenna2, max_row, max_column):
    x1, y1 = antenna1
    x2, y2 = antenna2 # Calculate the difference in x and y

    dx = x2 - x1
    dy = y2 - y1 # Calculate the new locations

    antinodes = []
    ii = 1
    while(True):
        new_loc1 = [x1 - dx * ii, y1 - dy * ii]

        if _within_limits(new_loc1, max_row, max_column):
            antinodes.append(new_loc1)
            ii += 1
        else:
            break
        
    ii = 1
    while(True):
        new_loc2 = [x2 + dx * ii, y2 + dy * ii]

        if _within_limits(new_loc2, max_row, max_column):
            antinodes.append(new_loc2)
            ii += 1
        else:
            break

    return antinodes

def get_antinode(map):
    antinodes = []
    linked_antennas = []
    for antenna_row in range(len(map)):
        for antenna_column in range(len(map[antenna_row])):
            if map[antenna_row][antenna_column] != '.':
                antenna = map[antenna_row][antenna_column]
                antenna_antinodes = [antenna]
                antenna_linked_antennas = [antenna]
                for row in range(len(map)):
                    for column in range(len(map[row])):
                        if row != antenna_row and column != antenna_column and antenna == map[row][column]:
                            antenna_linked_antennas.append([row,column])
                            antenna_antinodes.extend(_calculate_antinodes([antenna_row, antenna_column], [row, column], len(map), len(map[0])))
                antinodes.append(antenna_antinodes)
                linked_antennas.append(antenna_linked_antennas)
    antinodes = _merge_and_remove_duplicates(antinodes)
    return [antinodes, linked_antennas]

def filter_antennas(antennas):
    for antenna_array in reversed(range(len(antennas))):
        if len(antennas[antenna_array]) <= 3:
            antennas.pop(antenna_array)
    return _merge_and_remove_duplicates(antennas)

def get_linked_antennas(linked_antennas):
    num_of_antennas = 0
    for antannas in linked_antennas:
        num_of_antennas += len(antannas) - 1
    return num_of_antennas

map = get_map()
antinodes, linked_antennas = get_antinode(map)
linked_antennas = filter_antennas(linked_antennas)
antennas = get_linked_antennas(linked_antennas)

for row in range(len(map)):
    for column in range(len(map[row])):
        if map[row][column] != '.' and [row, column] in antinodes:
            antinodes.remove([row, column])

print(len(antinodes))

# for row in range(len(map)):
#     print_data = ""
#     for column in range(len(map[row])):
#         if map[row][column] != '.':
#             print_data += (map[row][column])
#         elif [row, column] in antinodes:
#             print_data += '#'
#         else:
#             print_data += '.'
#     print(print_data)