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

#[cfg(test)]
mod tests;

use axes::Axes;
use board::Board;
use game::{EndGame::*, Game, PlayerIndicator};
use players::{Human, SmartBot};

use PlayerIndicator::*;

fn heuristic(board: &Board, player: PlayerIndicator) -> isize {
    fn score_for_player(board: &Board, player: PlayerIndicator) -> isize {
        let max = Axes::new(&board)
            .map(|axis| {
                axis.streaks_with_room(player)
                    .iter()
                    .map(|x| x.len())
                    .max()
                    .unwrap_or(0) as isize
            }).max()
            .unwrap_or(0);

        if max == 5 {
            isize::max_value()
        } else {
            max
        }
    }

    let player_score = score_for_player(&board, player);
    player_score
}

fn main() {
    let depth = 2;

    let player1 = Human::new("Human Player");
    let player2 = SmartBot::new(Player2, heuristic, depth);
    let mut game = Game::new(player1, player2);

    match game.play_to_end() {
        Victory(p) => println!("{:?} has won!", p),
        Draw => println!("It's a draw!"),
    };

    println!("{} ({} turns)", game.board, game.moves.len());
}
