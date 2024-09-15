use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Welcome to noughts and crosses!");
    loop {
        let players: u8 = player_select();
        game(&players);
        if players == 1 {
            break;
        }
    }
}

fn player_select() -> u8 {
    loop {
        let mut players = String::new();
        print!("1 or 2 player? ");
        if let Err(error) = io::stdout().flush() {
            panic!("{}",error)
        };

        io::stdin()
            .read_line(&mut players)
            .expect("Failed to read line");
        let players: u8 = match players.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        for i in 1..3 {
            if i == players {
                return players;
            } else {
                continue;
            }
        }
    }
}

fn game(players: &u8) {
    println!("{}",players);
    let mut first: u8 = 0;
    match rand::thread_rng().gen_range(1..=2) {
        1 => {
            first = 1;
            if players.clone() == 1 {
                println!("Player goes first")
            } else {
                println!("Player 1 goes first")
            }
        },
        2 => {
            first = 2;
            if players.clone() == 1 {
                println!("Computer goes first")
            } else {
                println!("Player 2 goes first")
            }
        },
        _ => println!("Not sure what happened here"),
    };
    println!("{first}");
}