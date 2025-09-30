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
fn full_function()
{

    let mut test_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::X,Piece::X,Piece::X],
                    [Piece::None,Piece::None,Piece::None]],
        score : 0,
        computer_piece : Piece::None,
    };

    assert_eq!(test_board.full(), false);

    test_board.positions = [[Piece::X,Piece::X,Piece::X],
                            [Piece::X,Piece::O,Piece::O],
                            [Piece::O,Piece::X,Piece::O]];
    
    assert_eq!(test_board.full(), true);
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
        computer_piece : Piece::None,
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
        computer_piece : Piece::None,
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
        computer_piece : Piece::X,
    };
    let (score, _) = dive(&test_board, &test_board.computer_piece, 0,2, 1, false);
    println!("score : {}", score);
    assert_eq!(score, 895);

    /* Instant win */
    test_board.positions = [[Piece::O,Piece::O,Piece::None],
                            [Piece::None,Piece::X,Piece::X],
                            [Piece::None,Piece::None,Piece::None]];

    let (score, _) = dive(&test_board, &test_board.computer_piece, 1,0, 1, false);
    println!("score : {}", score);
    assert_eq!(score, 900);

    test_board.positions = [[Piece::O,Piece::O,Piece::None],
                            [Piece::None,Piece::X,Piece::X],
                            [Piece::None,Piece::None,Piece::None]];

    let (score, _) = dive(&test_board, &test_board.computer_piece, 2,0, 1, false);
    println!("score : {}", score);
    assert_eq!(score, 445);

    test_board.positions = [[Piece::O,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None]];
    let (score, _) = dive(&test_board, &test_board.computer_piece, 1,1, 1, false);
    println!("score : {}", score);
    assert_eq!(score, 295);


}

/* Add "-- debug --nocapture" to show search tree */
#[test]
fn get_next_move_instant_win()
{
    /* Test that computer chooses instant win instead of blocker */

    let mut test_board: Board = Board {
        positions : [[Piece::O,Piece::X,Piece::X],
                    [Piece::X,Piece::X,Piece::None],
                    [Piece::O,Piece::O,Piece::None]],
        score : 0,
        computer_piece: Piece::O
    };

    let mut debug = false;
    use std::env;
    let args: Vec<String> = env::args().collect();
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("debug")) {
        debug = true;
    }

    get_next_move(&mut test_board, debug);
    let winner = check_status(&test_board);
    let done = test_board.full();
    test_board.display_board(done, &winner);
    assert_eq!(winner, Piece::O);
}

/* Add "-- debug --nocapture" to show search tree */
#[test]
fn get_next_move_future_best()
{
    /* Test that computer chooses position that will block certain user win in future */

    let mut test_board: Board = Board {
        positions : [[Piece::O,Piece::X,Piece::None],
                    [Piece::None,Piece::X,Piece::None],
                    [Piece::None,Piece::O,Piece::None]],
        score : 0,
        computer_piece: Piece::X
    };

    let mut debug = false;
    use std::env;
    let args: Vec<String> = env::args().collect();
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("debug")) {
        debug = true;
    }

    get_next_move(&mut test_board, debug);
    let winner = check_status(&test_board);
    let done = test_board.full();
    test_board.display_board(done, &winner);
    assert_eq!(test_board.positions[2][0], Piece::X);
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
    let mut debug = false;
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("debug")) {
        debug = true;
    }
    /* Instant blocker */
    let mut test_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None]],
        score : 0,
        computer_piece : Piece::X,
    };
    let mut done : bool;
    let mut winner : Piece;
    use std::time::Duration;
    loop {
        get_next_move(&mut test_board, debug);
        winner = check_status(&test_board);
        done = test_board.full();
        test_board.display_board(done, &winner);
        if done || matches!(winner, Piece::O | Piece::X) { break };
        test_board.computer_piece = test_board.computer_piece.get_other_piece();
        std::thread::sleep(sleep_duration);
    } 
    assert!(matches!(winner, Piece::None)); // No winners
}

