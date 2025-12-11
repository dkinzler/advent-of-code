def main():
    part1()
    part2()

def parse_input(file):
    grid = []
    with open(file, "r") as f:
        for line in f:
            s = line.strip()
            grid.append(line)

    return grid

def part1():
    file="input"
    #file="example"
    grid = parse_input(file)

    m = len(grid)
    n = len(grid[0])

    # there can only ever be one beam in a cell
    # solve it row by row, lets assume we have computed the
    # set of cells that contain a beam in the previous row
    # then if there is a beam at cell j in row r-1, there will
    # be a beam in cell j in row r if that cell is empty
    # if it contains a splitter, there will be a beam at cell j-1
    # and cell j+1
    splitters_used = 0

    has_beam = [[False for c in range(n)] for r in range(m)]
    for c in range(n):
        if grid[0][c] == 'S':
            has_beam[0][c] = True
            break

    for r in range(1, m):
        for c in range(n):
            if has_beam[r-1][c]:
                if grid[r][c] == '.':
                    has_beam[r][c] = True
                elif grid[r][c] == '^':
                    splitters_used += 1
                    if c > 0:
                        has_beam[r][c-1] = True
                    if c < n-1:
                        has_beam[r][c+1] = True

    print(f"part1: {splitters_used}")

def part2():
    file="input"
    #file="example"
    grid = parse_input(file)

    m = len(grid)
    n = len(grid[0])

    # what do we need here? the number of unique paths = "timelines"
    # a particle can take?
    # we can compute this with DP
    # let d(r, c) = number of unique paths to get to cell (r,c)
    # d(r, c) = 1 for the starting cell
    # d(r, c) = 0 if there is a splitter in that cell
    # d(r, c) = d(r-1, c)
    #           + d(r-1, c+1) if there is a splitter to the right of it
    #           + d(r-1, c-1) if there is a splitter to the left of it

    d = [[0 for c in range(n)] for r in range(m)]
    for c in range(n):
        if grid[0][c] == 'S':
            d[0][c] = 1
            break

    for r in range(1, m):
        for c in range(n):
            if grid[r][c] != '.':
                continue

            v = d[r-1][c]
            if c < n-1 and grid[r][c+1] == '^':
                v += d[r-1][c+1]
            if c > 0 and grid[r][c-1] == '^':
                v += d[r-1][c-1]
            d[r][c] = v

    result = sum(d[m-1])

    print(f"part1: {result}")

if __name__ == "__main__":
    main()
