def read_input(fname):
    lines = open(fname).readlines()
    algorithm = lines[0].strip()
    input_image = [line.strip() for line in lines[2:]]
    return algorithm, input_image


def make_first_grid(input_image):
    grid = {}
    imin = 0
    imax = len(input_image)
    jmin = 0
    jmax = len(input_image[0])
    for i in range(imin, imax):
        for j in range(jmin, jmax):
            grid[(i, j)] = input_image[i][j]
    return grid, imin, imax, jmin, jmax


def step(grid, algorithm, imin, imax, jmin, jmax, default="."):
    new_grid = {}
    for i in range(imin, imax):
        for j in range(jmin, jmax):
            bin = ""
            for di, dj in [(-1, -1), (-1, 0), (-1, +1),
                           (0, -1), (0, 0), (0, +1),
                           (+1, -1), (+1, 0), (+1, +1)]:
                bin += grid.get((i + di, j + dj), default)
            dec = int(bin.replace("#", "1").replace(".", "0"), 2)
            new_grid[(i, j)] = algorithm[dec]
    return new_grid


###############################################################################
# part 1
algorithm, input_image = read_input("input")
grid, imin, imax, jmin, jmax = make_first_grid(input_image)

inc = 1
imin, jmin = imin - inc, jmin - inc
imax, jmax = imax + inc, jmax + inc
grid = step(grid, algorithm, imin, imax, jmin, jmax, default=".")

imin, jmin = imin - inc, jmin - inc
imax, jmax = imax + inc, jmax + inc
grid = step(grid, algorithm, imin, imax, jmin, jmax, default="#")

print(f"part 1: {sum(1 for val in grid.values() if val == '#')}")

###############################################################################
# part 2
algorithm, input_image = read_input("input")
grid, imin, imax, jmin, jmax = make_first_grid(input_image)

inc = 1
imin, jmin = imin - inc, jmin - inc
imax, jmax = imax + inc, jmax + inc
default = "."

for _ in range(50):
    grid = step(grid, algorithm, imin, imax, jmin, jmax, default=default)
    default = {".": "#", "#": "."}[default]
    imin, jmin = imin - inc, jmin - inc
    imax, jmax = imax + inc, jmax + inc

print(f"part 2: {sum(1 for val in grid.values() if val == '#')}")
