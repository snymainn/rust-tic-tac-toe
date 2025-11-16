#[cfg(test)]
use crate::neural_struct::TicTacToeNeuralNet;

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
fn flatten_board_function()
{

    let test_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::X,Piece::X,Piece::X],
                    [Piece::None,Piece::None,Piece::O]],
        score : 0,
        computer_piece : Piece::X,
    };
    let expected_result = [0, 0, 0, 1, 1, 1, 0, 0, -1];
    assert_eq!(test_board.flatten_board(), expected_result);
}


#[test]
fn reshape_board_function()
{
    let input_vector = [0, 0, 0, 1, 1, 1, 0, 0, -1];

    let mut input_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                [Piece::None,Piece::None,Piece::None],
                [Piece::None,Piece::None,Piece::None]],
        score : 0,
        computer_piece : Piece::X,
    };

    let expected_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::X,Piece::X,Piece::X],
                    [Piece::None,Piece::None,Piece::O]],
        score : 0,
        computer_piece : Piece::X,
    };
    input_board.reshape_board(input_vector);
    assert_eq!(input_board, expected_board);
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

    test_board.positions = [[Piece::None,Piece::O,Piece::X],
                            [Piece::None,Piece::O,Piece::X],
                            [Piece::None,Piece::None,Piece::X]];

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
// Test neural function with the ABC letters
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

