# %cd /Users/kappamaki/Documents/workspace/advent_of_code2021/day14
###############################################################################
# part 1 naive version
from collections import Counter


def read_input(fname):
    lines = open(fname).readlines()
    polymer = lines[0].strip()
    insertions = {}
    for line in lines[2:]:
        if len(line.strip()) > 0:
            left, right = line.strip().split(" -> ")
            insertions[left] = right


    return polymer, insertions


def step(polymer, rules):
    new_polymer = ""
    for i in range(len(polymer) - 1):
        doublet = polymer[i:i + 2]
        new_polymer += doublet[0] + rules[doublet]
    new_polymer += doublet[1]
    return new_polymer


polymer, insertions = read_input("input")
for i in range(10):
    polymer = step(polymer, insertions)
    # print(f"step {i + 1}, len: {len(polymer)}")

c = Counter(polymer)
items = list(c.items())
least, most = sorted(items, key=lambda t: t[1])[0], sorted(items, key=lambda t: t[1], reverse=True)[0]
print(f"part 1: {most[1] - least[1]}")


###############################################################################
# part 2 efficient algorithm

def counts(doublet, steps, memory):
    if steps == 0:
        return Counter(doublet[1:]), memory
    else:
        if (doublet, steps) in memory:
            return memory[(doublet, steps)], memory
        else:
            c = Counter()
            triplet = doublet[0] + insertions[doublet] + doublet[1]
            d1 = triplet[:2]
            d2 = triplet[1:]
            c1, memory = counts(d1, steps - 1, memory)
            c2, memory = counts(d2, steps - 1, memory)
            c.update(c1)
            c.update(c2)
            memory[(doublet, steps)] = c
            return c, memory


def compute(polymer, iters):
    memory = {}
    totals = Counter()
    for i in range(len(polymer) - 1):
        doublet = polymer[i:i + 2]
        c, memory = counts(doublet, iters, memory)
        totals.update(c)
    # since left hand side is not accounted for, add it at the end
    totals[polymer[0]] += 1
    return totals


polymer, insertions = read_input("input")
totals = compute(polymer, iters=10)
least, most = sorted(totals.items(), key=lambda t: t[1])[0], sorted(totals.items(), key=lambda t: t[1], reverse=True)[0]
print(f"part 1: {most[1] - least[1]}")

###############################################################################
totals = compute(polymer, iters=40)
least, most = sorted(totals.items(), key=lambda t: t[1])[0], sorted(totals.items(), key=lambda t: t[1], reverse=True)[0]
print(f"part 2: {most[1] - least[1]}")

###############################################################################
