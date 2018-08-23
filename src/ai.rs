use std::{collections::HashMap, default::Default, f64};

use board::Board;
use coordinates::Coordinates;
use players::Player;

#[derive(Debug)]
struct DecisionTree {
    state: Board,
    heuristic_value: Option<f64>,
    alpha: f64,
    beta: f64,
    children: HashMap<Coordinates, Box<DecisionTree>>,
}

impl Default for DecisionTree {
    fn default() -> Self {
        Self {
            state: Board::default(),
            heuristic_value: None,
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
            heuristic_value: Some(heuristic_value),
            alpha,
            beta,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct SmartBot<H, U>
where
    H: FnMut(&Board) -> f64,
    U: FnMut(&Board) -> f64,
{
    heuristic: H,
    utility_function: U,
    decision_tree: DecisionTree,
    recursion_limit: u32,
}

impl<H, U> SmartBot<H, U>
where
    H: FnMut(&Board) -> f64,
    U: FnMut(&Board) -> f64,
{
    pub fn new(
        heuristic: H,
        utility_function: U,
        recursion_limit: u32,
    ) -> Self {
        Self {
            heuristic,
            utility_function,
            decision_tree: Default::default(),
            recursion_limit,
        }
    }
}

impl<H, U> Player for SmartBot<H, U>
where
    H: FnMut(&Board) -> f64,
    U: FnMut(&Board) -> f64,
{
    fn decide(&mut self, board: &Board) -> Coordinates {
        unimplemented!()
    }
}