// cargo test neural_tic_tac_toe_train -- --nocapture [debug] [readkey] [rounds=<training rounds>]
// add debug to get more debug printout
// --nocapture will add 1 sec delay between each move 
// add readkey to stop and require enter to continue
// add rounds to change from default five training rounds
#[test]
//#[ignore = "wip"] // use -- --ignored to cargo test to run this test
fn neural_func_utils() {
    use crate::neural_utils::*;   
    use std::env;
    use std::time::Duration;
    let mut sleep_duration = Duration::default();
    let mut perfect_play: Vec<[i8;9]> = vec![];
    let mut readkey_input = String::new();

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
    let mut readkey = false;
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("readkey")) {
        readkey = true;
        debug = true;
    }
    let mut rounds: i32 = 5;
    if let Some(round_input) = args.iter().find(|&&ref a| a.starts_with("rounds")) {
        let parts = round_input.split_once("=");
        let num_str = match parts {
            Some((_,value)) => value.trim(),
            None => {
                println!("Error: String {} does not contain =", round_input);
                return;
            }
        };
        rounds = num_str.parse().expect("Error: Value is not integer");
    }
    
    let alpha = 0.1;

    //
    // GENERATE SYNAPSE MATRIXES
    //

    // Generate W_Out, output weight matrix, number of rows must be equal to previous number
    // of nodes. Number of columns must be equal to number of output nodes, which i 3x3=9
    let mut rows: usize = 15;
    let mut columns: usize = 9;
    let mut w_out: Vec<Vec<f64>> = vec![vec![0.0; columns]; rows]; 
    let mut w_out: Vec<&mut [f64]> = w_out.iter_mut().map(|r| r.as_mut_slice()).collect();
    gaussian_matrix(columns as i8, rows as i8, &mut w_out);
    //if debug {print_matrix(&w_out);}

    // Generate W_In, input weigth matrix, number of rows(y) must be equal to input nodes = 9
    // Number of columns (x) must be equal to number of nodes in next level = 15
    rows = 9;
    columns = 15;
    let mut w_in: Vec<Vec<f64>> = vec![vec![0.0; columns]; rows]; // Init dynamic matrix
    let mut w_in: Vec<&mut [f64]> = w_in.iter_mut().map(|r| r.as_mut_slice()).collect();
    gaussian_matrix(columns as i8, rows as i8, &mut w_in);
    //if debug {print_matrix(&w_in);}
    
    //
    // TRAIN NEURAL NET WITH TREE SEARCH GAME LOGIC (SIMPLE DEPTH FIRST)
    //
    let mut train_board: Board;

    for round in 1..=rounds {
        print!("\nTraining round {}, =>", round);
        train_board = Board {
            positions : [[Piece::None,Piece::None,Piece::None],
                        [Piece::None,Piece::None,Piece::None],
                        [Piece::None,Piece::None,Piece::None]],
            score : 0,                
            computer_piece : Piece::X,
        };
        let mut done : bool;
        let mut winner : Piece;
        print!(" loss : ");
        loop {
            let input_board = train_board.flatten_board();
            get_next_move(&mut train_board, false);
            let output_board = train_board.flatten_board();
            winner = check_status(&train_board);
            done = train_board.full();

            // Train on input and output boards
            //if test_board.computer_piece == Piece::X { // Activate this to only train on one Piece
            back_prop(&input_board, &output_board, &mut w_in, &mut w_out, alpha);
            //}
            // Display loss for last training round
            let out = forward(&input_board, &w_in, &w_out);
            let losss: f64 = loss(&output_board, &out);
            print!(" {:.2}", losss);
            if round >= rounds {
                // Add last round as a perfect play round
                perfect_play.push(output_board);
            }

            if done || matches!(winner, Piece::O | Piece::X) { break };
            train_board.computer_piece = train_board.computer_piece.get_other_piece();
        } 
    } 
    println!("");

    //
    // LET TRAINED NEURAL NET PLAY AGAINS TREE SEARCH GAME LOGIC
    //

    let mut test_board = Board {
        positions : [
            [Piece::None,Piece::None,Piece::None],
            [Piece::None,Piece::None,Piece::None],
            [Piece::None,Piece::None,Piece::None]],
        score : 0,
        computer_piece : Piece::X,
    };
    let mut done = false;
    let mut winner = Piece::None;

    // Reinit train board again for use to display
    // training play
    train_board = Board {
        positions : [
            [Piece::None,Piece::None,Piece::None],
            [Piece::None,Piece::None,Piece::None],
            [Piece::None,Piece::None,Piece::None]],
        score : 0,                
        computer_piece : Piece::X,
    };
    for turn in (0..9).step_by(2) {

        //
        // Neural net play
        //
        if debug { println!("Neural net move\n**************************"); }
        let input_board = test_board.flatten_board();
        if debug { println!("Input board: {:?}", input_board); }
        let out: Vec<f64> = forward(&input_board, &w_in, &w_out);
        let mut sorted_out: Vec<(f64,usize)> = out.into_iter().enumerate().map(|(i,v)| (v,i)).collect();
        sorted_out.sort_by(|a,b| b.0.partial_cmp(&a.0).unwrap());
        if debug {println!("Sorted out: {:?}",sorted_out);}
        let sorted_out_indexes: Vec<usize> = sorted_out.into_iter().map(|(_,i)| i).collect();
        if debug {println!("Sorted indexes: {:?}", sorted_out_indexes);}
        let mut output_board = input_board;
        let mut move_ok = false;
        for index in sorted_out_indexes {
            if output_board[index] == 0 {
                output_board[index] = 1;
                move_ok = true;
                break;
            }
        }
        if move_ok == false { panic!("No move available, should not be possible"); }       

        if debug { println!("Output board: {:?}", output_board); }
        test_board.reshape_board(output_board);
        winner = check_status(&test_board);
        if debug { test_board.display_board(done, &winner); }
        if done || matches!(winner, Piece::O | Piece::X) { break };

        if rounds > 0 && debug {
            println!("Perfect played board {:?}", perfect_play[turn]);
            train_board.reshape_board(perfect_play[turn]);
            train_board.display_board(done, &winner);
        }

        //
        // Tree search play
        //
        if debug { println!("Tree search move\n***************************"); }
        test_board.computer_piece = test_board.computer_piece.get_other_piece();
        get_next_move(&mut test_board, false);
        winner = check_status(&test_board);
        done = test_board.full();
        if debug { test_board.display_board(done, &winner);}
        if done || matches!(winner, Piece::O | Piece::X) { break };
        test_board.computer_piece = test_board.computer_piece.get_other_piece();

        if readkey && debug {
            println!("Press enter to continue...");
            let _ = std::io::stdin().read_line(&mut readkey_input);
        } else {
            if debug { std::thread::sleep(sleep_duration); }
        }

    }
    assert!(matches!(winner, Piece::None)); // No winners

}

