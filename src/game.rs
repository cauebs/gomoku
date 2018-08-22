use failure;

use board::Board;
use coordinates::Coordinates;
use players::Player;

const VICTORY_STREAK: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerIndicator {
    Player1,
    Player2,
}

type Cell = Option<PlayerIndicator>;

#[derive(Debug, PartialEq)]
pub enum EndGame {
    Victory(PlayerIndicator),
    Draw,
}

#[derive(Fail, Debug)]
enum Error {
    #[fail(display = "Cell is already occupied.")]
    OccupiedCell,
    #[fail(display = "Coordinates out of bounds.")]
    CoordinatesOutOfBounds,
}

#[derive(Debug)]
pub struct Game<P1: Player, P2: Player> {
    board: Board,
    player1: P1,
    player2: P2,
    current_turn: Cell,
    turns: u32,
}

impl<P1: Player, P2: Player> Game<P1, P2> {
    pub fn new(player1: P1, player2: P2) -> Self {
        Self {
            board: Default::default(),
            player1,
            player2,
            current_turn: None,
            turns: 0,
        }
    }

    pub fn play(&mut self, first: PlayerIndicator) -> EndGame {
        use self::PlayerIndicator::{Player2, Player1};

        self.current_turn = Some(first);

        loop {
            let (coords, next_turn) = match self.current_turn.unwrap() {
                Player1 => (self.player1.decide(&self.board), Player2),
                Player2 => (self.player2.decide(&self.board), Player1),
            };

            if let Err(e) = self.make_move(&coords) {
                println!("Invalid move: {}\n", e);
                continue;
            }

            if let Some(end) = self.check_end() {
                return end;
            }

            self.current_turn = Some(next_turn);
        }
    }

    fn make_move(&mut self, coords: &Coordinates) -> Result<(), failure::Error> {
        if coords.0 >= 15 || coords.1 >= 15 {
            Err(Error::CoordinatesOutOfBounds)?;
        }

        let cell = &mut self.board[coords.0][coords.1];

        if cell.is_some() {
            Err(Error::OccupiedCell)?;
        }

        *cell = self.current_turn;

        Ok(())
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
        for x in 0..self.board.width() {
            let (mut tracking, mut streak) = (None, 0);

            for y in 0..self.board.height() {
                let cell = self.board[y][x];

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

    fn diagonal_cell_search(&self, coords: &Coordinates, reverse: bool) -> Option<EndGame> {
        let (mut x, mut y) = (coords.0, coords.1);

        let tracking = self.board[y][x]?;
        let mut streak = 1;

        let d: i32 = if reverse { -1 } else { 1 };

        for _ in 0..VICTORY_STREAK - 1 {
            x = (x as i32 + d) as usize;
            y += 1;

            if self.board[y][x]? == tracking {
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

        for x in 0..height + 1 - VICTORY_STREAK {
            for y in 0..width {
                let coords = Coordinates(x, y);

                if y <= width - VICTORY_STREAK {
                    if let Some(end) = self.diagonal_cell_search(&coords, false) {
                        return Some(end);
                    }
                }

                if y >= VICTORY_STREAK - 1 {
                    if let Some(end) = self.diagonal_cell_search(&coords, true) {
                        return Some(end);
                    }
                }
            }
        }

        None
    }
}
