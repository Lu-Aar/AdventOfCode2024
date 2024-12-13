import itertools
import operator

def get_data():
    with open('Day7_data.txt', 'r') as data:
        data_list = data.read().splitlines()

        data_result = []
        for item in data_list:
            temp_data = []
            split_item = item.split(": ")
            temp_data.append(int(split_item[0]))

            behind_dots = []
            for number in split_item[1].split(" "):
                behind_dots.append(int(number))
            temp_data.append(behind_dots)
            
            data_result.append(temp_data)
        return data_result

def _equation_testing(data_point, operators):
    # Generate all possible combinations of operations
    all_combinations = list(itertools.product(operators, repeat=len(data_point[1])-1))
    
    results = []
    for combination in all_combinations:
        result = data_point[1][0]
        for i, op in enumerate(combination):
            if op == operator.concat:
                result = int(op(str(result), str(data_point[1][i+1])))
            else:
                result = op(result, data_point[1][i+1])
        results.append(result)
    return (data_point[0] in results)

def test_equation(data):
    ops = [operator.add, operator.mul, operator.concat]
    total_calibration_results = 0

    for data_point in data:
        if _equation_testing(data_point, ops):
            total_calibration_results += data_point[0]
    
    return total_calibration_results


data = get_data()
print(test_equation(data))

