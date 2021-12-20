def read_input(fname):
    lines = open(fname).readlines()
    starts = [ind for (ind, line) in enumerate(lines) if line.startswith("--- scanner")]
    scanners = []
    for start in starts:
        start += 1
        block = []
        while lines[start].strip() != "":
            block.append(list(map(int, lines[start].strip().split(","))))
            start += 1
        scanners.append(block)
    return scanners

scanners = read_input("input_test")

###############################################################################
# encode all possible transforms
X, Y, Z = 0, 1, 2
all_possible_transforms = [((1, X), (1, Y), (1, Z)),
                           ((1, X), (-1, Z), (1, Y)),
                           ((1, X), (-1, Y), (-1, Z)),
                           ((1, X), (1, Z), (-1, Y)),
                           ((-1, X), (-1, Y), (1, Z)),
                           ((-1, X), (-1, Z), (-1, Y)),
                           ((-1, X), (1, Y), (-1, Z)),
                           ((-1, X), (1, Z), (1, Y)),
                           ((-1, Y), (1, X), (1, Z)),
                           ((1, Z), (1, X), (1, Y)),
                           ((1, Y), (1, X), (-1, Z)),
                           ((-1, Z), (1, X), (-1, Y)),
                           ((1, Y), (-1, X), (1, Z)),
                           ((-1, Z), (-1, X), (1, Y)),
                           ((-1, Y), (-1, X), (-1, Z)),
                           ((1, Z), (-1, X), (-1, Y)),
                           ((1, Y), (1, Z), (1, X)),
                           ((-1, Z), (1, Y), (1, X)),
                           ((-1, Y), (-1, Z), (1, X)),
                           ((1, Z), (-1, Y), (1, X)),
                           ((1, Z), (1, Y), (-1, X)),
                           ((1, Y), (-1, Z), (-1, X)),
                           ((-1, Z), (-1, Y), (-1, X)),
                           ((-1, Y), (1, Z), (-1, X))]

def apply_transform(pts, transform):
    new_pts = []
    for pt in pts:
        new_pts.append([sign * pt[i] for (sign, i) in transform ])
    return new_pts

pts = [[1, 2, 3]]

apply_transform(pts, all_possible_transforms[1])

###############################################################################
# unit test for apply transform
def in_common(ref_points, points):
    s = set(tuple(item) for item in ref_points)
    common = [item for item in points if tuple(item) in s]
    return common


scanners = read_input("input_test2")

ref_points = scanners[0]

for pts in scanners[1:]:
    for transform in all_possible_transforms:
        rotated_pts = apply_transform(pts, transform)
        if len(in_common(ref_points, rotated_pts)) == 6:
            print("all close", transform)
            break


###############################################################################
# part 1 test input

def in_common_with_translation(ref_pts, rotated_pts):
    common = []
    for ref_pt in ref_pts:
        for rotated_pt in rotated_pts:
            translation = [rotated_pt[0] - ref_pt[0],
                           rotated_pt[1] - ref_pt[1],
                           rotated_pt[2] - ref_pt[2]]
            translated = [(pt[0] - translation[0],
                           pt[1] - translation[1],
                           pt[2] - translation[2]) for pt in rotated_pts]
            common_pts = in_common(ref_pts, translated)
            if len(common_pts) >= 12:
                return common_pts, translation
    return common, [[], [], []]


scanners = read_input("input_test")
def find_correspondences(scanners):
    translations = {}
    transformations = {}
    common_points = {}
    for i in range(len(scanners)):
        ref_pts = scanners[i]
        for j in range(len(scanners)):
            if i != j:
                pts = scanners[j]
                for transform in all_possible_transforms:
                    rotated_pts = apply_transform(pts, transform)
                    common, translation = in_common_with_translation(ref_pts, rotated_pts)
                    if len(common) > 0:
                        print(f"{i}->{j} found {len(common)} common items")
                        print(common)
                        print(translation)
                        translations[(i, j)] = translation
                        transformations[(i, j)] = transform
                        common_points[(i, j)] = common

    return translations, transformations, common_points

translations, transformations, common_points = find_correspondences(scanners)

