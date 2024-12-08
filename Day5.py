import math

def get_update_order() -> list:
    with open('Day5_data_update_order.txt', 'r') as data:
        datapoints = data.read().splitlines()
        return [list(map(int, item.split('|'))) for item in datapoints]

def get_updates() -> list:
    with open('Day5_data_updates.txt', 'r') as data:
        updates = data.read().splitlines()
        return [list(map(int, item.split(','))) for item in updates]

def sort_list(order_list, update_list):
    while True:
        action = False
        for update_spot in range(len(update_list)):
            for order in order_list:
                if update_list[update_spot] == order[0]:
                    try:
                        index = update_list.index(order[1])
                    except:
                        index = -1
                    
                    if index != -1 and index < update_spot:
                        buf = update_list[update_spot]
                        update_list.pop(update_spot)
                        update_list.insert(index, buf)
                        action = True
        if not action:
            break

def check_list_order(order_list, update_list):
    for update_spot in range(len(update_list)):
        for order in order_list:
            if update_list[update_spot] == order[0]:
                try:
                    index = update_list.index(order[1])
                except:
                    index = -1

                if index != -1 and index < update_spot:
                    return False
    return True
update_order = get_update_order()
updates = get_updates()

middle_pages_sum = 0
for update in updates:
    if not check_list_order(update_order, update):
        sort_list(update_order, update)
        middle_pages_sum += update[math.floor(len(update) / 2)]
print(middle_pages_sum)