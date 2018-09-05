use std::{
    cmp::{max, min},
    collections::HashMap,
};

use board::{Board, Coord};
use game::PlayerIndicator;

#[derive(Debug)]
pub struct DecisionTree {
    state: Board,
    player: PlayerIndicator,
    static_value: Option<i32>,
    alpha: i32,
    beta: i32,
    children: HashMap<Coord, Box<DecisionTree>>,
}

impl Default for DecisionTree {
    fn default() -> Self {
        Self::new(Board::default(), PlayerIndicator::Player1)
    }
}

impl DecisionTree {
    pub fn new(state: Board, player: PlayerIndicator) -> Self {
        Self {
            state,
            player,
            static_value: None,
            alpha: i32::min_value(),
            beta: i32::max_value(),
            children: HashMap::new(),
        }
    }

    pub fn deepen<F>(&mut self, evaluator: &F, remaining_levels: u32, player: PlayerIndicator)
    where
        F: Fn(&Board, PlayerIndicator) -> i32,
    {
        use PlayerIndicator::{Player1, Player2};

        // TODO: check if it's a leaf node
        if remaining_levels == 0 {
            self.static_value = Some(evaluator(&self.state, player));
            return;
        }

        for next_move in self.state.possible_moves() {
            let mut next_state = self.state.clone();
            next_state
                .make_move(self.player, next_move)
                .expect("AI thinks it can make a move it actually cannot!");

            let next_player = match self.player {
                Player1 => Player2,
                Player2 => Player1,
            };

            let mut child = DecisionTree::new(next_state, next_player);
            child.deepen(evaluator, remaining_levels - 1, player);

            if let Some(value) = child.static_value {
                if self.player == player {
                    self.alpha = max(value, self.alpha);
                    self.static_value = Some(self.alpha);
                } else {
                    self.beta = min(value, self.beta);
                    self.static_value = Some(self.beta);
                }
            }

            self.children.insert(next_move, Box::new(child));

            if self.alpha >= self.beta {
                break;
            }
        }

        // ...
    }

    pub fn decide(&self) -> Coord {
        *self
            .children
            .iter()
            .filter(|(_, subtree)| subtree.static_value.is_some())
            .max_by_key(|(_, subtree)| subtree.static_value.unwrap())
            .expect("No available moves.")
            .0
    }

    pub fn extract_subtree(&mut self, coords: &Coord) -> Option<DecisionTree> {
        self.children.remove(coords).map(|boxed| *boxed)
    }
}
