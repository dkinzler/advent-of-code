def main():
    part1()
    part2()

def parse_input(file):
    banks = []
    with open(file, "r") as f:
        for line in f:
            bank = []
            s = line.strip()
            for d in s:
                bank.append(int(d))

            banks.append(bank)

    return banks

def find_max_digit(x):
    m = 0
    mi = -1 
    for i in range(len(x)):
        if x[i] > m:
            m = x[i]
            mi = i

    return (m, mi)

def part1():
    file="input"
    #file="example"
    banks = parse_input(file)
    result = 0
    for bank in banks:
        (first_digit, i) = find_max_digit(bank[0:len(bank)-1])
        (second_digit, _) = find_max_digit(bank[i+1:len(bank)])
        result += first_digit*10+second_digit

    print(f"part1: {result}")

def part2():
    file="input"
    #file="example"
    banks = parse_input(file)
    result = 0

    # this is seems like a classic DP problem, one for each bank
    # let d(i, j) = larget value possible by using j digits from those
    #               with index <= i
    #
    # d(i, 0) = 0 for all i
    # d(0, j) = 0 for j > 1
    # d(i, 1) = largest digit among those with index <= i
    # d(i, j) = 0 if j > i+1
    #           else max(d(i-1, j), d(i-1, j-1)*10 + bank[i])
    #

    # all the banks have the same number of elements
    # and j can be max 12
    n = len(banks[0])
    m = 12
    d=[[0 for j in range(m+1)] for i in range(n)]

    for bank in banks:
        d[0][1] = bank[0]
        for i in range(1, n):
            d[i][1] = max(d[i-1][1], bank[i])

        for j in range(2, m+1):
            for i in range(1, n):
                if j > i+1:
                    d[i][j] = 0
                else:
                    d[i][j] = max(d[i-1][j], d[i-1][j-1]*10 + bank[i])

        result += d[n-1][m]

    print(f"part1: {result}")

if __name__ == "__main__":
    main()
