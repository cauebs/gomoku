#![allow(dead_code)]

use rand::{thread_rng, Rng};

use std::io::{stdin, stdout, Write};

pub use ai::SmartBot;
use board::Board;
use coordinates::coordinates_from_hex_str;

pub trait Player {
    fn decide(
        &mut self,
        board: &Board,
        last_move: Option<(usize, usize)>,
    ) -> (usize, usize);
}

#[derive(Debug)]
pub struct Human(String);

impl Human {
    pub fn new(name: &str) -> Self {
        Human(name.to_owned())
    }
}

impl Player for Human {
    fn decide(
        &mut self,
        board: &Board,
        _last_move: Option<(usize, usize)>,
    ) -> (usize, usize) {
        loop {
            println!("\n{}", board);

            print!("{}, insert your next move ([0-E],[0-E]): ", self.0);
            stdout().flush().expect("Failed to flush stdout.");

            let mut buffer = String::new();
            if stdin().read_line(&mut buffer).is_ok() {
                if let Ok(coords) = coordinates_from_hex_str(buffer.trim()) {
                    return coords;
                }
            }

            println!("Invalid input.\n");
        }
    }
}

pub struct RandomBot;

impl Player for RandomBot {
    fn decide(
        &mut self,
        _board: &Board,
        _last_move: Option<(usize, usize)>,
    ) -> (usize, usize) {
        let mut rng = thread_rng();
        (rng.gen_range(0, 15), rng.gen_range(0, 15))
    }
}

pub struct TestBot {
    moves: Vec<(usize, usize)>,
}

impl TestBot {
    pub fn new(mut moves: Vec<(usize, usize)>) -> Self {
        moves.reverse();
        Self { moves }
    }
}

impl Player for TestBot {
    fn decide(
        &mut self,
        _board: &Board,
        _last_move: Option<(usize, usize)>,
    ) -> (usize, usize) {
        self.moves.pop().expect("Not enough moves.")
    }
}

// TODO: implement intelligent bot
