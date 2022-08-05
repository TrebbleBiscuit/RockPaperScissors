from random import choice
import ai_strats


class RockPaperScissors:
    def __init__(self):
        self.games = {
            0: 0,  # ties
            1: 0,  # wins
            2: 0,  # losses
        }
        self.last_result = 0
        self.history = {
            1: 0,  # rock
            2: 0,  # paper
            3: 0,  # scissors
        }
        self.last_choice = 1
        self.ai_last_choice = 1

    def play_game(self, choice = None):
        if choice not in self.history:
            choice = self.guess()
        self.last_choice = choice
        self.history[choice] += 1
        opchoice = self.aiguess()
        result = self.evaluate_game(choice, opchoice)
        result_output = {
            0: "We tied!",
            1: f"{RockPaperScissors.choicename(choice)} beats {RockPaperScissors.choicename(opchoice)}! You win!",
            2: f"{RockPaperScissors.choicename(opchoice).title()} beats {RockPaperScissors.choicename(choice)}! You lost!",
        }
        print(result_output[result])

    @staticmethod
    def choicename(choice: int) -> str:
        """Given a choice number, return its name"""
        choice = int(choice)
        if choice == 1:
            return "rock"
        elif choice == 2:
            return "paper"
        elif choice == 3:
            return "scissors"

    def guess(self) -> int:
        """Prompt player to select input, returns their choice"""
        guess = input("play 1 for rock, 2 for paper, and 3 for scissors. ")
        while guess not in ["1", "2", "3"]:
            guess = input("Either pick 1, 2, or 3. ")
        guess = int(guess)
        print(f"You chose {RockPaperScissors.choicename(guess)}!")
        return guess

    def aiguess(self) -> int:
        """Generate and return an AI guess"""
        strat = choice(
            [ai_strats.counter_most, ai_strats.counter_last, ai_strats.random, ai_strats.zhejiang]
        )
        opponent = strat(self)
        print("I picked %s." % RockPaperScissors.choicename(opponent))
        self.ai_last_choice = opponent
        return opponent

    @staticmethod
    def counter_choice(choice: int) -> int:
        """Given a choice, return the choice that would counter it

        Args:
            choice (int)

        Returns:
            int: counter choice
        """
        if choice == 3:
            return choice - 2
        else:
            return choice + 1

    def evaluate_game(self, guess_1: int, guess_2: int) -> int:
        """Evaluate the winner of a game given the players guesses

        Args:
            guess_1 (int): Function returns True if this player wins
            guess_2 (int): Functions returns False if this player wins

        Returns:
            int: The number of the player who won (zero for ties)
        """
        if guess_2 == guess_1:
            g = 0
        elif guess_2 == guess_1 - 1 or guess_2 == guess_1 + 2:
            g = 1
        elif guess_2 == guess_1 + 1 or guess_2 == guess_1 - 2:
            g = 2
        self.last_result = g
        self.games[g] += 1
        return g
    
    def get_record(self) -> str:
        """Return Win-Loss-Ties of the format 0-0-0"""
        return f"{self.games[1]}-{self.games[2]}-{self.games[0]}"
    
    def get_wl_percent(self) -> int:
        return int(100*self.games[1]/(self.games[1]+self.games[2] or 1))


if __name__ == "__main__":
    print("Let's play rock paper scissors!")
    rps = RockPaperScissors()
    while True:
        rps.play_game()
        print(
            f"Total record (W-L-T): {rps.get_record()} - ({rps.get_wl_percent()}% W/L)"
        )
