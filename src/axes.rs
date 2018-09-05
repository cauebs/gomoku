use std::mem::replace;

use board::{Board, Coord};
use game::PlayerIndicator;

#[derive(Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
    BottomLeftDiagonal,
    TopLeftDiagonal,
    BottomRightDiagonal,
    TopRightDiagonal,
}

pub struct Axis<'a> {
    board: &'a Board,
    direction: Direction,
    axis_index: usize,
    cell_index: usize,
}

impl<'a> Axis<'a> {
    fn new(board: &'a Board, direction: Direction, axis_index: usize) -> Self {
        Self {
            board,
            direction,
            axis_index,
            cell_index: 0,
        }
    }
}

impl<'a> Iterator for Axis<'a> {
    type Item = (Coord, Option<PlayerIndicator>);

    fn next(&mut self) -> Option<Self::Item> {
        use self::Direction::*;

        let axis_index = self.axis_index as isize;
        let cell_index = self.cell_index as isize;

        let (i, j) = match self.direction {
            Horizontal => (axis_index, cell_index),
            Vertical => (cell_index, axis_index),
            BottomLeftDiagonal => (14 - axis_index + cell_index, cell_index),
            TopLeftDiagonal => (axis_index - cell_index, cell_index),
            TopRightDiagonal => (axis_index + cell_index, 14 - cell_index),
            BottomRightDiagonal => (14 - axis_index - cell_index, 14 - cell_index),
        };

        self.cell_index += 1;

        if let (0...14, 0...14) = (i, j) {
            let (i, j) = (i as usize, j as usize);
            Some(((i, j), self.board[i][j]))
        } else {
            None
        }
    }
}

pub struct Axes<'a> {
    board: &'a Board,
    direction: Direction,
    axis_index: usize,
    axis: Option<Axis<'a>>,
}

impl<'a> Axes<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            direction: Direction::Horizontal,
            axis_index: 0,
            axis: Some(Axis::new(board, Direction::Horizontal, 0)),
        }
    }
}

impl<'a> Iterator for Axes<'a> {
    type Item = Axis<'a>;

    fn next(&mut self) -> Option<Axis<'a>> {
        use self::Direction::*;

        self.axis_index += 1;
        if self.axis_index > 15 {
            self.axis_index = 0;
            if let BottomRightDiagonal = self.direction {
                return self.axis.take();
            }

            self.direction = match self.direction {
                Horizontal => Vertical,
                Vertical => BottomLeftDiagonal,
                BottomLeftDiagonal => TopLeftDiagonal,
                TopLeftDiagonal => TopRightDiagonal,
                TopRightDiagonal => BottomRightDiagonal,
                BottomRightDiagonal => Horizontal,
            };
        }

        replace(
            &mut self.axis,
            Some(Axis::new(self.board, self.direction, self.axis_index)),
        )
    }
}
