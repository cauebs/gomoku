use std::collections::HashSet;

use game::{EndGame::*, Game, PlayerIndicator::*, possible_moves};
use players::TestBot;
use board::Coord;

#[test]
fn test_horizontal_wrap() {
    let p1 = TestBot::new(vec![(0, 11), (0, 12), (0, 13), (0, 14), (1, 0)]);

    let p2 = TestBot::new(vec![(1, 14), (2, 0), (2, 1), (2, 2), (2, 3)]);

    assert_eq!(Game::new(p1, p2).play_turns(10), None);
}

#[test]
fn test_vertical_wrap() {
    let p1 = TestBot::new(vec![(11, 0), (12, 0), (13, 0), (14, 0), (0, 1)]);

    let p2 = TestBot::new(vec![(14, 1), (0, 2), (1, 2), (2, 2), (3, 2)]);

    assert_eq!(Game::new(p1, p2).play_turns(10), None);
}

#[test]
fn test_horizontal_victory() {
    let p1 = TestBot::new(vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]);

    let p2 = TestBot::new(vec![(0, 5), (0, 7), (0, 9), (0, 11), (0, 13)]);

    assert_eq!(
        Game::new(p1, p2).play_turns(10),
        Some(Victory(Player1))
    );
}

#[test]
fn test_vertical_victory() {
    let p1 = TestBot::new(vec![(1, 2), (3, 4), (5, 6), (7, 8), (9, 10)]);

    let p2 = TestBot::new(vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);

    assert_eq!(
        Game::new(p1, p2).play_turns(10),
        Some(Victory(Player2))
    );
}

#[test]
fn test_diagonal_victory() {
    let p1_moves = vec![(14, 0), (14, 2), (14, 4), (14, 6), (14, 8)];

    let p1 = TestBot::new(p1_moves.clone());

    let p2 = TestBot::new(vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)]);

    assert_eq!(
        Game::new(p1, p2).play_turns(10),
        Some(Victory(Player2))
    );

    let p1 = TestBot::new(p1_moves.clone());

    let p2 = TestBot::new(vec![(0, 14), (1, 13), (2, 12), (3, 11), (4, 10)]);

    assert_eq!(
        Game::new(p1, p2).play_turns(10),
        Some(Victory(Player2))
    );

    let p1 = TestBot::new(p1_moves.clone());

    let p2 = TestBot::new(vec![(1, 5), (2, 6), (3, 7), (4, 8), (5, 9)]);

    assert_eq!(
        Game::new(p1, p2).play_turns(10),
        Some(Victory(Player2))
    );
}

#[test]
fn test_possible_moves() {
    let p1_moves = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)];
    let p2_moves = vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)];

    let p1 = TestBot::new(p1_moves.clone());
    let p2 = TestBot::new(p2_moves.clone());

    let mut game = Game::new(p1, p2);

    let mut expected = HashSet::new();
    for i in 0..15 {
        for j in 0..15 {
            expected.insert((i, j));
        }
    }

    assert_eq!(
        possible_moves(&game),
        expected
    );

    for (p1_move, p2_move) in p1_moves.iter().zip(&p2_moves) {
        for p in [p1_move, p2_move].iter() {
            game.play_turns(1);
            expected.remove(p);
            assert_eq!(
                possible_moves(&game),
                expected
            );
        }
    }
}
