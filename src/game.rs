use board::Board;
use players::Player;

const VICTORY_STREAK: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerIndicator {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq)]
pub enum EndGame {
    Victory(PlayerIndicator),
    Draw,
}

#[derive(Debug)]
pub struct Game<P1: Player, P2: Player> {
    board: Board,
    player1: P1,
    player2: P2,
    current_turn: PlayerIndicator,
    moves: Vec<(usize, usize)>,
}

impl<P1: Player, P2: Player> Game<P1, P2> {
    pub fn new(player1: P1, player2: P2) -> Self {
        Self {
            board: Default::default(),
            player1,
            player2,
            current_turn: PlayerIndicator::Player1,
            moves: Vec::new(),
        }
    }

    pub fn play_turn(&mut self) -> Option<EndGame> {
        use self::PlayerIndicator::{Player1, Player2};

        let last_move = self.moves.last().cloned();

        loop {
            let (coords, next_turn) = match self.current_turn {
                Player1 => (self.player1.decide(&self.board, last_move), Player2),
                Player2 => (self.player2.decide(&self.board, last_move), Player1),
            };

            if let Err(e) = self.board.make_move(self.current_turn, coords) {
                eprintln!("Jogada inválida: {}", e);
                continue;
            } else {
                self.moves.push(coords);
                self.current_turn = next_turn;
                break;
            }
        }

        self.check_end()
    }

    pub fn play_turns(&mut self, turns: u32) -> Option<EndGame> {
        for _ in 0..turns {
            let end = self.play_turn();
            if end.is_some() {
                return end;
            }
        }

        None
    }

    pub fn play_to_end(&mut self) -> EndGame {
        loop {
            if let Some(end) = self.play_turn() {
                return end;
            }
        }
    }

    pub fn check_end(&self) -> Option<EndGame> {
        if let Some(end) = self.check_for_horizontal_victory() {
            return Some(end);
        }

        if let Some(end) = self.check_for_vertical_victory() {
            return Some(end);
        }

        if let Some(end) = self.check_for_diagonal_victory() {
            return Some(end);
        }

        if self
            .board
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
        {
            Some(EndGame::Draw)
        } else {
            None
        }
    }

    fn check_for_horizontal_victory(&self) -> Option<EndGame> {
        for row in self.board.iter() {
            let (mut tracking, mut streak) = (None, 0);

            for &cell in row.iter() {
                streak = match (cell, tracking) {
                    (Some(c), Some(t)) if c == t => streak + 1,
                    (Some(_), _) => 1,
                    (None, _) => 0,
                };

                if streak >= VICTORY_STREAK {
                    return tracking.map(EndGame::Victory);
                }

                tracking = cell;
            }
        }

        None
    }

    fn check_for_vertical_victory(&self) -> Option<EndGame> {
        for j in 0..self.board.width() {
            let (mut tracking, mut streak) = (None, 0);

            for i in 0..self.board.height() {
                let cell = self.board[i][j];

                streak = match (cell, tracking) {
                    (Some(c), Some(t)) if c == t => streak + 1,
                    (Some(_), _) => 1,
                    (None, _) => 0,
                };

                if streak >= VICTORY_STREAK {
                    return tracking.map(EndGame::Victory);
                }

                tracking = cell;
            }
        }

        None
    }

    fn diagonal_cell_search(&self, coords: (usize, usize), reverse: bool) -> Option<EndGame> {
        let (mut i, mut j) = (coords.0, coords.1);

        let tracking = self.board[i][j]?;
        let mut streak = 1;

        let d = if reverse { -1 } else { 1 };

        for _ in 0..VICTORY_STREAK - 1 {
            i += 1;
            j = (j as isize + d) as usize;

            if self.board[i][j]? == tracking {
                streak += 1;
            }
        }

        if streak >= VICTORY_STREAK {
            return Some(EndGame::Victory(tracking));
        }

        None
    }

    fn check_for_diagonal_victory(&self) -> Option<EndGame> {
        let height = self.board.height();
        let width = self.board.width();

        for i in 0..height + 1 - VICTORY_STREAK {
            for j in 0..width {
                if j <= width - VICTORY_STREAK {
                    if let Some(end) = self.diagonal_cell_search((i, j), false) {
                        return Some(end);
                    }
                }

                if j >= VICTORY_STREAK - 1 {
                    if let Some(end) = self.diagonal_cell_search((i, j), true) {
                        return Some(end);
                    }
                }
            }
        }

        None
    }
}
