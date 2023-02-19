use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    // human_vs_ai_rps_loop()
    ai_vs_ai_rps_loop()
}

enum Strats {
    Zhejiang,
    Random,
}
// todo: specify ome of these in arg?

enum RPSOptions {
    Rock,
    Paper,
    Scissors,
}
// todo: use these instead of i32

struct RPSGame {
    total_ties: i32,
    total_losses: i32,
    total_wins: i32,
    last_p1_guess: i32,
    last_p2_guess: i32,
    last_result: i32,
}

impl RPSGame {
    fn rps_round(&mut self, human_guess: i32) -> i32 {
        // return 1 for victory, 0 for tie, -1 for defeat
        let ai_guess = self.get_ai_guess(self.last_p1_guess, self.last_result, 50);
        // let ai_guess_name = choice_name(ai_guess);
        // let guess_name = choice_name(human_guess);
        // println!("AI chose {ai_guess_name}");
        // println!("You chose {guess_name}");

        let result: i32;
        match human_guess - ai_guess {
            0 => result = 0,
            -2 => result = 1,
            -1 => result = -1,
            1 => result = 1,
            2 => result = -1,
            _ => panic!("Invalid guess comparison"),
        }
        self.last_result = result;
        self.last_p1_guess = human_guess;
        return result;
    }

    fn get_ai_guess(&self, last_opp_guess: i32, last_result: i32, random_chance: i32) -> i32 {
        // last_opp_guess is the opposing player's last guess
        // last result is relative to the opponent i.e. 1 means opponent won
        // random_chance is between 0 and 100
        // zhejiang strat 1/3 the time, random 2/3
        if rand::thread_rng().gen_range(1..=100) <= random_chance {
            zhejiang(last_opp_guess, last_result)
        } else {
            rand::thread_rng().gen_range(1..=3)
        }
    }
}

fn make_rps_game() -> RPSGame {
    RPSGame {
        total_ties: 0,
        total_losses: 0,
        total_wins: 0,
        last_p1_guess: rand::thread_rng().gen_range(1..=3),
        last_p2_guess: rand::thread_rng().gen_range(1..=3),
        last_result: 0,
    }
}

fn human_vs_ai_rps_loop() {
    println!("Let's play rock paper scissors!");
    let mut game = make_rps_game();
    loop {
        let human_guess = solicit_human_guess();
        println!("AI chose {}", choice_name(game.last_p2_guess));
        println!("You chose {}", choice_name(game.last_p1_guess));
        match game.rps_round(human_guess) {
            1 => {
                println!("You Win!");
                game.total_wins += 1
            }
            0 => {
                println!("It's a Tie!");
                game.total_ties += 1
            }
            -1 => {
                println!("You Lost!");
                game.total_losses += 1
            }
            _ => panic!("RPS returned invalid result"),
        }
        println!(
            "Record: {}-{}-{}",
            game.total_wins, game.total_losses, game.total_ties
        );
        println!()
    }
}

fn ai_vs_ai_rps_loop() {
    println!("Let's play rock paper scissors!");
    let mut game = make_rps_game();
    loop {
        let ai1_guess = game.get_ai_guess(game.last_p2_guess, -game.last_result, 10);
        match game.rps_round(ai1_guess) {
            1 => game.total_wins += 1,
            0 => game.total_ties += 1,
            -1 => game.total_losses += 1,
            _ => panic!("RPS returned invalid result"),
        }
        println!(
            "Record: {}-{}-{}",
            game.total_wins, game.total_losses, game.total_ties
        );
        println!()
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
