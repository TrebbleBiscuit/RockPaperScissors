from random import randint


def counter_most(rps: "RockPaperScissors") -> int:
    """Counter the opponent's most frequent choice"""
    most_chosen = max(rps.history, key=rps.history.get)
    return rps.counter_choice(most_chosen)


def counter_last(rps: "RockPaperScissors") -> int:
    """Counter the opponent's last choice"""
    return rps.counter_choice(rps.last_choice)


def random(rps: "RockPaperScissors") -> int:
    """Return a random choice"""
    return randint(1, 3)


def zhejiang(rps: "RockPaperScissors") -> int:
    """If you lose, switch to beat what the opponent just played.
    If you win, play what the opponent just played.

    https://arxiv.org/pdf/1404.5199v1.pdf
    """
    if rps.last_result == 0:  # tie
        return random(rps)
    elif rps.last_result == 1:  # human win
        return rps.counter_choice(rps.ai_last_choice)
    elif rps.last_result == 2:  # human loss
        return rps.ai_last_choice
