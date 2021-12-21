# %cd /Users/kappamaki/Documents/workspace/advent_of_code2021/day15
import networkx as nx


def read_grid(fname):
    grid = []
    lines = open(fname).readlines()
    for line in lines:
        grid.append(list(map(int, line.strip())))
    return grid


def make_graph(grid):
    costs = {}
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            costs[(i, j)] = grid[i][j]

    # build graph
    G = nx.DiGraph()
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            G.add_node((i, j))
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            for di, dj in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
                ii, jj = i + di, j + dj
                if (ii, jj) in costs:
                    G.add_weighted_edges_from([[(i, j), (ii, jj), costs[(ii, jj)]]])
    return costs, G


grid = read_grid("input")
costs, G = make_graph(grid)
top_left = (0, 0)
bottom_right = (len(grid) - 1, len(grid[0]) - 1)
shortest = nx.dijkstra_path(G, top_left, bottom_right)
total = 0
for node in shortest[1:]:
    total += costs[node]
print(f"part 1:", total)

###############################################################################
# part 2: let’s modify the grid building
def inc(val, iters):
    for _ in range(iters):
        if val == 9:
            val =  1
        else:
            val += 1
    return val

def read_grid2(fname):
    grid = []
    lines = open(fname).readlines()
    for line in lines:
        start = list(map(int, line.strip()))
        this_line = []
        this_line.extend(start)
        for iter in range(1, 5):
            this_line.extend(map(lambda val: inc(val, iter), start))
        grid.append(this_line)
    new_lines = []
    for iter in range(1, 5):
        for line in grid:
            new_lines.append(list(map(lambda val: inc(val, iter), line)))
    grid.extend(new_lines)
    return grid

# grid = read_grid2("input_test")
# for line in grid:
#     print(line)

grid = read_grid2("input_test")
costs, G = make_graph(grid)
top_left = (0, 0)
bottom_right = (len(grid) - 1, len(grid[0]) - 1)
shortest = nx.dijkstra_path(G, top_left, bottom_right)
total = 0
for node in shortest[1:]:
    total += costs[node]
print(f"part 2:", total)

###############################################################################

