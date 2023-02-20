use rand::Rng;
use std::env;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p1_strat = if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            "random" => Strats::Random,
            "zhejiang" => Strats::Zhejiang,
            _ => {
                println!("Provided argument not recognized; playing as a human");
                println!("Specify 'random' or 'human' to select p1 strat");
                Strats::Human
            }
        }
    } else {
        println!("Playing human vs AI");
        Strats::Human
    };
    play_rps_forever(p1_strat, Strats::Zhejiang)
}

#[derive(PartialEq)]
enum Strats {
    Human,
    Zhejiang,
    Random,
}
// todo: specify ome of these in arg?

struct RPSGame {
    total_ties: i32,
    total_losses: i32,
    total_wins: i32,
    last_p1_guess: i32,
    last_p2_guess: i32,
    last_result: i32,
    p1_strat: Strats,
    p2_strat: Strats,
}

impl RPSGame {
    fn rps_round(&mut self) -> i32 {
        // return 1 for player 1 victory, 0 for tie, -1 for defeat
        let p1_guess = self.guess_from_strat(&self.p1_strat, self.last_p2_guess, -self.last_result);
        let p2_guess = self.guess_from_strat(&self.p2_strat, self.last_p1_guess, self.last_result);
        // let ai_guess_name = choice_name(ai_guess);
        // let guess_name = choice_name(human_guess);
        // println!("AI chose {ai_guess_name}");
        // println!("You chose {guess_name}");

        let result: i32;
        match p1_guess - p2_guess {
            0 => result = 0,
            -2 => result = 1,
            -1 => result = -1,
            1 => result = 1,
            2 => result = -1,
            _ => panic!("Invalid guess comparison"),
        }
        match result {
            1 => self.total_wins += 1,
            0 => self.total_ties += 1,
            -1 => self.total_losses += 1,
            _ => panic!("RPS returned invalid result"),
        }
        self.last_result = result;
        self.last_p1_guess = p1_guess;
        self.last_p2_guess = p2_guess;
        return result;
    }

    fn guess_from_strat(&self, strat: &Strats, last_opp_guess: i32, last_result: i32) -> i32 {
        match strat {
            Strats::Zhejiang => {
                // add some randomness to zhejiang so it's not so predictable
                if rand::thread_rng().gen_range(1..=100) <= 30 {
                    zhejiang(last_opp_guess, last_result)
                } else {
                    rand::thread_rng().gen_range(1..=3)
                }
            }
            Strats::Human => solicit_human_guess(),
            Strats::Random => rand::thread_rng().gen_range(1..=3),
        }
    }
}

fn make_rps_game(p1_strat: Strats, p2_strat: Strats) -> RPSGame {
    RPSGame {
        total_ties: 0,
        total_losses: 0,
        total_wins: 0,
        last_p1_guess: rand::thread_rng().gen_range(1..=3),
        last_p2_guess: rand::thread_rng().gen_range(1..=3),
        last_result: 0,
        p1_strat,
        p2_strat,
    }
}

fn play_rps_forever(p1_strat: Strats, p2_strat: Strats) {
    println!("Let's play rock paper scissors!");
    let mut game = make_rps_game(p1_strat, p2_strat);
    loop {
        let result = game.rps_round();
        if { game.p1_strat == Strats::Human } || { game.p2_strat == Strats::Human } {
            // more verbose if there's a human
            println!();
            println!("AI chose {}", choice_name(game.last_p2_guess));
            println!("You chose {}", choice_name(game.last_p1_guess));
            match result {
                1 => println!("Player 1 wins!"),
                0 => println!("It's a tie!"),
                -1 => println!("Player 2 wins!"),
                _ => panic!("RPS returned invalid result"),
            }
        }
        println!(
            "Record: {}-{}-{}",
            game.total_wins, game.total_losses, game.total_ties
        );
    }
}

fn zhejiang(last_guess: i32, last_result: i32) -> i32 {
    // Tie: random
    // AI Loss: counter human's last choice
    // AI Win: play what opponent played last
    // https://arxiv.org/pdf/1404.5199v1.pdf
    match last_result {
        0 => return rand::thread_rng().gen_range(1..=3),
        1 => return counter_choice(last_guess),
        -1 => return last_guess,
        _ => panic!("Invalid input to zhejiang"),
    }
}

fn counter_choice(choice: i32) -> i32 {
    if choice == 3 {
        return 1;
    } else {
        return choice + 1;
    }
}

fn solicit_human_guess() -> i32 {
    println!("Please select your choice.");
    loop {
        let mut raw_guess = String::new();
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        }
        io::stdin()
            .read_line(&mut raw_guess)
            .expect("Failed to read line");
        match raw_guess.trim().to_ascii_lowercase().as_str() {
            "1" | "r" | "rock" => return 1,
            "2" | "p" | "paper" => return 2,
            "3" | "s" | "scissors" => return 3,
            _ => println!("User input must correspond to a valid option"),
        }
    }
}

fn choice_name(choice_num: i32) -> String {
    match choice_num {
        1 => return "Rock".to_string(),
        2 => return "Paper".to_string(),
        3 => return "Scissors".to_string(),
        _ => panic!("Choice must be between 1 and 3"),
    }
}
