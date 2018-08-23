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
use players::{Human, SmartBot, RandomBot};

fn main() {
    let cauebs = Human::new("cauebs");
    // let jptiz = Human::new("jptiz");
    let bot = SmartBot::new(
        |g| 0.5,
        |g| 0.2,
        2,
    );

    match Game::new(cauebs, bot).play(PlayerIndicator::Player2) {
        Victory(p) => println!("{:?} has won!", p),
        Draw => println!("It's a draw!"),
    };
}
