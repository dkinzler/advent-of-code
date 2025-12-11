def main():
    part1()
    part2()

def parse_input(file):
    edges = []
    name_to_index = {}
    next_index = 0

    def get_index(name):
        nonlocal name_to_index
        nonlocal next_index
        nonlocal edges

        if not name in name_to_index:
            name_to_index[name] = next_index
            next_index += 1
            edges.append([])

        return name_to_index[name]

    with open(file, "r") as f:
        for line in f:
            s = line.strip().split(" ")

            sa = s[0].rstrip(":")
            a = get_index(sa)
            for sb in s[1:]:
                b = get_index(sb)
                edges[a].append(b)

    return (edges, name_to_index)

def part1():
    file="input"
    #file="example"

    edges, name_to_index = parse_input(file)
    s = name_to_index["you"]
    e = name_to_index["out"]
    n = len(edges)

    # we want to find the number of paths from s to e in the given
    # directed graph
    # the graph should be acyclic, because otherwise there would be
    # an inifnite number of different paths
    # any path can use the same node at most once, otherwise there would be a cycle
    # can we just do a dfs?

    # the graph is directed and acyclic but not necessarily a tree
    # we might have to visit the same node multiple times
    # memoize the nodes values already computed
    count = [-1 for i in range(n)]
    def dfs(node):
        nonlocal count

        if node == e:
            return 1
        if count[node] != -1:
            return count[node]

        v = 0
        for nb in edges[node]:
            v += dfs(nb)
        
        count[node] = v
        return v

    result = dfs(s)
    print(f"part1: {result}")

def part2():
    file="input"
    #file="example2"

    edges, name_to_index = parse_input(file)
    n = len(edges)

    # now we need to find the number of paths from svr to out
    # that use both the nodes dac and fft
    # maybe we can find the number of paths from dac to fft
    # and then number of paths from svr to dac that do not use fft
    # and then the number of paths from fft to out that do not use
    # dac
    # and then vice versa where we first visit fft then dac
    # do we always need to block both nodes that should be unused?
    # we can, why not, not a big overhead
    
    # computes number of paths from start to end that
    # do not use any of the nodes in blacklist
    def n_paths(start, end, blacklist):
        count = [-1 for i in range(n)]
        for node in blacklist:
            count[node] = 0

        def dfs(node):
            nonlocal count

            if node == end:
                return 1

            if count[node] != -1:
                return count[node]

            v = 0
            for nb in edges[node]:
                v += dfs(nb)
            
            count[node] = v
            return v

        return dfs(start)

    svr = name_to_index["svr"]
    out = name_to_index["out"]
    dac = name_to_index["dac"]
    fft = name_to_index["fft"]

    result = 0

    a = n_paths(svr, dac, [fft, out])
    b = n_paths(dac, fft, [svr, out])
    c = n_paths(fft, out, [svr, dac])
    result += a*b*c

    a = n_paths(svr, fft, [dac, out])
    b = n_paths(fft, dac, [svr, out])
    c = n_paths(dac, out, [svr, fft])
    result += a*b*c

    print(f"part2: {result}")

if __name__ == "__main__":
    main()
