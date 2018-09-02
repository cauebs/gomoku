use std::{collections::HashMap, default::Default, f64};

use board::{Board, Coord};
use players::Player;
use game::{possible_moves};

#[derive(Debug)]
struct DecisionTree {
    state: Board,
    static_value: Option<f64>,
    alpha: f64,
    beta: f64,
    children: HashMap<Coord, Box<DecisionTree>>,
}

impl Default for DecisionTree {
    fn default() -> Self {
        Self {
            state: Board::default(),
            static_value: None,
            alpha: f64::NEG_INFINITY,
            beta: f64::INFINITY,
            children: HashMap::new(),
        }
    }
}

impl DecisionTree {
    fn new(state: Board, static_value: f64, alpha: f64, beta: f64) -> Self {
        Self {
            state,
            static_value: Some(static_value),
            alpha,
            beta,
            children: HashMap::new(),
        }
    }

    fn from_possibility(
        state: Board,
        depth: u32,
    ) -> Self {
        use PlayerIndicator::*;

        let mut children = HashMap::new();

        if depth != 0 {
            let players = [Player1, Player2];
            for (&possibility, &player) in possible_moves(&state)
                                         .iter()
                                         .zip(players.iter().cycle()) {
                let mut child_board = state.clone();
                child_board.make_move(player, possibility);
                children.insert(
                    possibility,
                    Box::new(DecisionTree::from_possibility(child_board, depth - 1)));
            }
        }

        Self {
            state: state,
            static_value: None,
            alpha: f64::NEG_INFINITY,
            beta: f64::INFINITY,
            children: children
        }
    }
}

#[derive(Debug)]
pub struct SmartBot<F>
where
    F: FnMut(&Board) -> f64,
{
    static_evaluator: F,
    decision_tree: DecisionTree,
    recursion_limit: u32,
}

fn minimax<F>
(
    mut alpha: f64,
    mut beta: f64,
    node: &mut DecisionTree,
    minimizing: bool,
    depth: u32,
    evaluator: &mut F
) -> f64
where F: FnMut(&Board) -> f64
{
    if depth == 0 || node.children.is_empty() {
        let value = evaluator(&node.state);
        node.static_value = Some(value);
        return value;
    }

    for (_move, mut child) in &mut node.children {
        let grade = minimax(alpha, beta, &mut child, !minimizing, depth - 1, evaluator);
        if minimizing {
            beta = f64::min(grade, beta);
        } else {
            alpha = f64::max(grade, alpha);
        }
    }

    if minimizing {
        return beta;
    }
    return alpha;
}

impl<F> SmartBot<F>
where
    F: FnMut(&Board) -> f64,
{
    pub fn new(static_evaluator: F, recursion_limit: u32) -> Self {
        Self {
            static_evaluator,
            decision_tree: Default::default(),
            recursion_limit,
        }
    }

    pub fn update(&mut self, board: Board) {
        self.decision_tree = DecisionTree::from_possibility(board, 2);

        minimax(
            f64::NEG_INFINITY,
            f64::INFINITY,
            &mut self.decision_tree,
            true,
            5,
            &mut self.static_evaluator);
    }
}

impl<F> Player for SmartBot<F>
where
    F: FnMut(&Board) -> f64,
{
    fn decide(&mut self, _board: &Board, _last_move: Option<Coord>) -> Coord {
        self.update(_board.clone());

        let max = self.decision_tree
            .children
            .iter()
            .max_by(
                |(_, acc), (_, child)|
                acc.static_value.unwrap()
                .partial_cmp(
                    &child.static_value.unwrap()
                ).unwrap());

        *max.unwrap().0
    }
}
