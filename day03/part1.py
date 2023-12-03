f = open("input1.txt", "r")
row_length = 10
input_str = f.read().splitlines()
symbols = "#*$%&@!^+/|-"
nums = []
index = 0

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
            index -= 1
            print("curr_num: ", curr_num)
            # Check if a symbol is next to or diagonal to the number
            top = []
            bottom = []
            left = ""
            right = ""
            top_left_diag = ""
            top_right_diag = ""
            bottom_left_diag = ""
            bottom_right_diag = ""
            # top
            if t > 0:
                top = input_str[t - 1][first_idx : index + 1]
            # bottom
            if t < len(input_str) - 1:
                bottom = input_str[t + 1][first_idx : index + 1]
            # left
            if first_idx > 0:
                left = line[first_idx - 1]
            # right
            if (index + 1) < len(line):
                right = line[index + 1]
            # top left diag
            if t > 0 and first_idx != 0:
                top_left_diag = input_str[t - 1][first_idx - 1]
            # top right diag
            if t > 0 and index + 1 <= len(line) - 1:
                top_right_diag = input_str[t - 1][index + 1]
            # bottom left diag
            if t < len(input_str) - 1 and first_idx != 0:
                bottom_left_diag = input_str[t + 1][first_idx - 1]
            # bottom rigth diag
            if t < len(input_str) - 1 and index + 1 <= len(line) - 1:
                bottom_right_diag = input_str[t + 1][index + 1]
            # check if any are in symbols
            if (
                any(not x.isdigit() and x != "." for x in top)
                or any(not x.isdigit() and x != "." for x in bottom)
                or (not left == "" and not left.isdigit() and left != ".")
                or (not right == "" and not right.isdigit() and right != ".")
                or (
                    not top_left_diag == ""
                    and not top_left_diag.isdigit()
                    and top_left_diag != "."
                )
                or (
                    not top_right_diag == ""
                    and not top_right_diag.isdigit()
                    and top_right_diag != "."
                )
                or (
                    not bottom_left_diag == ""
                    and not bottom_left_diag.isdigit()
                    and bottom_left_diag != "."
                )
                or (
                    not bottom_right_diag == ""
                    and not bottom_right_diag.isdigit()
                    and bottom_right_diag != "."
                )
            ):
                # print any symbols found and the number
                print("Symbols found: ", end="")
                if any(not x.isdigit() and x != "." for x in top):
                    print("top: ", top)
                if any(not x.isdigit() and x != "." for x in bottom):
                    print("bottom: ", bottom)
                if not top_left_diag.isdigit() and top_left_diag != ".":
                    print("tld: ", top_left_diag)
                if not top_right_diag.isdigit() and top_right_diag != ".":
                    print("trd: ", top_right_diag)
                    print(top_right_diag in symbols)
                if bottom_left_diag in symbols:
                    print("bld: ", bottom_left_diag)
                if bottom_right_diag in symbols:
                    print("brd: ", bottom_right_diag)
                print("Number found: ", end="")
                print(curr_num)
                nums.append(int(curr_num))
            index += 1
        else:
            index += 1

print(nums)
print(sum(nums))
