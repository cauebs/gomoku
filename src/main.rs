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
    const SCORE_PER_MY_SPACE: u32 = 10;

    fn score_for_player(board: &Board, player: PlayerIndicator) -> isize {
        let mut score = 0;

        for axis in Axes::new(&board) {
            for streak in axis.streaks_with_room(player) {
                match streak.len() {
                    2...4 => {
                        score += streak.len().pow(SCORE_PER_MY_SPACE) as isize;
                    }
                    5 => return isize::max_value(),
                    _ => {}
                }
            }
        }

        score
    }

    let opponent = if player == Player1 { Player2 } else { Player1 };
    score_for_player(&board, player) - score_for_player(&board, opponent)
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
