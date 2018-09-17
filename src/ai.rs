use board::{Board, Coord};
use game::PlayerIndicator;
use players::Player;

#[derive(Debug)]
pub struct SmartBot<F>
where
    F: Fn(&Board, PlayerIndicator) -> isize,
{
    player_id: PlayerIndicator,
    static_evaluator: F,
    recursion_limit: u32,
}

impl<F> SmartBot<F>
where
    F: Fn(&Board, PlayerIndicator) -> isize,
{
    pub fn new(player_id: PlayerIndicator, static_evaluator: F, recursion_limit: u32) -> Self {
        Self {
            player_id,
            static_evaluator,
            recursion_limit,
        }
    }

    fn minimax_aux(
        &mut self,
        board: &Board,
        depth: u32,
        mut alpha: isize,
        mut beta: isize,
        maximizing: bool,
    ) -> (isize, Option<Coord>) {
        // TODO: check if it's a leaf node
        if depth == 0 {
            return ((self.static_evaluator)(&board, self.player_id), None);
        }

        let mut best_move = None;
        let mut best_value = if maximizing {
            isize::min_value()
        } else {
            isize::max_value()
        };

        for m in board.possible_moves() {
            let mut child_board = board.clone();
            child_board
                .make_move(self.player_id, m)
                .expect("AI thinks it can make a move it actually cannot!");

            let (value, _) = self.minimax_aux(&child_board, depth - 1, alpha, beta, !maximizing);

            if (value > best_value && maximizing) || value < best_value {
                best_value = value;
                best_move = Some(m);
            }

            if maximizing && value > alpha {
                alpha = value;
            } else if value < beta {
                beta = value;
            }

            if alpha > beta {
                break;
            }
        }

        (best_value, best_move)
    }

    fn minimax(&mut self, board: &Board) -> Coord {
        let depth = self.recursion_limit;
        let (alpha, beta) = (isize::min_value(), isize::max_value());

        let m = self.minimax_aux(board, depth, alpha, beta, true).1;
        m.expect("The only winning move is not to play.")
    }
}

impl<F> Player for SmartBot<F>
where
    F: Fn(&Board, PlayerIndicator) -> isize,
{
    fn decide(&mut self, board: &Board, _last_move: Option<Coord>) -> Coord {
        self.minimax(board)
    }
}
