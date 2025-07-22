#[cfg(test)]
use super::*;

#[test]
fn data_enum_piece_function_return_correct() {
    let piece: Piece = Piece::O;
    let new_piece: Piece = piece.get_other_piece();
    assert!(matches!(new_piece, Piece::X));

    let piece: Piece = Piece::O;
    let new_piece: Piece = piece.get_other_piece();
    assert!(matches!(new_piece, Piece::X));

    let piece: Piece = Piece::None;
    let new_piece: Piece = piece.get_other_piece();
    assert!(matches!(new_piece, Piece::None));
}

#[test]
fn utils_check_status_function()
{
    /*
        Horizontal (row) test
     */
    let mut test_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::X,Piece::X,Piece::X],
                    [Piece::None,Piece::None,Piece::None]],
        score : 0,
        current_piece : Piece::None,
    };

    let mut winner = check_status(&test_board);
    assert!(matches!(winner, Piece::X));

    test_board.positions = [[Piece::X,Piece::X,Piece::None],
                            [Piece::None,Piece::None,Piece::None],
                            [Piece::O,Piece::O,Piece::O]];

    winner = check_status(&test_board);
    assert!(matches!(winner, Piece::O));

    /*
        Vertical (column) test
     */

    test_board.positions = [[Piece::X,Piece::O,Piece::None],
                            [Piece::None,Piece::O,Piece::None],
                            [Piece::O,Piece::O,Piece::X]];

    winner = check_status(&test_board);
    assert!(matches!(winner, Piece::O));

    test_board.positions = [[Piece::X,Piece::X,Piece::None],
                            [Piece::X,Piece::None,Piece::None],
                            [Piece::X,Piece::O,Piece::O]];

    winner = check_status(&test_board);
    assert!(matches!(winner, Piece::X));

    /*
        Oblique
     */
    test_board.positions = [[Piece::X,Piece::X,Piece::O],
                            [Piece::None,Piece::O,Piece::None],
                            [Piece::O,Piece::O,Piece::X]];

    winner = check_status(&test_board);
    assert!(matches!(winner, Piece::O));

    test_board.positions = [[Piece::X,Piece::X,Piece::None],
                            [Piece::None,Piece::X,Piece::None],
                            [Piece::O,Piece::O,Piece::X]];

    winner = check_status(&test_board);
    assert!(matches!(winner, Piece::X));

}



#[test]
fn utils_check_blocker_function()
{
    let mut test_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::X,Piece::O,Piece::X],
                    [Piece::None,Piece::None,Piece::None]],
        score : 0,
        current_piece : Piece::None,
    };

    let blocker = check_blocker(&test_board, 1,1);
    assert_eq!(blocker, true);

    test_board.positions = [[Piece::X,Piece::X,Piece::O],
                            [Piece::None,Piece::O,Piece::None],
                            [Piece::X,Piece::O,Piece::X]];

    let blocker = check_blocker(&test_board, 2,0);
    assert_eq!(blocker, true);

    test_board.positions = [[Piece::X,Piece::X,Piece::O],
                            [Piece::None,Piece::O,Piece::None],
                            [Piece::X,Piece::O,Piece::X]];

    let blocker = check_blocker(&test_board, 0,0);
    assert_eq!(blocker, false);   

}

#[test]
fn utils_diver_function()
{
    /* Instant blocker */
    let mut test_board = Board {
        positions : [[Piece::O,Piece::O,Piece::None],
                    [Piece::None,Piece::X,Piece::X],
                    [Piece::None,Piece::None,Piece::None]],
        score : 0,
        current_piece : Piece::X,
    };
    let score = dive(&test_board, &test_board.current_piece, 0,2, 1);
    println!("score : {}", score);
    assert_eq!(score, 900);

    /* Instant win */
    test_board.positions = [[Piece::O,Piece::O,Piece::None],
                            [Piece::None,Piece::X,Piece::X],
                            [Piece::None,Piece::None,Piece::None]];

    let score = dive(&test_board, &test_board.current_piece, 1,0, 1);
    println!("score : {}", score);
    assert_eq!(score, 900);

    test_board.positions = [[Piece::O,Piece::O,Piece::None],
                            [Piece::None,Piece::X,Piece::X],
                            [Piece::None,Piece::None,Piece::None]];

    let score = dive(&test_board, &test_board.current_piece, 2,0, 1);
    println!("score : {}", score);
    assert_eq!(score, 450);

    test_board.positions = [[Piece::O,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None]];
    let score = dive(&test_board, &test_board.current_piece, 1,1, 1);
    println!("score : {}", score);
    assert_eq!(score, 300);

}

#[test]
fn computer_vs_computer() {
    use std::env;
    let mut sleep_duration = Duration::default();

    // Detect command line arguments after -- e.g. cargo test -- --nocapture
    // Here --nocapture will be detected and thus a delay will be inserted so
    // we can see the computer playing the game with itself as opponent
    let args: Vec<String> = env::args().collect();
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("--nocapture")) {
        sleep_duration = Duration::new(1,0);
    }

    /* Instant blocker */
    let mut test_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None]],
        score : 0,
        current_piece : Piece::X,
    };
    let mut done : bool;
    let mut winner : Piece;
    use std::time::Duration;
    loop {
        done = get_next_move(&mut test_board);
        winner = check_status(&test_board);
        test_board.display_board(done, &winner);
        if done || matches!(winner, Piece::O | Piece::X) { break };
        test_board.current_piece = test_board.current_piece.get_other_piece();
        std::thread::sleep(sleep_duration);
    } 
    assert!(matches!(winner, Piece::None)); // No winners
}

#[test]
#[ignore = "wip"] // use -- --ignored to cargo test to run this test
fn back_prop_function() {

    let a_input: Vec<i8> = vec![
        0, 0, 1, 1, 0, 0, 
        0, 1, 0, 0, 1, 0,
        1, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 1];

    let a_output: Vec<i8> = vec![1, 0, 0];

    let result = back_prop(a_input, a_output);

    assert_eq!(result, 1f32/(1f32 + (-2f32).exp()));
}