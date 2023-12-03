from functools import reduce

f = open("input2.txt", "r")
row_length = 10
input_str = f.read().splitlines()
symbols = "#*$%&@!^+/|-"
nums = []
index = 0
stars = []
to_multiply = []

for t, line in enumerate(input_str):
    index = 0
    while index < len(line):
        char = line[index]
        # If a number, check if part of a larger number. If so, grab the whole number
        if char.isdigit():
            first_idx = index
            curr_num = char
            index += 1

            # TODO: this will be wrong due to next line runover
            while index < len(line) and line[index].isdigit():
                curr_num += line[index]
                index += 1
            nums.append((int(curr_num), t, first_idx, index - 1))
            continue
        if char == "*":
            stars.append((char, t, index, index))
        index += 1
print(stars, nums)
for star in stars:
    # check if a number is next to or diagonal to the star
    # do this by checking if a number's range overlaps with the star's up, left, right, down, and diagonals
    # use nums and stars arrays
    star_line_num = star[1]
    star_idx = star[2]
    truths = []
    for num in nums:
        num_start = num[2]
        num_end = num[3]
        num_line = num[1]
        # check if the number is to the left or right of the star
        if num_line == star_line_num and (
            num_end == star_idx - 1 or num_start == star_idx + 1
        ):
            truths.append(num[0])
        # check if the number is above or below the star
        if num_line == star_line_num - 1 or num_line == star_line_num + 1:
            if num_start <= star_idx <= num_end:
                truths.append(num[0])
            elif num_start <= star_idx + 1 <= num_end:
                truths.append(num[0])
            elif num_start <= star_idx - 1 <= num_end:
                print(num_start, num_end, num[0])
                truths.append(num[0])
    if len(truths) >= 2:
        print(truths)
        # multiplty truths
        result = reduce(lambda x, y: x * y, truths)
        to_multiply.append(result)
print(sum(to_multiply))
