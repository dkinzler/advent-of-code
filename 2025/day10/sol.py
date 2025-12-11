from scipy.optimize import linprog

def main():
    part1()
    part2()

def parse_input(file):
    machines = []
    with open(file, "r") as f:
        for line in f:
            s = line.strip().split(" ")

            lights = s[0][1:-1]
            n_lights = len(lights)
            lights_bitmask = 0
            for i in range(n_lights):
                if lights[i] == "#":
                    lights_bitmask |= 1<<i

            buttons = []
            for x in s[1:-1]:
                buttons.append([int(a) for a in x[1:-1].split(",")])

            joltages = [int(a) for a in s[-1][1:-1].split(",")]

            machines.append((n_lights, lights_bitmask, buttons, joltages))

    return machines

def part1():
    file="input"
    #file="example"

    machines = parse_input(file)

    result = 0
    for (n_lights, lights, buttons_list, _) in machines:
        buttons = []
        for b in buttons_list:
            button_bitmask = 0
            for i in b:
                button_bitmask |= 1<<i
            buttons.append(button_bitmask)

        n_buttons = len(buttons)

        # we want the shortest path from 0 to lights by xoring
        # with values from buttons
        # since the lights are toggled withe very button press
        # pressing the same button twice does not make sense
        # x ^ x = 0
        # the order of the button presses doesn't matter
        # so we can press each button at most once
        # there are not a lot of buttons, so we can just
        # iterate over all the subsets and find the smallest one
        # that yields the number lights
        #
        # alternatively we could have modelled this as a graph problem
        # where we have to find the shortest path
        # nodes would be a specific state of the lights, and edges would be the
        # buttons
        
        min_pressed = 1<<60
        for z in range(1<<n_buttons):
            pressed = 0
            # the lights currently turned on/off, initially all are off
            state = 0
            for i in range(n_buttons):
                if z & (1<<i) > 0:
                    pressed += 1
                    state ^= buttons[i]
            
            if state == lights and pressed < min_pressed:
                min_pressed = pressed

        result += min_pressed

    print(f"part1: {result}")

def part2():
    file="input"
    #file="example"

    machines = parse_input(file)

    result = 0
    for (_, _, buttons, target_joltages) in machines:
        n_buttons = len(buttons)
        n_joltages = len(target_joltages)
        max_joltage = max(target_joltages)
        
        # now we have to press buttons to get the required joltage levels
        # the levels start out as all 0
        # a button like (0, 3, 4) means pressing it will increase the joltage levels
        # with index 0, 3 and 4 by 1
        # we have to find the minimum number of button presses to get the
        # target levels
        # 
        # for the given input we have at most 10 joltages with
        # the max target joltage level being 294
        #
        # one approach would be to model it as an ILP?
        # one variable x_i for the number of presses of each button
        # minmize the sum of x_i while reaching the target joltages
        #
        # we can't model it directly as a graph problem where
        # we have a node for each possible state like (j_1, j_2, ..., j_n)
        # there are way too many nodes
        # 
        # lets just do the ILP, why not

        # c^T * x is the linear objective function to minimize
        # = the total number of buttons pressed in our case
        c = [1] * n_buttons
        # a matrix to define equality constraints
        # a_eq * x = b_eq where b_eq will contain the target values
        # b_eq has size n_joltages
        # x has size n_buttons
        # so a_eq needs to be a n_joltages x n_buttons matrix
        # a_eq(r, c) = 1 if pressing button c increases joltage i by 1
        # i.e. column c corresponds to the joltages increased by button c
        a_eq = [[0 for j in range(n_buttons)] for i in range(n_joltages)]
        for j in range(n_buttons):
            button = buttons[j]
            for i in button:
                a_eq[i][j] = 1
        b_eq = target_joltages
        bounds = [(0, max_joltage) for i in range(n_buttons)]
        # number of button presses needs to be integer
        integrality = 1

        sol = linprog(c, A_eq=a_eq, b_eq=b_eq, bounds=bounds, integrality=integrality)
        # could also use the sol.fun value which is the optimal
        # value of the objective function, but it is float
        # so lets just avoid any float shenanigans
        for z in sol.x:
            result += z

    print(f"part2: {result}")

if __name__ == "__main__":
    main()
