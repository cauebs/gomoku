use failure;

use board::Board;
use coordinates::Coordinates;
use players::Player;

const VICTORY_STREAK: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerIndicator {
    Human,
    Bot,
}

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
pub struct Game<H: Player, B: Player> {
    board: Board,
    human: H,
    bot: B,
    current_turn: Option<PlayerIndicator>,
    turns: u32,
}

impl<H: Player, B: Player> Game<H, B> {
    pub fn new(human: H, bot: B) -> Self {
        Self {
            board: Default::default(),
            human,
            bot,
            current_turn: None,
            turns: 0,
        }
    }

    pub fn play(&mut self, first: PlayerIndicator) -> EndGame {
        use self::PlayerIndicator::{Bot, Human};

        self.current_turn = Some(first);

        loop {
            let (coords, next_turn) = match self.current_turn.unwrap() {
                Human => (self.human.decide(&self.board), Bot),
                Bot => (self.bot.decide(&self.board), Human),
            };

            if let Err(e) = self.make_move(&coords) {
                println!("Invalid move: {}\n", e);
                continue;
            }

            println!("\n{}", self.board);

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

    fn check_end(&self) -> Option<EndGame> {
        // TODO: detect diagonal victories

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
        let (mut tracking, mut streak) = (None, 0);

        for row in self.board.iter() {
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
            tracking = None;
        }

        None
    }

    fn check_for_vertical_victory(&self) -> Option<EndGame> {
        for j in 0..self.board.width() {
            let (mut tracking, mut streak) = (None, 0);
            for &cell in (0..self.board.height()).map(|i| &self.board[i][j]) {
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

    fn check_for_diagonal_victory(&self) -> Option<EndGame> {
        let height = self.board.height();
        let width = self.board.width();

        for j in 0..height + 1 - VICTORY_STREAK {
            for i in 0..width + 1 - VICTORY_STREAK {
                for dir in [-1, 1].iter() {
                    let (mut tracking, mut streak) = (None, 0);
                    for &cell in (0..VICTORY_STREAK)
                                 .map(|d| match dir {
                                     x if *x < 0 => &self.board[i + d][VICTORY_STREAK - 1 + j - d],
                                     _ => &self.board[i + d][j + d],
                                 }) {
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
            }
        }

        None
    }
}
