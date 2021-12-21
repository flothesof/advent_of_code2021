def DeterministicDie():
    count = 0
    while True:
        yield (count % 1000) + 1, count + 1
        count += 1


def play(player1, player2):
    die = DeterministicDie()
    players = [player1, player2]
    active = 0
    while True:
        player = players[active]
        sum_rolls = 0
        for i in range(3):
            side, rolls = die.__next__()
            sum_rolls += side
        new_pos = (player[0] - 1 + sum_rolls) % 10 + 1
        new_score = player[1] + new_pos
        players[active] = (new_pos, new_score)
        # print(f"player {active + 1} moves to {new_pos} for a total score of {new_score}")
        if new_score >= 1000:
            # print(f"player {active + 1} wins")
            score = players[(active + 1) % 2][1] * rolls
            return score
        active = (active + 1) % 2


###############################################################################
# test input
player1 = (4, 0)
player2 = (8, 0)
assert play(player1, player2) == 739785

###############################################################################
# my input
player1 = (8, 0)
player2 = (6, 0)
print(f"part 1: {play(player1, player2)}")


###############################################################################

def wins(player1, player2, active, memory):
    if (player1, player2, active) in memory:
        val = memory[(player1, player2, active)]
        return val[0], val[1], memory
    else:
        wins_player1, wins_player2 = 0, 0
        players = [player1, player2]
        player = players[active]
        for i in range(3):
            for j in range(3):
                for k in range(3):
                    sum_rolls = i + j + k + 3
                    new_pos = (player[0] - 1 + sum_rolls) % 10 + 1
                    new_score = player[1] + new_pos
                    new_player = (new_pos, new_score)
                    if new_score >= 21:
                        # print(f"player {active + 1} wins")
                        if active == 0:
                            wins_player1 += 1
                        else:
                            wins_player2 += 1
                    else:
                        players[active] = new_player
                        wp1, wp2, mem = wins(players[0], players[1], (active + 1) % 2, memory)
                        wins_player1 += wp1
                        wins_player2 += wp2
                        memory.update(mem)
        memory[(player1, player2, active)] = wins_player1, wins_player2
        return wins_player1, wins_player2, memory


player1 = (4, 0)  # pos, score, number of players at that location
player2 = (8, 0)
memory = {}
out = wins(player1, player2, 0, memory)
print("part 2 test input:", out[0], out[1])

###############################################################################
# part 2

player1 = (8, 0)  # pos, score, number of players at that location
player2 = (6, 0)
memory = {}
out = wins(player1, player2, 0, memory)
print("part 2:", max(out[0], out[1]))

###############################################################################
