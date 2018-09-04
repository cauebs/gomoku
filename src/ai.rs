use std::default::Default;

use board::{Board, Coord};
use game::PlayerIndicator;
use players::Player;
use tree::DecisionTree;

#[derive(Debug)]
pub struct SmartBot<F>
where
    F: Fn(&Board, PlayerIndicator) -> i32,
{
    player_id: PlayerIndicator,
    static_evaluator: F,
    decision_tree: DecisionTree,
    recursion_limit: u32,
}

impl<F> SmartBot<F>
where
    F: Fn(&Board, PlayerIndicator) -> i32,
{
    pub fn new(player_id: PlayerIndicator, static_evaluator: F, recursion_limit: u32) -> Self {
        let mut tree = DecisionTree::default();
        tree.deepen(&static_evaluator, recursion_limit, player_id);

        Self {
            player_id,
            static_evaluator,
            decision_tree: tree,
            recursion_limit,
        }
    }
}

impl<F> Player for SmartBot<F>
where
    F: Fn(&Board, PlayerIndicator) -> i32,
{
    fn decide(&mut self, _board: &Board, last_move: Option<Coord>) -> Coord {
        if let Some(last_move) = last_move {
            self.decision_tree = self
                .decision_tree
                .extract_subtree(&last_move)
                .expect("The last move wasn't predicted by the decision tree.");
        }

        self.decision_tree
            .deepen(&self.static_evaluator, self.recursion_limit, self.player_id);

        let next_move = self.decision_tree.decide();
        self.decision_tree = self.decision_tree.extract_subtree(&next_move).unwrap();
        next_move
    }
}
