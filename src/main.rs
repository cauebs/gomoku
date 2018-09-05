#[macro_use]
extern crate failure_derive;
extern crate failure;
extern crate rand;
extern crate rayon;

mod axes;
mod board;
mod coordinates;
mod game;

mod ai;
mod players;
mod tree;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use board::Board;
use game::{EndGame::*, Game, PlayerIndicator};
use players::{Human, RandomBot, SmartBot};

fn heuristic(board: &Board, player: PlayerIndicator) -> i32 {
    use PlayerIndicator::{Player1, Player2};
    const ci: i32 = 100000;
    const cp: i32 = 100;
    const cj: i32 = -1;

    let s = board;

    let mut scores = HashMap::new();

    let mut combo_player = (None, 0);

    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Some(cell_player) = cell {
                combo_player = match combo_player {
                    (Some(comber), streak) if comber == cell_player => (Some(comber), streak + 1),
                    (Some(comber), streak) => {
                        scores.entry(cell_player).or_insert(0);
                        scores.entry(cell_player).and_modify(|yay| *yay += streak);
                        (Some(comber), 0)
                    }
                    (None, streak) => (Some(cell_player), streak),
                };
            }
        }
    }

    *scores.entry(&player).or_insert(0) as i32
}

fn main() {
    let human = Human::new("cauebs");
    let bot = SmartBot::new(PlayerIndicator::Player2, heuristic, 2);
    let mut game = Game::new(human, bot);

    match game.play_to_end() {
        Victory(p) => println!("{:?} has won!", p),
        Draw => println!("It's a draw!"),
    };

    println!("{} ({} turns)", game.board, game.moves.len());
}
