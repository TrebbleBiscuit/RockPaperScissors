from src.main import RockPaperScissors

def test_rps():
    rps = RockPaperScissors()
    for x in range(10):
        rps.play_game(choice=1)
        rps.play_game(choice=2)
        rps.play_game(choice=3)
    print(
        f"Total record (W-L-T): {rps.get_record()} - ({rps.get_wl_percent()}% W/L)"
    )
