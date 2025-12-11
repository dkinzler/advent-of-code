def main():
    part1()
    part2()

def parse_input(file):
    grid = []
    with open(file, "r") as f:
        for line in f:
            s = line.strip()
            row = []
            for x in s:
                if x == "@":
                    row.append(True)
                else:
                    row.append(False)
            grid.append(row)

    return grid

def is_pos_valid(r, c, m, n):
    if r < 0 or r >= m or c < 0 or c >= n:
        return False
    return True

def part1():
    file="input"
    #file="example"
    grid = parse_input(file)
    
    m = len(grid)
    n = len(grid[0])

    result = 0
    for r in range(m):
        for c in range(n):
            if not grid[r][c]:
                continue

            adjacent_papers = 0
            for dr in range(-1, 2, 1):
                for dc in range(-1, 2, 1):
                    if dr == 0 and dc == 0:
                        continue
                    nr = r + dr 
                    nc = c + dc
                    if is_pos_valid(nr, nc, m, n) and grid[nr][nc]:
                        adjacent_papers += 1
            if adjacent_papers < 4:
                result += 1

    print(f"part1: {result}")

def part2():
    file="input"
    #file="example"
    grid = parse_input(file)
    
    m = len(grid)
    n = len(grid[0])

    def can_remove(r, c):
        if not grid[r][c]:
            return False

        adjacent_papers = 0
        for dr in range(-1, 2, 1):
            for dc in range(-1, 2, 1):
                if dr == 0 and dc == 0:
                    continue
                nr = r + dr 
                nc = c + dc
                if is_pos_valid(nr, nc, m, n) and grid[nr][nc]:
                    adjacent_papers += 1

        return adjacent_papers < 4

    result = 0
    q = []
    qi = 0

    for r in range(m):
        for c in range(n):
            if can_remove(r, c):
                result += 1
                grid[r][c] = False
                q.append((r, c))

    while qi < len(q):
        r, c = q[qi]
        qi += 1
        for dr in range(-1, 2, 1):
            for dc in range(-1, 2, 1):
                if dr == 0 and dc == 0:
                    continue
                nr = r + dr 
                nc = c + dc
                if is_pos_valid(nr, nc, m, n) and grid[nr][nc]:
                    if can_remove(nr, nc):
                        result += 1
                        grid[nr][nc] = False
                        q.append((nr, nc))

    print(f"part2: {result}")

if __name__ == "__main__":
    main()
