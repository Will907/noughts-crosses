//use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Welcome to noughts and crosses!");
    let players: u8 = player_select();
    println!("{players} player game selected");
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