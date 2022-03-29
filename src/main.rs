use std::io;
use rand::seq::SliceRandom;

fn main() {
    let choices = vec!["r".to_string(), "p".to_string(), "s".to_string()];
    let playing = true;

    while playing {
        println!("Welcome to Rock-Paper-Scissors written in Rust! Enter your choice:");
        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("failed to readline");
        // println!("Choice: {}", choice);
        choice = choice.trim().to_string();
        if !choices.contains(&choice) {
            println!("\"{}\" is not r, p, or s. Please type one of those three letters", choice);
            continue;
        }

        let rch = choices.choose(&mut rand::thread_rng()).expect("Couldn't convert random choice to string");

        // Rock
        if *rch == choices[0] {
            if choice == choices[0] {
                println!("Rock ties rock!")
            } else if choice == choices[1] {
                println!("Paper covers rock. You win!")
            } else if choice == choices[2] {
                println!("Rock smashes scissors. I win!")
            }
        // Paper
        } else if *rch == choices[1] {
            if choice == choices[0] {
                println!("Paper covers rock. I win!")
            } else if choice == choices[1] {
                println!("Paper ties paper.")
            } else if choice == choices[2] {
                println!("Scissors cut paper. You win!")
            }
        // Scissors
        } else if *rch == choices[2] {
            if choice == choices[0] {
                println!("Paper covers rock. You win!")
            } else if choice == choices[1] {
                println!("Scissors cuts paper. I win!")
            } else if choice == choices[2] {
                println!("Scissors ties scissors!")
            }
        }
    }
}