#[test]
//#[ignore = "wip"] // use -- --ignored to cargo test to run this test
fn neural_functions() {
    use crate::neural_utils::*;   
    use crate::neural_data::*; 
    use approx::assert_abs_diff_eq;
    
    let alpha = 0.1;

    // Create a mutable version of the original random weight matrixes
    let mut w1: Vec<Vec<f64>> = W1.iter().map(|row_ref| row_ref.to_vec()).collect();
    let mut w2: Vec<Vec<f64>> = W2.iter().map(|row_ref| row_ref.to_vec()).collect();
    // Create a shadow variable pointing to the mutable version of the matrixes
    // where the number of rows is not changeable, but the rows themselves are slices that
    // point to memory that can be modified
    let mut w1: Vec<&mut [f64]> = w1.iter_mut().map(|r| r.as_mut_slice()).collect();
    let mut w2: Vec<&mut [f64]> = w2.iter_mut().map(|r| r.as_mut_slice()).collect();
    
    back_prop(A_INPUT, A_OUTPUT, &mut w1, &mut w2, alpha);

    for (index, result_row) in w1.iter().enumerate() {
        assert_abs_diff_eq!(&result_row[..], &W1_BACK_PROP_1[index][..], epsilon=0.0001);
    }

    for (index, result_row) in w2.iter().enumerate() {
        assert_abs_diff_eq!(&result_row[..], &W2_BACK_PROP_1[index][..], epsilon=0.0001);
    }

    back_prop(B_INPUT, B_OUTPUT, &mut w1, &mut w2, alpha);
    back_prop(C_INPUT, C_OUTPUT, &mut w1, &mut w2, alpha);

    for _ in 0..100 {
        back_prop(A_INPUT, A_OUTPUT, &mut w1, &mut w2, alpha);
        back_prop(B_INPUT, B_OUTPUT, &mut w1, &mut w2, alpha);
        back_prop(C_INPUT, C_OUTPUT, &mut w1, &mut w2, alpha); 
    }

    let mut out;
    let mut losss;
    let mut pos;

    out = forward(A_INPUT, &w1, &w2);
    losss = loss(A_OUTPUT, &out);
    assert_abs_diff_eq!(losss, 0.007450141319513159, epsilon=1e-10);
    pos = find_largest_index(&out);
    assert!(pos == 0, "Failed to guess A");
    println!("Guessed {}", A_B_C[pos]);
    
    out = forward(B_INPUT, &w1, &w2);
    losss = loss(B_OUTPUT, &out);
    assert_abs_diff_eq!(losss, 0.008544915933039758, epsilon=1e-10);
    pos = find_largest_index(&out);
    assert!(pos == 1, "Failed to guess B");
    println!("Guessed {}", A_B_C[pos]);

    out = forward(C_INPUT, &w1, &w2);
    losss = loss(C_OUTPUT, &out);
    assert_abs_diff_eq!(losss, 0.010083027388383743, epsilon=1e-10);
    pos = find_largest_index(&out);
    assert!(pos == 2, "Failed to guess C");
    println!("Guessed {}", A_B_C[pos]);

    out = forward(A_ERROR, &w1, &w2);
    losss = loss(A_OUTPUT, &out);
    assert_abs_diff_eq!(losss, 0.01035458738888999, epsilon=1e-10);
    pos = find_largest_index(&out);
    assert!(pos == 0, "Failed to guess A");
    println!("Guessed {}", A_B_C[pos]);
}

