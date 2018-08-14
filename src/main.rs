extern crate rand;

mod board;
mod game;
mod players;

use game::Game;

fn main() {
    let cauebs = players::Human::new("cauebs");
    // let jptiz = players::Human::new("jptiz");
    let bot = players::RandomBot;

    Game::new(cauebs, bot).play();
}
