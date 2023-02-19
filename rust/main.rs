use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Let's play rock paper scissors!");
    let mut total_ties: i32 = 0;
    let mut total_losses: i32 = 0;
    let mut total_wins: i32 = 0;
    let mut last_guess: i32 = rand::thread_rng().gen_range(1..=3);
    let mut last_result = 0;
    loop {
        let human_guess = solicit_human_guess();
        last_result = rock_paper_scissors(human_guess, last_guess, last_result);
        match last_result {
            1 => {
                println!("You Win!");
                total_wins += 1
            }
            0 => {
                println!("It's a Tie!");
                total_ties += 1
            }
            -1 => {
                println!("You Lost!");
                total_losses += 1
            }
            _ => panic!("RPS returned invalid result"),
        }
        last_guess = human_guess;
        println!("Record: {total_wins}-{total_losses}-{total_ties}");
        println!()
    }
}

fn rock_paper_scissors(human_guess: i32, last_guess: i32, last_result: i32) -> i32 {
    // Prompt a user for a guess; play RPS vs an "AI"
    // return 1 for victory, 0 for tie, -1 for defeat
    let ai_guess = get_ai_guess(last_guess, last_result);
    let ai_guess_name = choice_name(ai_guess);
    let guess_name = choice_name(human_guess);
    println!("AI chose {ai_guess_name}");
    println!("You chose {guess_name}");

    match human_guess - ai_guess {
        0 => return 0,
        -2 => return 1,
        -1 => return -1,
        1 => return 1,
        2 => return -1,
        _ => panic!("Invalid guess comparison"),
    }
}

fn get_ai_guess(last_guess: i32, last_result: i32) -> i32 {
    // zhejiang strat 1/3 the time, random 2/3
    if rand::thread_rng().gen_range(1..=3) == 1 {
        zhejiang(last_guess, last_result)
    } else {
        rand::thread_rng().gen_range(1..=3)
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
