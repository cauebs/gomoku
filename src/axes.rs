#![allow(dead_code)]
use std::{collections::HashSet, mem::replace};

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

#[derive(Clone)]
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

    pub fn streaks(self, player: PlayerIndicator) -> HashSet<Vec<Coord>> {
        let mut current = Vec::new();
        let mut streaks = HashSet::new();

        for (coord, cell) in self {
            if cell == Some(player) {
                current.push(coord);
                continue;
            }

            if !current.is_empty() {
                streaks.insert(current);
            }
            current = Vec::new();
        }

        if !current.is_empty() {
            streaks.insert(current);
        }

        streaks
    }

    pub fn streaks_with_room(mut self, player: PlayerIndicator) -> HashSet<Vec<Coord>> {
        let mut preceding_spaces = 0;
        let mut current = Vec::new();
        let mut streaks = HashSet::new();

        while let Some((coord, cell)) = self.next() {
            if cell == Some(player) {
                current.push(coord);
                continue;
            }
            if !current.is_empty() {
                if current.len() + preceding_spaces >= 5 {
                    streaks.insert(current);
                } else {
                    let axis = self.clone();
                    let succeding_spaces = axis.take_while(|(_, cell)| cell.is_none()).count();
                    if current.len() + preceding_spaces + succeding_spaces >= 5 {
                        streaks.insert(current);
                    }
                }
            }

            current = Vec::new();

            if cell.is_none() {
                preceding_spaces += 1;
            } else {
                preceding_spaces = 0;
            }
        }

        if !current.is_empty() {
            streaks.insert(current);
        }

        streaks
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

    pub fn streaks(self, player: PlayerIndicator) -> HashSet<Vec<Coord>> {
        self.map(|axis| axis.streaks(player)).flatten().collect()
    }

    pub fn streaks_with_room(self, player: PlayerIndicator) -> HashSet<Vec<Coord>> {
        self.map(|axis| axis.streaks_with_room(player))
            .flatten()
            .collect()
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
