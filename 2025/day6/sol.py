def main():
    part1()
    part2()

def parse_input(file):
    grid = []
    with open(file, "r") as f:
        for line in f:
            s = line.strip()

            row = []
            parts = s.split()
            for p in parts:
                if len(p) > 0:
                    row.append(p)

            if row[0].isnumeric(): 
                row = [int(x) for x in row]

            grid.append(row)

    return grid

def part1():
    file="input"
    #file="example"
    grid = parse_input(file)

    result = 0
    m = len(grid)
    n = len(grid[0])
    for j in range(n):
        if grid[m-1][j] == "+":
            v = 0
            for i in range(m-1):
                v += grid[i][j]
            result += v
        else:
            v = 1
            for i in range(m-1):
                v *= grid[i][j]
            result += v

    print(f"part1: {result}")

def parse_input2(file):
    grid = []
    with open(file, "r") as f:
        for line in f:
            grid.append(line.rstrip('\n'))

    m = len(grid)
    n = len(grid[0])
    nums = []
    curr = []
    for j in range(n):
        # parse number bottom to top
        # note that there are no zeros in the input
        # so if v == 0 at the end we know this was the divider
        # between two columns
        v = 0
        exp = 1
        for i in range(m-2, -1, -1):
            if grid[i][j] != ' ':
                v += int(grid[i][j]) * exp
                exp *= 10
        if v > 0:
            curr.append(v)
        else:
            nums.append(curr)
            curr = []

    nums.append(curr)

    ops = []
    i = 0
    for j in range(len(grid[m-1])):
        if grid[m-1][j] != ' ':
            ops.append(grid[m-1][j])
            i += 1

    return (nums, ops)

def part2():
    file="input"
    #file="example"
    nums, ops = parse_input2(file)

    result = 0
    for i in range(len(ops)):
        if ops[i] == '+':
            result += sum(nums[i])
        else:
            v = 1
            for x in nums[i]:
                v *= x
            result += v

    print(f"part2: {result}")
    
if __name__ == "__main__":
    main()
