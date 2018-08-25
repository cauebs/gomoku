#[macro_use]
extern crate failure_derive;
extern crate failure;
extern crate rand;

mod ai;
mod board;
mod coordinates;
mod game;
mod players;

#[cfg(test)]
mod tests;

use game::{EndGame::*, Game, PlayerIndicator};
use players::{Human, SmartBot};

fn main() {
    let bot = SmartBot::new(
        |b| {
            let mut static_value = 0;
            for (i, row) in b.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if let Some(PlayerIndicator::Player1) = cell {
                        static_value += i + j;
                    }
                }
            }
            static_value as f64
        },
        10,
    );
    let mut game = Game::new(Human::new("cauebs"), bot);

    match game.play_to_end() {
        Victory(p) => println!("{:?} has won!", p),
        Draw => println!("It's a draw!"),
    };
}
