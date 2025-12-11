def main():
    part1()
    part2()

def parse_input(file):
    tiles = []
    with open(file, "r") as f:
        for line in f:
            s = line.strip().split(",")
            # swapped since in the example the first number
            # is column and second number is row
            tiles.append((int(s[1]), int(s[0])))

    return tiles

def part1():
    file="input"
    #file="example"

    tiles = parse_input(file)
    n = len(tiles)

    max_area = 0
    for i in range(n-1):
        xr, xc = tiles[i]
        for j in range(n):
            yr, yc = tiles[j]
            a = (abs(xr-yr)+1) * (abs(xc-yc)+1)
            if a > max_area:
                max_area = a

    print(f"part1: {max_area}")

def part2():
    # ok this one is gonna be interesting
    # the tile coordinates are roughly in the range 0-100k
    # so we definitely can't just build the entire grid
    #
    # the red tiles could form the outline of e.g. a "horseshoe"
    # it does not have to be a clean shape like in the example
    # that means e.g. in some rows we could have multiple disconnected
    # ranges of tiles that are red/green

    file="input"
    #file="example"
    tiles = parse_input(file)
    n = len(tiles)

    max_r = 0
    max_c = 0
    min_r = 1 << 60
    min_c = 1 << 60
    for (xr, xc) in tiles:
        if xr > max_r:
            max_r = xr
        if xr < min_r:
            min_r = xr
        if xc > max_c:
            max_c = xc
        if xc < min_c:
            min_c = xc

    print(f"range r: {min_r}-{max_r}")
    print(f"range c: {min_c}-{max_c}")
    
    # start at the tile at the top left (there must be a single unique such tile)
    # and determine for each vertical/horizontal line what side is inside
    # or outside the shape
    # e.g. consider any top-most horizontal line, the outside must be
    # in the top direction = lower row coordinate
    #
    # when we know for each line which direction is inside/outside,
    # we can compute the inside of the shape as a set of rectangles

    topleft_r, topleft_c = tiles[0]
    topleft_i = 0
    for i in range(1, n):
        xr, xc = tiles[i]
        if xr < topleft_r:
            topleft_r, topleft_c = xr, xc
            topleft_i = i
        elif xr == topleft_r and xc < topleft_c:
            topleft_r, topleft_c = xr, xc
            topleft_i = i

    print(f"top left: {topleft_r},{topleft_c}")

    HORIZONTAL="HORIZONTAL"
    VERTICAL="VERTICAL"
    TOP_OUTSIDE="TOP_OUTSIDE"
    BOTTOM_OUTSIDE="BOTTOM_OUTSIDE"
    LEFT_OUTSIDE="LEFT_OUTSIDE"
    RIGHT_OUTSIDE="RIGHT_OUTSIDE"
    TOP="TOP"
    BOTTOM="BOTTOM"
    LEFT="LEFT"
    RIGHT="RIGHT"

    #returns (start, end), is_horizontal
    def to_line(xr, xc, yr, yc):
        if xr == yr:
            return (xr, min(xc, yc), max(xc, yc)), True
        else:
            return (xc, min(xr, yr), max(xr, yr)), False
    
    horz_edges = []
    vert_edges = []

    curr = topleft_i
    last_dir = None
    last_outside = None
    # whether the last tile was at the top/bottom for a vertical edge
    # or left/right for a horizontal edge
    last_pos = None
    for _ in range(n):
        next = (curr+1)%n
        curr_r, curr_c = tiles[curr]
        next_r, next_c = tiles[next]
        line, is_horizontal = to_line(curr_r, curr_c, next_r, next_c)
        if last_dir is None:
            # we are at the beginning
            # next point must be either to the right or down
            if is_horizontal:
                dir = HORIZONTAL
                outside = TOP_OUTSIDE
                horz_edges.append((dir, line[0], line[1], line[2], outside))
                last_dir = dir
                last_outside = outside
                last_pos = RIGHT
            else:
                dir = VERTICAL
                outside = LEFT_OUTSIDE
                vert_edges.append((dir, line[0], line[1], line[2], outside))
                last_dir = dir
                last_outside = outside
                last_pos = BOTTOM
        else:
            # we assume that edges are alternating between horizontal and vertical
            # that doesn't have to be the case in general but is the case for our
            # input
            if is_horizontal:
                if last_dir == HORIZONTAL:
                    print("non-alternating edges detected")

                dir = HORIZONTAL
                outside = None
                pos = RIGHT if next_c > curr_c else LEFT
                # need to know whether the corner of these two edges
                # is at the top of the vertical edge or the bottom
                if last_pos == TOP:
                    if pos == RIGHT:
                        if last_outside == LEFT_OUTSIDE:
                            outside = TOP_OUTSIDE
                        else:
                            outside = BOTTOM_OUTSIDE
                    else:
                        if last_outside == LEFT_OUTSIDE:
                            outside = BOTTOM_OUTSIDE 
                        else:
                            outside = TOP_OUTSIDE
                else:
                    if pos == RIGHT:
                        if last_outside == LEFT_OUTSIDE:
                            outside = BOTTOM_OUTSIDE
                        else:
                            outside = TOP_OUTSIDE
                    else:
                        if last_outside == LEFT_OUTSIDE:
                            outside = TOP_OUTSIDE 
                        else:
                            outside = BOTTOM_OUTSIDE

                horz_edges.append((dir, line[0], line[1], line[2], outside))
                last_dir = dir
                last_outside = outside
                last_pos = pos
            else:
                if last_dir == VERTICAL:
                    print("non-alternating edges detected")

                dir = VERTICAL
                outside = None
                pos = BOTTOM if next_r > curr_r else TOP
                # need to know whether the corner of these two edges
                # is at the left of the horizontal edge or the right
                if last_pos == LEFT:
                    if pos == TOP:
                        if last_outside == BOTTOM_OUTSIDE:
                            outside = LEFT_OUTSIDE
                        else:
                            outside = RIGHT_OUTSIDE
                    else:
                        if last_outside == BOTTOM_OUTSIDE:
                            outside = RIGHT_OUTSIDE 
                        else:
                            outside = LEFT_OUTSIDE
                else:
                    if pos == TOP:
                        if last_outside == BOTTOM_OUTSIDE:
                            outside = RIGHT_OUTSIDE
                        else:
                            outside = LEFT_OUTSIDE
                    else:
                        if last_outside == BOTTOM_OUTSIDE:
                            outside = LEFT_OUTSIDE 
                        else:
                            outside = RIGHT_OUTSIDE

                vert_edges.append((dir, line[0], line[1], line[2], outside))
                last_dir = dir
                last_outside = outside
                last_pos = pos

        curr = next

    horz_edges.sort(key=lambda x: (x[1], x[2]))

    # print("horizontal edges")
    # for e in horz_edges:
    #     print(e)
    #
    # print("\nvertical edges")
    # for e in vert_edges:
    #     print(e)

    # rectangles that cover the shape given by
    # top left and bottom right corners
    rectangles = []

    for (_, r, start, end, outside) in horz_edges:
        if outside == TOP_OUTSIDE:
            # find all the closest (with respect to rows)
            # horizontal edges with BOTTOM_OUTSIDE
            # all the space in between will be inside the shape
            parts = [(start, end)]
            pi = 0
            while pi < len(parts):
                curr_start, curr_end = parts[pi]
                pi += 1
                for (_, nr, next_start, next_end, next_outside) in horz_edges:
                    if next_outside == BOTTOM_OUTSIDE and nr > r:
                        if not (curr_end < next_start or next_end < curr_start):
                            z_start = max(curr_start, next_start)
                            z_end = min(curr_end, next_end)

                            rectangles.append((r, z_start, nr, z_end))

                            if curr_start < z_start:
                                parts.append((curr_start, z_start-1))
                            if z_end < curr_end:
                                parts.append((z_end+1, curr_end))

                            break

    # for (xr, xc, yr, yc) in rectangles:
    #     print(f"{xr},{xc}  {yr},{yc}")


    # some of the vertical edges might be missed in the
    # above calculations
    for (_, c, start, end, _) in vert_edges:
        rectangles.append((start, c, end, c))

    def is_covered_horiz(line, rectangles):
        r, start, end = line

        parts = []
        for (xr, xc, yr, yc) in rectangles:
            if r >= xr and r <= yr:
                if not (end < xc or yc < start):
                    parts.append((max(xc, start), min(yc, end)))

        if len(parts) == 0:
            return False

        parts.sort()
        curr_start, curr_end = parts[0]
        valid = True
        for (x_start, x_end) in parts[1:]:
            # we know here curr_start <= x_start 
            if x_start <= curr_end+1:
                curr_end = max(curr_end, x_end)
            else:
                valid = False
                break

        if curr_start > start or curr_end < end:
            valid = False

        return valid

    def is_covered_vert(line, rectangles):
        c, start, end = line

        parts = []
        for (xr, xc, yr, yc) in rectangles:
            if c >= xc and c <= yc:
                if not (end < xr or yr < start):
                    parts.append((max(xr, start), min(yr, end)))

        if len(parts) == 0:
            return False

        parts.sort()
        curr_start, curr_end = parts[0]
        valid = True
        for (x_start, x_end) in parts[1:]:
            # we know here curr_start <= x_start 
            if x_start <= curr_end+1:
                curr_end = max(curr_end, x_end)
            else:
                valid = False
                break

        if curr_start > start or curr_end < end:
            valid = False

        return valid


    # returns True if the given line is completely
    # covered by the set of rectangles
    def is_covered(line, isHorizontal, rectangles):
        if isHorizontal:
            return is_covered_horiz(line, rectangles)
        else:
            return is_covered_vert(line, rectangles)


    max_area = 0
    # iterate over every pair of red tiles
    # compute the other two corners of the rectangle formed by the two tiles
    # to check if the rectangle lies completely in the shape
    # we don't have to actually check all of it
    # we can just check that the 4 lines bounding the rectangle
    # are all inside the shape, then the rest must be inside as well
    for i in range(n-1):
        xr, xc = tiles[i]
        for j in range(i+1, n):
            yr, yc = tiles[j]

            if xr == yr or xc == yc:
                continue

            corner1_r, corner1_c = xr, yc
            corner2_r, corner2_c = yr, xc

            #print()
            #print(f"{xr},{xc}  {yr},{yc}   {zr},{zc}")
            #print(f"corner {corner_r},{corner_c}")

            valid = True

            line, isHorizontal = to_line(xr, xc, corner1_r, corner1_c)
            if not is_covered(line, isHorizontal, rectangles):
                valid = False

            if valid:
                line, isHorizontal = to_line(xr, xc, corner2_r, corner2_c)
                if not is_covered(line, isHorizontal, rectangles):
                    valid = False

            if valid:
                line, isHorizontal = to_line(yr, yc, corner1_r, corner1_c)
                if not is_covered(line, isHorizontal, rectangles):
                    valid = False

            if valid:
                line, isHorizontal = to_line(yr, yc, corner2_r, corner2_c)
                if not is_covered(line, isHorizontal, rectangles):
                    valid = False

            if valid:
                a = (abs(xr-yr)+1) * (abs(xc-yc)+1)
                if a > max_area:
                    max_area = a

    print(f"part2: {max_area}")

if __name__ == "__main__":
    main()
