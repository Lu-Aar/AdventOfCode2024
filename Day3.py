import re

pattern = r'mul\(\d+,\d+\)|do\(\)|don\'t\(\)'

with open('Day3_data.txt', 'r') as data:
    text = data.read()
    matches =  re.findall(pattern, text)

    result = 0
    skip = False
    for match in matches:
        if match == "don't()":
            skip = True
        if match == "do()":
            skip = False
        
        if not skip and match != "do()":
            numbers = re.findall(r'\d+', match)
            result += (int(numbers[0]) * int(numbers[1]))

    print(result)