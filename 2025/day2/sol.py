def main():
    part1()
    part2()

def part1():
    ranges=[]

    file="input"
    #file="example"
    with open(file, "r") as f:
        for line in f:
            s = line.strip().split(",")
            for r in s:
                z = r.split("-")
                ranges.append((int(z[0]), int(z[1])))

            break

    result = 0
    for (start, end) in ranges:
        for i in range(start, end+1):
            a = str(i)
            if len(a) % 2 == 0:
                n = len(a)//2
                invalid = True
                for j in range(n):
                    if a[j] != a[n+j]:
                        invalid = False
                        break
                if invalid:
                    result += i

    print(f"part1: {result}")

def part2():
    ranges=[]

    file="input"
    #file="example"
    with open(file, "r") as f:
        for line in f:
            s = line.strip().split(",")
            for r in s:
                z = r.split("-")
                ranges.append((int(z[0]), int(z[1])))

            break

    result = 0
    for (start, end) in ranges:
        for i in range(start, end+1):
            a = str(i)
            n = len(a)
            # k = length of repeating sequence
            invalid = False
            for k in range(1, n//2+1):
                if len(a) % k != 0:
                    continue

                m = n//k

                equal = True
                for j in range(k):
                    t = a[j]
                    for s in range(1, m):
                        if t != a[s*k+j]:
                            equal=False
                            break
                    if not equal:
                        break
                
                if equal:
                    invalid = True
                    break
                        

            if invalid:
                result += i

    print(f"part2: {result}")

if __name__ == "__main__":
    main()
