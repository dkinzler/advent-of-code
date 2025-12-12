def main():
    part1()

def parse_input(file):
    shapes = []
    trees = []
    with open(file, "r") as f:
        # both example and input have 6 shapes
        for i in range(6):
            f.readline()
            shape = []
            for j in range(3):
                shape.append(f.readline().strip())
            f.readline()
            shapes.append(shape)

        s = f.readline()
        while s != "":
            parts = s.strip().split(" ")
            dimensions = parts[0].rstrip(":").split("x")
            m = int(dimensions[0])
            n = int(dimensions[1])
            shape_counts = [int(z) for z in parts[1:]]
            trees.append((m, n, shape_counts))
            s = f.readline()

    return (shapes, trees)

def part1():
    file="input"
    #file="example"

    # ok this was a bait problem
    # in general solving this problem would be very hard if at all possible
    # for the given input sizes
    # you have > 100 shapes to fit, you can rotate and flip them etc.
    #
    # but all of the instances here are easy
    # as a heuristic we can look at the total number of cells used
    # by all the presents
    # e.g. #.#
    #      ###
    #      #.#
    # uses 7 cells, if you add all the cells together and that number
    # is already larger than the grid size m*n, then there can be no solution
    #
    # and that is the case here, either instances fail this test and the presents
    # can't fit under the tree or there are ~600 cells to spare in the grid
    # so they can probably fit somehow
    # -> and the number of those trees is indeed the solution

    shapes, trees = parse_input(file)
    shape_cell_counts = []
    for shape in shapes:
        count = 0
        for r in range(3):
            for c in range(3):
                if shape[r][c] == "#":
                    count += 1
        shape_cell_counts.append(count)

    result = 0
    for (m, n, counts) in trees:
        z = 0
        for i in range(6):
            z += counts[i]*shape_cell_counts[i]

        if z <= m*n:
            result += 1

    print(f"part1: {result}")

if __name__ == "__main__":
    main()
