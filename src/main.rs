#[macro_use]
extern crate failure_derive;
extern crate failure;
extern crate rand;

mod board;
mod coordinates;
mod game;
mod players;

use game::{Game, PlayerIndicator, EndGame::*};
use players::{Human, RandomBot};

fn main() {
    let cauebs = Human::new("cauebs");
    // let jptiz = Human::new("jptiz");
    let bot = RandomBot;

    match Game::new(cauebs, bot).play(PlayerIndicator::Bot) {
        Victory(p) => println!("{:?} has won!", p),
        Draw => println!("It's a draw!"),
    };
}
