from random import randint

def choicename(int):
    if int == 1:
        return "rock"
    elif int == 2:
        return "paper"
    elif int == 3:
        return "scissors"

def guess():
    print("Let's play rock paper scissors!")
    guess = int(input("play 1 for rock, 2 for paper, and 3 for scissors. "))
    while guess != 1 and guess != 2 and guess != 3:
        guess = int(input("Either pick 1, 2, or 3. "))
    print("You chose %s!" % choicename(guess))
    return guess

def aiguess():
    opponent = randint(1,3)
    print("I picked %s." % choicename(opponent))
    return opponent

def game():
    choice = guess()
    opchoice = aiguess()
    if opchoice == choice - 1 or opchoice == choice + 2:
        print (f"{choicename(choice)} beats {choicename(opchoice)}! You win!")
    elif opchoice == choice + 1 or opchoice == choice - 2:
        print (f"{choicename(opchoice).title()} beats {choicename(choice)}! You lost!")
    elif opchoice == choice:
        print("We tied!")

game()