#[test]
fn select_option_test() {
    use std::io::Cursor;

    let mock_input = "x\n";
    let keyboard_reader = Cursor::new(mock_input);

    let response = 
        select_option(&vec![("X", Some(Piece::X)), ("O", Some(Piece::O))], keyboard_reader);
    assert_eq!(response, Some(Piece::X));

    let mock_input = "r\n";
    let keyboard_reader = Cursor::new(mock_input);

    let response = 
        select_option(&vec![("X", Some(Piece::X)), ("O", Some(Piece::O))], keyboard_reader);
    assert_eq!(response, None);

    let mock_input = "\n";
    let keyboard_reader = Cursor::new(mock_input);

    let response = 
        select_option(&vec![("X", Some(Piece::X)), ("O", Some(Piece::O))], keyboard_reader);
    assert_eq!(response, None);
    
    let mock_input = "O\n";
    let keyboard_reader = Cursor::new(mock_input);

    let response = 
        select_option(&vec![("X", Some(Piece::X)), ("O", Some(Piece::O))], keyboard_reader);
    assert_eq!(response, Some(Piece::O));

    let mock_input = "T\n";
    let keyboard_reader = Cursor::new(mock_input);

    let response = 
        select_option(&vec![("T", Some(ComputerPlayerType::TreeSearch)), ("N", Some(ComputerPlayerType::Neural))], keyboard_reader);
    assert_eq!(response, Some(ComputerPlayerType::TreeSearch));

    let mock_input = "N\n";
    let keyboard_reader = Cursor::new(mock_input);

    let response = 
        select_option(&vec![("T", Some(ComputerPlayerType::TreeSearch)), ("N", Some(ComputerPlayerType::Neural))], keyboard_reader);
    assert_eq!(response, Some(ComputerPlayerType::Neural));
}

#[test]
fn neural_struct_play() {
    use std::env;
    let mut readkey_input = String::new();

    // Detect command line arguments after -- e.g. cargo test -- --nocapture
    // Here --nocapture will be detected and thus a delay will be inserted so
    // we can see the computer playing the game with itself as opponent
    let args: Vec<String> = env::args().collect();
 
    let mut debug = false;
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("debug")) {
        debug = true;
    }
    let mut readkey = false;
    if let Some(_any) = args.iter().find(|&&ref a| a.starts_with("readkey")) {
        readkey = true;
        debug = true;
    }
    let neural_play = TicTacToeNeuralNet::train(5);
    //neural_play.print_matrix(&neural_play.w_in);
    //neural_play.print_matrix(&neural_play.w_out);

    let mut test_board = Board {
        positions : [
            [Piece::None,Piece::None,Piece::None],
            [Piece::None,Piece::None,Piece::None],
            [Piece::None,Piece::None,Piece::None]],
        score : 0,
        computer_piece : Piece::X,
    };
    let mut done = false;
    let mut winner = Piece::None;

    for _ in (0..9).step_by(2) {

        //
        // Neural net play
        //
        if debug { println!("Neural net move\n**************************"); }
        neural_play.forward_wrapped(&mut test_board);
        winner = check_status(&test_board);
        if debug { test_board.display_board(done, &winner); }
        if done || matches!(winner, Piece::O | Piece::X) { break };

        //
        // Tree search play
        //
        if debug { println!("Tree search move\n***************************"); }
        test_board.computer_piece = test_board.computer_piece.get_other_piece();
        get_next_move(&mut test_board, false);
        winner = check_status(&test_board);
        done = test_board.full();
        if debug { test_board.display_board(done, &winner); }
        if done || matches!(winner, Piece::O | Piece::X) { break };
        test_board.computer_piece = test_board.computer_piece.get_other_piece();
        if readkey {
            println!("Press enter to continue...");
            let _ = std::io::stdin().read_line(&mut readkey_input);
        }
    }
    assert!(matches!(winner, Piece::None)); // No winners

    
}