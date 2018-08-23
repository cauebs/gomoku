use coordinates::Coordinates;
use game::{EndGame, Game, PlayerIndicator::*};
use players::TestBot;

#[test]
fn test_horizontal_wrap() {
    let p1 = TestBot::new(vec![
        Coordinates(0, 11),
        Coordinates(0, 12),
        Coordinates(0, 13),
        Coordinates(0, 14),
        Coordinates(1, 0),
    ]);

    let p2 = TestBot::new(vec![
        Coordinates(0, 14),
        Coordinates(1, 0),
        Coordinates(1, 1),
        Coordinates(1, 2),
        Coordinates(1, 3),
    ]);

    assert_eq!(Game::new(p1, p2).check_end(), None);
}

#[test]
fn test_vertical_wrap() {
    let p1 = TestBot::new(vec![
        Coordinates(11, 0),
        Coordinates(12, 0),
        Coordinates(13, 0),
        Coordinates(14, 0),
        Coordinates(0, 1),
    ]);

    let p2 = TestBot::new(vec![
        Coordinates(14, 0),
        Coordinates(0, 1),
        Coordinates(1, 1),
        Coordinates(2, 1),
        Coordinates(3, 1),
    ]);

    assert_eq!(Game::new(p1, p2).check_end(), None);
}

#[test]
fn test_horizontal_victory() {
    let p1 = TestBot::new(vec![
        Coordinates(11, 0),
        Coordinates(12, 0),
        Coordinates(13, 0),
        Coordinates(14, 0),
        Coordinates(0, 1),
    ]);

    let p2 = TestBot::new(vec![
        Coordinates(14, 0),
        Coordinates(0, 1),
        Coordinates(1, 1),
        Coordinates(2, 1),
        Coordinates(3, 1),
    ]);

    assert_eq!(Game::new(p1, p2).check_end(), None);
}
