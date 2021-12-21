# %cd /Users/kappamaki/Documents/workspace/advent_of_code2021/day12

def read_input(fname):
    lines = open(fname).readlines()
    maze = {}
    for line in lines:
        left, right = line.strip().split("-")
        existing = maze.get(left, list())
        existing.append(right)
        maze[left] = existing
        if left != "start" and right != "end":
            existing = maze.get(right, list())
            existing.append(left)
            maze[right] = existing
    return maze

maze = read_input("input")

routes = [("start",)]

def is_small(cave):
    return cave.upper() != cave


while True:
    before = len(routes)
    new_routes = []
    for route in routes:
        last = route[-1]
        if last != "end":
            visited = set(route)
            connections = maze[last]
            for node in connections:
                if is_small(node):
                    if node not in visited:
                        new_routes.append(route + (node,))
                else:
                    new_routes.append(route + (node,))
        else:
            new_routes.append(route)
    routes = new_routes
    if len(new_routes) == before:
        break

print(f"part 1:", len(routes))

###############################################################################
# part 2 a single small cave may be visited twice

def solve(routes):
    while True:
        before = len(routes)
        new_routes = []
        for route in routes:
            last = route[-1]
            if last != "end":
                visited = set(route)
                connections = maze[last]
                for node in connections:
                    if is_small(node):
                        if node not in visited:
                            new_routes.append(route + (node,))
                    else:
                        new_routes.append(route + (node,))
            else:
                new_routes.append(route)
        routes = new_routes
        if len(new_routes) == before:
            break
    return routes


maze = read_input("input")
routes = [("start",)]
routes = solve(routes)
print(f"part 1:", len(routes))

###############################################################################
from collections import Counter

def solve2(routes, small_cave_twice):
    while True:
        before = len(routes)
        new_routes = []
        for route in routes:
            last = route[-1]
            if last != "end":
                visited = Counter(route)
                connections = maze[last]
                for node in connections:
                    if is_small(node):
                        if node == small_cave_twice:
                            if visited.get(node, 0) <= 1:
                                new_routes.append(route + (node,))
                        elif node not in visited:
                            new_routes.append(route + (node,))
                    else:
                        new_routes.append(route + (node,))
            else:
                new_routes.append(route)
        routes = new_routes
        if len(new_routes) == before:
            break
    return routes

maze = read_input("input")
small_caves = list(cave for cave in maze if is_small(cave) if cave != "start" if cave != "end")

all_routes = set()
for small_cave in small_caves:
    routes = [("start",)]
    routes = solve2(routes, small_cave)
    all_routes.update(routes)

print(f"part 2:", len(all_routes))

###############################################################################

