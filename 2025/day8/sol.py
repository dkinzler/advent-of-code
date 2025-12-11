def main():
    part1()
    part2()

def parse_input(file):
    boxes = []
    with open(file, "r") as f:
        for line in f:
            s = line.strip().split(",")
            boxes.append((int(s[0]), int(s[1]), int(s[2])))

    return boxes

def part1():
    file="input"
    n_pairs = 1000
    #file="example"
    #n_pairs = 10

    boxes = parse_input(file)
    n = len(boxes)


    # connect the 1000 pairs of boxes which are closest together
    # we can use a union-find structure to keep track of the connected
    # components/circuits

    dist = []
    for i in range(n-1):
        a = boxes[i]
        for j in range(i+1, n):
            b = boxes[j]
            d = (a[0]-b[0])**2 + (a[1]-b[1])**2 + (a[2]-b[2])**2
            dist.append((i, j, d))

    dist.sort(key=lambda x: x[2])

    uf = [i for i in range(n)]
    size = [1] * n

    def union(uf, size, a, b):
        if a == b:
            return

        uf[a] = b
        size[b] += size[a]

    def find(uf, x):
        if uf[x] == x:
            return x
        
        r = find(uf, uf[x])
        uf[x] = r
        return r

    for i in range(n_pairs):
        a, b, _ = dist[i]
        fa = find(uf, a)
        fb = find(uf, b)
        if fa != fb:
            union(uf, size, fa, fb)

    circuits = []
    for i in range(n):
        # uf[i] = i means i is the root of a circuit
        if find(uf, i) == i:
            circuits.append(size[i])

    circuits.sort(reverse=True)
    result = circuits[0]*circuits[1]*circuits[2]
    print(f"part1: {result}")

def part2():
    file="input"
    #file="example"

    boxes = parse_input(file)
    n = len(boxes)


    # keep connecting the closest pair of boxes together until
    # all boxes are in one circuit

    dist = []
    for i in range(n-1):
        a = boxes[i]
        for j in range(i+1, n):
            b = boxes[j]
            d = (a[0]-b[0])**2 + (a[1]-b[1])**2 + (a[2]-b[2])**2
            dist.append((i, j, d))

    dist.sort(key=lambda x: x[2])

    uf = [i for i in range(n)]
    size = [1] * n

    def union(uf, size, a, b):
        if a == b:
            return

        uf[a] = b
        size[b] += size[a]

    def find(uf, x):
        if uf[x] == x:
            return x
        
        r = find(uf, uf[x])
        uf[x] = r
        return r

    for (a, b, _) in dist: 
        fa = find(uf, a)
        fb = find(uf, b)
        if fa != fb:
            union(uf, size, fa, fb)
            if size[fb] == n:
                result = boxes[a][0] * boxes[b][0]
                print(f"part2: {result}")
                return


if __name__ == "__main__":
    main()
