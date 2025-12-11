def main():
    part1()
    part2()

def part1():
    rotations = []
    with open("input", "r") as f:
        for line in f:
            x = line.strip()
            rotations.append((x[0], int(x[1:])))

    password = 0
    curr = 50
    for (dir, count) in rotations:
        if dir == "L":
            curr = (curr - count) % 100
        else:
            curr = (curr + count) % 100
        if curr == 0:
            password += 1

    print(f"part1: {password}")

def part2():
    rotations = []
    with open("input", "r") as f:
        for line in f:
            x = line.strip()
            rotations.append((x[0], int(x[1:])))

    password = 0
    curr = 50
    for (dir, count) in rotations:
        if dir == "L":
            k = count//100
            r = count%100
            password += k
            # e.g. curr=0 and count=100 should only add 1
            if curr > 0 and r >= curr:
                password += 1
            curr = (curr - count) % 100
        else:
            k = count//100
            r = count%100
            password += k
            if curr > 0 and r >= (100-curr):
                password += 1
            curr = (curr + count) % 100

    print(f"part2: {password}")

if __name__ == "__main__":
    main()