/* Add "-- debug --nocapture" to show search tree */
#[test]
fn neural_test_1()
{
    /* Test that computer chooses position that will block certain user win in future */
    
    /*
    let mut test_board: Board = Board {
        positions : [[Piece::O,Piece::X,Piece::None],
                    [Piece::None,Piece::X,Piece::None],
                    [Piece::None,Piece::O,Piece::None]],
        score : 0,
        computer_piece: Piece::X
    };
    */
    
    // Since computer is X, make all O=1, and all X=2
    // Input array then becomes 1, 2, 0, 0, 2, 0, 0, 1, 0
    // Ouput array must then be 3,0

    let mut _debug = false;
    use std::env;
    let args: Vec<String> = env::args().collect();
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("debug")) {
        _debug = true;
    }
    use crate::neural_utils::*;  
    let alpha = 0.1;
    let input_array: [i8; 9] = [1, 2, 0, 0, 2, 0, 0, 1, 0];
    let input_array_off: [i8; 9] = [1, 2, 0, 0, 2, 0, 2, 1, 0];
    let output_array: [i8; 2] = [1, 0];

    let mut w1: &mut [&mut [f64]] = &mut [
        &mut [1.4471, 0.1089, -0.5946, -0.3561, 1.0164, ],
        &mut [0.5838, -1.2973, 1.6353, -0.1877, 0.5103, ],
        &mut [-1.2306, 0.4537, 1.0708, 0.1518, 1.2684, ],
        &mut [0.1237, -0.5785, -1.7884, -0.2588, -0.6410, ],
        &mut [-1.0211, -1.3709, 0.7936, 0.9652, -0.2878, ],
        &mut [0.8478, -1.2553, 0.2006, 0.4499, -1.1906, ],
        &mut [0.0827, -0.5795, 0.1719, 0.3833, -0.4288, ],
        &mut [-2.1429, -1.0544, 0.6255, -1.4042, 0.8119, ],
        &mut [-0.6639, 0.2168, -0.2936, 0.4671, -0.5450, ]];

    
    let mut w2: &mut [&mut [f64]] = &mut [
        &mut [1.1697, 1.5113],
        &mut [-0.8300, 0.0493],
        &mut [0.3861, -2.4398],
        &mut [-0.6741, 0.1552],
        &mut [-0.9879, -0.8454]];

    for _ in 0..20 {
        back_prop(&input_array, &output_array, &mut w1, &mut w2, alpha);
    }

    let mut out = forward(&input_array, &w1, &w2);
    let mut losss = loss(&output_array, &out);
    println!("Loss : {}, out : {:?}", losss, out);

    out = forward(&input_array_off, &w1, &w2);
    losss = loss(&output_array, &out);
    println!("Loss : {}, out : {:?}", losss, out);

    // assert_eq!(test_board.positions[2][0], Piece::X);
}

// cargo test gaussian_matrix_test
// cargo test gaussian_matrix_test -- --nocapture
#[test]
fn gaussian_matrix_test() {
    use crate::neural_utils::*;
    let rows: usize = 5;
    let columns: usize = 3;
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; columns]; rows]; 
    let mut matrix: Vec<&mut [f64]> = matrix.iter_mut().map(|r| r.as_mut_slice()).collect();
    gaussian_matrix(columns as i8, rows as i8, &mut matrix);

    println!(" {:?}", matrix);
    for row in 0..rows {
        for column in 0..columns {
            assert!(matrix[row][column] > -2.0 && matrix[row][column] < 2.0 && matrix[row][column] !=  0.0, "Number is not within range -2.0 to 2.0, or equal to 0.0");
        }
    }
}

#[test]
//#[ignore = "wip"] // use -- --ignored to cargo test to run this test
fn neural_tic_tac_toe_train() {
    use crate::neural_utils::*;   
    use crate::neural_data::*; 
    //use approx::assert_abs_diff_eq;
    
    let alpha = 0.1;

    // Generate W_Out, output weight matrix, number of rows must be equal to previous number
    // of nodes. Number of columns must be equal to number of output nodes, which i 3x3=9
    let rows: usize = 15;
    let columns: usize = 9;
    let mut w_out: Vec<Vec<f64>> = vec![vec![0.0; columns]; rows]; 
    let mut w_out: Vec<&mut [f64]> = w_out.iter_mut().map(|r| r.as_mut_slice()).collect();
    gaussian_matrix(columns as i8, rows as i8, &mut w_out);
    println!(" {:?}", w_out);

    // Create a mutable version of the original random weight matrixes
    let mut w1: Vec<Vec<f64>> = W1.iter().map(|row_ref| row_ref.to_vec()).collect();
    let mut w2: Vec<Vec<f64>> = W2.iter().map(|row_ref| row_ref.to_vec()).collect();
    // Create a shadow variable pointing to the mutable version of the matrixes
    // where the number of rows is not changeable, but the rows themselves are slices that
    // point to memory that can be modified
    let mut w1: Vec<&mut [f64]> = w1.iter_mut().map(|r| r.as_mut_slice()).collect();
    let mut w2: Vec<&mut [f64]> = w2.iter_mut().map(|r| r.as_mut_slice()).collect();
    
    back_prop(A_INPUT, A_OUTPUT, &mut w1, &mut w2, alpha);


}
