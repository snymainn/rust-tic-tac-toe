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
    use crate::neural_utils::*;   
    use crate::neural_data::*; 
    use approx::assert_abs_diff_eq;

    let mut result_vector: Vec<Vec<f64>> = Vec::new();
    // Create a mutable version of the original random weight matrixes
    let mut w1: Vec<Vec<f64>> = W1.iter().map(|row_ref| row_ref.to_vec()).collect();
    let mut w2: Vec<Vec<f64>> = W2.iter().map(|row_ref| row_ref.to_vec()).collect();
    // Create a shadow variable pointing to the mutable version of the matrixes
    // where the number of rows is not changeable, but the rows themselves are slices that
    // point to memory that can be modified
    let mut w1: Vec<&mut [f64]> = w1.iter_mut().map(|r| r.as_mut_slice()).collect();
    let mut w2: Vec<&mut [f64]> = w2.iter_mut().map(|r| r.as_mut_slice()).collect();
    
    back_prop(A_INPUT, A_OUTPUT, &mut w1, &mut w2, 0.1);

    for (index, result_row) in w1.iter().enumerate() {
        assert_abs_diff_eq!(&result_row[..], &W1_BACK_PROP_1[index][..], epsilon=0.0001);
    }

    for (index, result_row) in w2.iter().enumerate() {
        assert_abs_diff_eq!(&result_row[..], &W2_BACK_PROP_1[index][..], epsilon=0.0001);
    }

}    