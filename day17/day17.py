# %cd /Users/kappamaki/Documents/workspace/advent_of_code2021/day17
def read_input(fname):
    line = open(fname).readline().strip().split("target area: ")[1]
    x, y = line.split(", ")
    x = list(map(int, x.split("x=")[1].split("..")))
    y = list(map(int, y.split("y=")[1].split("..")))
    return x, y


xlim, ylim = read_input("input")

###############################################################################
from math import copysign
from math import inf


def step(pos, vel):
    pos += vel
    velimag = vel.imag - 1
    if abs(vel.real) > 0:
        velreal = copysign(1, vel.real) * (abs(vel.real) - 1)
    else:
        velreal = vel.real
    return pos, velreal + 1j * velimag


def min_xvel(dist):
    xvel = 1
    while True:
        if xvel * (xvel + 1) / 2 > dist:
            return xvel
        else:
            xvel += 1


def step_until_out(pos, vel, xlim, ylim, verbose=False):
    max_height = pos.imag
    reached_target = (xlim[0] <= pos.real <= xlim[1]) & (ylim[0] <= pos.imag <= ylim[1])
    while pos.imag > ylim[0]:
        pos, vel = step(pos, vel)
        max_height = max(max_height, pos.imag)
        if (xlim[0] <= pos.real <= xlim[1]) & (ylim[0] <= pos.imag <= ylim[1]):
            reached_target = True
        if verbose:
            print(f"just stepped to pos: {pos}, vel: {vel}, reached: {reached_target}")
    return max_height, reached_target


maxi = - inf
count = 0
for xvel in range(min_xvel(xlim[0]), xlim[1] + 1):
    for yvel in range(-1000, 1000):
        pos = 0 + 0j
        vel = xvel + 1j * yvel
        max_height, reached_target = step_until_out(pos, vel, xlim, ylim)
        if reached_target:
            maxi = max(maxi, max_height)
            count += 1

print(f"part 1: {maxi}")
print(f"part 2: {count}")
# 710 is too low
###############################################################################
