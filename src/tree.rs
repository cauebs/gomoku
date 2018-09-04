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

    pub fn deepen<F>(&mut self, evaluator: &F, depth: u32)
    where
        F: Fn(&Board) -> i32,
    {
        self.deepen_aux(evaluator, depth);
    }

    fn deepen_aux<F>(&mut self, evaluator: &F, remaining_levels: u32)
    where
        F: Fn(&Board) -> i32,
    {
        use PlayerIndicator::{Player1, Player2};

        let possible_moves = self.state.possible_moves();

        if remaining_levels == 0 || possible_moves.is_empty() {
            self.static_value = Some(evaluator(&self.state));
            return;
        }

        for next_move in possible_moves {
            let mut next_state = self.state.clone();
            next_state
                .make_move(self.player, next_move)
                .expect("AI thinks it can make a move it actually cannot!");

            let next_player = match self.player {
                Player1 => {
                    if let Some(value) = self.static_value {
                        self.beta = min(value, self.beta);
                    }
                    Player2
                }
                Player2 => {
                    if let Some(value) = self.static_value {
                        self.alpha = max(value, self.alpha);
                    }
                    Player1
                }
            };

            if self.alpha >= self.beta {
                break;
            }

            let mut child = DecisionTree::new(next_state, next_player);
            child.deepen_aux(evaluator, remaining_levels - 1);

            self.children.insert(next_move, Box::new(child));
        }
    }

    pub fn decide(&self) -> Coord {
        *self.children
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
