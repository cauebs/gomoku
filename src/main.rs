#[macro_use]
extern crate failure_derive;
extern crate failure;
extern crate rand;
extern crate rayon;

mod board;
mod coordinates;
mod game;

mod ai;
mod players;
mod tree;

#[cfg(test)]
mod tests;

use game::{EndGame::*, Game, PlayerIndicator};
use players::{Human, SmartBot};

fn main() {
    let human = Human::new("cauebs");

    let bot = SmartBot::new(
        PlayerIndicator::Player2,
        |b| {
            let mut static_value = 0;
            for (i, row) in b.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if let Some(PlayerIndicator::Player1) = cell {
                        static_value += i + j;
                    }
                }
            }
            static_value as i32
        },
        2,
    );

    let mut game = Game::new(human, bot);

    match game.play_to_end() {
        Victory(p) => println!("{:?} has won!", p),
        Draw => println!("It's a draw!"),
    };
}
