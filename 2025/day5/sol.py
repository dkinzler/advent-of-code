def main():
    part1()
    part2()

def parse_input(file):
    fresh_ranges = []
    ingredients = []
    with open(file, "r") as f:
        in_first_section=True
        for line in f:
            s = line.strip()
            if in_first_section:
                if len(s) == 0:
                    in_first_section=False
                    continue
                parts = s.split("-")
                left = int(parts[0])
                right = int(parts[1])
                fresh_ranges.append((left, right))
            else:
                ingredients.append(int(s))

    return fresh_ranges, ingredients

def part1():
    file="input"
    #file="example"
    fresh_ranges, ingredients = parse_input(file)

    result = 0
    for ing in ingredients:
        for (left, right) in fresh_ranges:
            if left <= ing and ing <= right:
                result +=1
                break

        
    print(f"part1: {result}")

def part2():
    file="input"
    #file="example"
    fresh_ranges, ingredients = parse_input(file)

    # find the total number of fresh ingredient ids
    # the ranges are not disjoint, so you need to combine them where possible

    # sort ranges in ascending order of left value
    fresh_ranges.sort(key=lambda x: x[0])
    combined = []
    curr_left, curr_right = fresh_ranges[0]
    for (left, right) in fresh_ranges[1:]:
        if curr_right < left:
            # there can be no more overlap with any range
            combined.append((curr_left, curr_right))
            curr_left, curr_right = left, right
        else:
            # they overlap, combine into a single range
            curr_right = max(right, curr_right)

    combined.append((curr_left, curr_right))

    result = 0
    for (left, right) in combined:
        result += right-left+1
        
    print(f"part2: {result}")
    
if __name__ == "__main__":
    main()
