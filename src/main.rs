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
use players::{Player, Human, RandomBot, SmartBot};

use PlayerIndicator::*;

const SCORE_PER_MY_SPACE: u32 = 10;

fn heuristic(board: &Board, player: PlayerIndicator) -> isize {
    fn score_for_player(board: &Board, player: PlayerIndicator) -> isize {
        let mut score = 0;

        for axis in Axes::new(&board) {
            for streak in axis.streaks_with_room(player) {
                match streak.len() {
                    x if x == 1 => {
                        continue;
                    }
                    x if x < 5 => {
                        score += (streak.len() as isize).pow(SCORE_PER_MY_SPACE);
                    }
                    x if x == 5 => {
                        return isize::max_value() / 2;
                    }
                    _ => {},
                }
            }
        }

        score
    }

    let (p1, p2) = match player {
        x if x == Player1 => (Player1, Player2),
        _ => (Player2, Player1),
    };

    score_for_player(&board, p1) - score_for_player(&board, p2)
}

#[allow(unused)]
fn run_stub() {
    let mut board = Board::default();
    println!("{}\nScore: {}", board, heuristic(&board, Player2));
    board.make_move(Player2, (0, 0));
    println!("{}\nScore: {}", board, heuristic(&board, Player2));
    board.make_move(Player2, (0, 1));
    board.make_move(Player2, (0, 2));
    board.make_move(Player2, (0, 3));
    println!("{}\nScore: {}", board, heuristic(&board, Player2));
    board.make_move(Player2, (0, 4));
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

#[allow(unused)]
enum TestType {
    HS,
    RS,
    STUB,
}

fn main() {
    let test_type = TestType::HS;
    let to_end = true;
    let depth = 2;

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
