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

use std::cmp::min;

use axes::Axes;
use board::Board;
use game::{EndGame::*, Game, PlayerIndicator};
use players::{Player, Human, RandomBot, SmartBot};

const SCORE_PER_SPACE: u32 = 1;
const SCORE_PER_MY_SPACE: u32 = 4;

fn heuristic(board: &Board, player: PlayerIndicator) -> isize {
    let mut score = 0;

    for axis in Axes::new(&board) {
        let mut spaces = 0;
        let mut my_spaces = 0isize;
        for cell in axis {
            let (i, j) = cell.0;
            match board[i][j] {
                Some(cell_player) if cell_player == player => {
                    my_spaces += 1;
                },
                Some(_) => {
                    spaces = 0;
                    my_spaces = 0;
                },
                None => spaces += 1,
            }

            if my_spaces == 5 {
                return isize::max_value();
            }

            if spaces + my_spaces > 0 {
                let plus_score = (5 - min(5, spaces)) * SCORE_PER_SPACE as isize
                    + my_spaces.pow(SCORE_PER_MY_SPACE);
                score += plus_score;
            }
        }
    }

    score
}

#[allow(unused)]
fn run_stub() {
    use PlayerIndicator::*;

    let mut board = Board::default();
    board.make_move(Player2, (0,0));
    println!("{}\nScore: {}", board, heuristic(&board, Player2));
    board.make_move(Player2, (0,1));
    board.make_move(Player2, (0,2));
    board.make_move(Player2, (0,3));
    println!("{}\nScore: {}", board, heuristic(&board, Player2));
}

fn run_test<P1, P2>(player1: P1, player2: P2, to_end: bool)
where
    P1: Player,
    P2: Player,
{
    let mut game = Game::new(player1, player2);

    let result = if to_end {
        Some(game.play_to_end())
    } else {
        game.play_turns(10)
    };

    match result {
        Some(Victory(p)) => println!("{:?} has won!", p),
        Some(Draw) => println!("It's a draw!"),
        None => println!("No one won."),
    };

    println!("{} ({} turns)", game.board, game.moves.len());
}

enum TestType {
    HS,
    RS,
    STUB,
}

fn main() {
    let test_type = TestType::RS;
    let to_end = true;
    let depth = 3;

    match test_type {
        TestType::STUB => {
            run_stub();
            return;
        }
        _ => {},
    }

    let bot = SmartBot::new(PlayerIndicator::Player2, heuristic, depth);
    match test_type {
        TestType::HS => run_test(Human::new("cauebs"), bot, to_end),
        TestType::RS => run_test(RandomBot{}, bot, to_end),
        _ => {},
    }
}
