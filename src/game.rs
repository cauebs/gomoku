use board::Board;
use players::{Player, PlayerIndicator};

pub enum EndGame {
    Victory(PlayerIndicator),
    Draw,
}

enum Error {
    ColumnOverflow,
    ColumnOutOfBounds,
}

#[derive(Debug)]
pub struct Game<P1: Player, P2: Player> {
    board: Board,
    player1: P1,
    player2: P2,
    current_turn: PlayerIndicator,
}

impl<P1: Player, P2: Player> Game<P1, P2> {
    pub fn new(player1: P1, player2: P2) -> Self {
        Self {
            player1,
            player2,
            board: Default::default(),
            current_turn: PlayerIndicator::P1,
        }
    }

    pub fn play(&mut self) -> EndGame {
        loop {
            let (column, next_turn) = match self.current_turn {
                PlayerIndicator::P1 => (self.player1.decide(&self.board), PlayerIndicator::P2),
                PlayerIndicator::P2 => (self.player2.decide(&self.board), PlayerIndicator::P1),
            };

            if self.make_move(column).is_err() {
                println!("Jogada inválida.\n");
                continue;
            }

            println!("\n{}", self.board);

            if let Some(end_game) = self.has_ended() {
                return end_game;
            }

            self.current_turn = next_turn;
        }
    }

    fn make_move(&mut self, column: usize) -> Result<(), Error> {
        if column >= 7 {
            return Err(Error::ColumnOutOfBounds);
        }

        let first_filled_row = self
            .board
            .iter()
            .map(|row| &row[column])
            .position(|cell| cell.is_some());

        match first_filled_row {
            Some(0) => return Err(Error::ColumnOverflow),
            Some(n) => self.board[n - 1][column] = Some(self.current_turn),
            None => self.board[5][column] = Some(self.current_turn),
        }

        Ok(())
    }

    fn has_ended(&self) -> Option<EndGame> {
        // TODO: detect victory states

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
}
