use rand::{thread_rng, Rng};

use std::io::{stdin, stdout, Write};

use board::Board;
use coordinates::Coordinates;

pub trait Player {
    fn decide(&mut self, board: &Board) -> Coordinates;
}

#[derive(Debug)]
pub struct Human(String);

impl Human {
    pub fn new(name: &str) -> Self {
        Human(name.to_owned())
    }
}

impl Player for Human {
    fn decide(&mut self, _: &Board) -> Coordinates {
        loop {
            print!("{}, insert your next move ([0-E],[0-E]): ", self.0);
            stdout().flush().expect("Failed to flush stdout.");

            let mut buffer = String::new();
            if stdin().read_line(&mut buffer).is_ok() {
                if let Ok(coords) = Coordinates::from_hex_str(buffer.trim()) {
                    return coords;
                }
            }

            println!("Invalid input.\n");
        }
    }
}

pub struct RandomBot;

impl Player for RandomBot {
    fn decide(&mut self, _board: &Board) -> Coordinates {
        let mut rng = thread_rng();
        Coordinates(rng.gen_range(0, 15), rng.gen_range(0, 15))
    }
}

pub struct TestBot {
    moves: Vec<Coordinates>,
}

impl TestBot {
    pub fn new(mut moves: Vec<Coordinates>) -> Self {
        moves.reverse();
        Self { moves }
    }
}

impl Player for TestBot {
    fn decide(&mut self, _board: &Board) -> Coordinates {
        self.moves.pop().expect("Not enough moves.")
    }
}

// TODO: implement intelligent bot
