use crate::HanoiGame;

#[test]
fn test_simple(){
    let n = 3;
    let mut game = HanoiGame::new(n);
    assert_eq!(game.is_complete(), false);
    let result = game.move_discs(n, 0, 2, 1);
    assert_eq!(result, Ok(()));
    assert_eq!(game.is_complete(), true);
}

#[test]
fn test_no_disc(){
    let mut game = HanoiGame::new(3);
    let result = game.move_disc(1, 2);
    assert_eq!(result, Err("No disc to move"));
    assert_eq!(game.is_complete(), false);
}

#[test]
fn test_invalid_rod(){
    let mut game = HanoiGame::new(3);
    let result = game.move_disc(3, 2);
    assert_eq!(result, Err("Invalid rod index"));
    assert_eq!(game.is_complete(), false);
}

#[test]
fn test_larger_on_top(){
    let mut game = HanoiGame::new(3);
    let result = game.move_disc(0, 1);
    assert_eq!(result, Ok(()));
    let result = game.move_disc(0, 1);
    assert_eq!(result, Err("Cannot place a larger disc on top of a smaller one"));
    assert_eq!(game.is_complete(), false);
}

#[test]
fn test_complete(){
    let game = {
        let num_discs = 3;
        HanoiGame {
            num_discs,
            rods: [
                Vec::new(),
                Vec::new(),
                (1..=num_discs).rev().collect::<Vec<_>>(),
            ],
        }
    };
    game.print_game();
    assert_eq!(game.is_complete(), true);
}