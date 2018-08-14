use rand::{thread_rng, Rng};

use std::io::{stdin, stdout, Write};

use board::Board;

pub trait Player {
    fn decide(&mut self, board: &Board) -> usize;
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerIndicator {
    P1,
    P2,
}

#[derive(Debug)]
pub struct Human(String);

impl Human {
    pub fn new(name: &str) -> Self {
        Human(name.to_owned())
    }
}

impl Player for Human {
    fn decide(&mut self, _: &Board) -> usize {
        loop {
            print!("{}, insira a sua jogada [0-6]: ", self.0);
            stdout().flush().expect("Failed to flush stdout.");

            let mut buffer = String::new();
            if stdin().read_line(&mut buffer).is_ok() {
                if let Ok(num) = buffer.trim().parse() {
                    return num;
                }
            }

            println!("Entrada inválida.\n");
        }
    }
}

pub struct RandomBot;

impl Player for RandomBot {
    fn decide(&mut self, _board: &Board) -> usize {
        thread_rng().gen_range(0, 7)
    }
}

// TODO: implement intelligent bot
