use std::{collections::HashMap, default::Default, f64};

use board::Board;
use players::Player;

#[derive(Debug)]
struct DecisionTree {
    state: Board,
    static_value: Option<f64>,
    alpha: f64,
    beta: f64,
    children: HashMap<(usize, usize), Box<DecisionTree>>,
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
    fn new(state: Board, heuristic_value: f64, alpha: f64, beta: f64) -> Self {
        Self {
            state,
            static_value: Some(heuristic_value),
            alpha,
            beta,
            children: HashMap::new(),
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
}

impl<F> Player for SmartBot<F>
where
    F: FnMut(&Board) -> f64,
{
    fn decide(&mut self, board: &Board, last_move: Option<(usize, usize)>) -> (usize, usize) {
        unimplemented!()
    }
}