###############################################################################
# how to translate from 1 to 0 ? by using the opposite of 1 to 0 !
# this must be the coordinate of scanner 1 as seen by scanner 0
def mult(vec, val):
    return [item * val for item in vec]

_0to1_0 = mult(translations[(0, 1)], -1)

_0to1_0
###############################################################################
# we can express the location of scanner 4 in referential 1
_1to4_1 = mult(translations[(1, 4)], -1)

_1to4_1
###############################################################################
# change coords
def add(v1, v2):
    return [a+b for (a,b) in zip(v1, v2)]

_1to4_0 = apply_transform([_1to4_1], transformations[(0, 1)])[0]

_0to4_0 = add(_1to4_0, mult(_0to1_0, 1))

_0to4_0
###############################################################################
# let’s express the coords of scanner 2 in ref 0
# first we need to express them in ref 4
_4to2_4 = mult(translations[(4, 2)], -1)

# transform to ref 1 vector
_4to2_1 = apply_transform([_4to2_4], transformations[(1, 4)])[0]

# transform to ref 0 vector
_4to2_0 = apply_transform([_4to2_1], transformations[(0, 1)])[0]

# add offset
_0_to2_0 = add(_4to2_0, _0to4_0)

_0_to2_0
###############################################################################
# let’s generalize the above
# let’s add in some graph stuff
import networkx as nx

def build_graph(transformations):
    G = nx.Graph()
    for key in transformations:
        G.add_node(key[0])
        G.add_node(key[1])

    for key in transformations:
        G.add_edge(key[0], key[1])
    return G

G = build_graph(transformations)

nx.shortest_path(G, 2, 0)

###############################################################################

def project_coords_to0(vec, reference, transformations, G):
    path = nx.shortest_path(G, reference, 0)
    if len(path) >= 2:
        for start, end in zip(path[:-1], path[1:]):
            vec = apply_transform([vec], transformations[(end, start)])[0]
    return vec

project_coords_to0(_1to4_1, 1, transformations, G)


###############################################################################
def compute_centers(G, scanners, translations, transformations):
    centers = {}
    node_order = sorted(range(1, len(scanners)), key=lambda node: len(nx.shortest_path(G, node, 0)))
    # node_order = [1, 4, 3, 2]
    for node in node_order:
        path = nx.shortest_path(G, node, 0)
        if len(path) == 2:
            # we already have a projection
            vec = mult(translations[(0, node)], -1)
        else:
            for key in centers:
                if (key, node) in translations:
                    break
            assert (key, node) in translations
            # print(node, key)
            vec = mult(translations[(key, node)], -1)
            # print(vec)
            vec = project_coords_to0(vec, key, transformations, G)
            # print(vec)
            vec = add(vec, centers[key])
            # print(vec)

        centers[node] = vec
    return centers

centers = compute_centers(G, scanners, translations, transformations)

###############################################################################
def compute_final_points(scanners, G, transformations, centers):
    final_points = set()
    for node in range(len(scanners)):
        pts = scanners[node]
        if node == 0:
            final_points.update([tuple(item) for item in pts])
        else:
            for pt in pts:
                pt = project_coords_to0(pt, node, transformations, G)
                pt = add(pt, centers[node])
                final_points.add(tuple(pt))
    return final_points

final_points = compute_final_points(scanners, G, transformations, centers)
len(final_points)
###############################################################################
# part 1 full solution

scanners = read_input("input")
translations, transformations, common_points = find_correspondences(scanners)
G = build_graph(transformations)
centers = compute_centers(G, scanners, translations, transformations)
final_points = compute_final_points(scanners, G, transformations, centers)
print(f"part 1: {len(final_points)}")

###############################################################################
def dist(v1, v2):
    return sum(abs(a - b) for a, b in zip(v1, v2))

assert dist((1105,-1205,1229), (-92,-2380,-20)) == 3621

max_dist = 0
centers[0] = [0, 0, 0]
for center1 in centers.values():
    for center2 in centers.values():
        max_dist = max(max_dist, dist(center1, center2))

print(f"part 2: {max_dist}")

###############################################################################

