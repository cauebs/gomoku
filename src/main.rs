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

const SCORE_PER_SPACE: isize = 1;
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

            if spaces + my_spaces > 0 {
                let plus_score = SCORE_PER_SPACE * (5 - spaces)
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

fn main() {
    let stub_test = false;
    let use_human = true;
    let to_end = true;

    if stub_test {
        run_stub();
        return;
    }

    let bot = SmartBot::new(PlayerIndicator::Player2, heuristic, 3);

    if use_human {
        run_test(Human::new("cauebs"), bot, to_end);
    } else {
        run_test(RandomBot{}, bot, to_end);
    }
}
