use std::io;
use std::str::FromStr;
use rand::{distributions::{Distribution, Standard}, Rng};

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
pub struct ParseChoiceError;

impl FromStr for Choice {
    type Err = ParseChoiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rock" => Ok(Choice::Rock),
            "paper" => Ok(Choice::Paper),
            "scissors" => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError),
        }
    }
}

impl Distribution<Choice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        match rng.gen_range(0..=2) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            _ => Choice::Scissors,
        }
    }
}

pub fn rock_paper_scissors() {
    'outer: loop {
        println!("Welcome to Rock-Paper-Scissors written in Rust! Enter your choice: rock / paper / scissors");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("failed to readline");

        let c: Choice = match choice.trim().to_lowercase().parse() {
            Err(_e) => { println!("\"{}\" is not rock, paper, or scissors. \
            Please type one of those three letters", choice.trim()); continue }
            Ok(choice) => { choice }
        };

        let rch: Choice = rand::random();

        match (rch, c) {
            (Choice::Rock, Choice::Rock) => println!("Rock ties rock!"),
            (Choice::Rock, Choice::Paper) => println!("Paper covers rock. You win!"),
            (Choice::Rock, Choice::Scissors) => println!("Rock smashes scissors. I win!"),

            (Choice::Paper, Choice::Rock) => println!("Paper covers rock. I win!"),
            (Choice::Paper, Choice::Paper) => println!("Paper ties paper!"),
            (Choice::Paper, Choice::Scissors) => println!("Scissors cuts paper. You win!"),

            (Choice::Scissors, Choice::Rock) => println!("Rock smashes scissors. You win!"),
            (Choice::Scissors, Choice::Paper) => println!("Scissors cuts paper. I win!"),
            (Choice::Scissors, Choice::Scissors) => println!("Scissors ties scissors!"),
        }

        loop {
            println!("Would you like to play again? y / n");
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).expect("failed to readline");
            ans = ans.trim().to_string();
            if ans.to_lowercase() == "y" {
                continue 'outer;
            } else if ans.to_lowercase() == "n" {
                return;
            } else {
                println!("Please type \"y\" (yes) or \"n\" (no)");
                continue;
            }
        }
    }
}