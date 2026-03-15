use rand::thread_rng;
use rand_distr::num_traits::One;
use rand_distr::{Normal, Distribution};
use crate::data::*;
use crate::utils::*;
use crate::neural_utils::*;
use approx::assert_abs_diff_eq;

pub struct TicTacToeNeuralNet {
    pub w_in: Vec<[f64; 15]>,
    pub w_out: Vec<[f64; 9]>,
    pub w_hidden: Option<Vec<[f64; 15]>>, 
    pub piece_that_should_be_one: Piece,
    pub test: Option<bool>
}

impl TicTacToeNeuralNet {    

    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn new_test(piece_that_should_be_one: Piece, hidden_layers : Option<bool>) -> Self {
        let mut net = Self {
            w_in : vec![[0.0; 15]; 9],
            w_out : vec![[0.0; 9]; 15],
            w_hidden : match hidden_layers {
                Some(true) => vec![[0.0; 15]; 15].into(),
                _ => None
            },
            piece_that_should_be_one : piece_that_should_be_one,
            test : Some(true)
        };
        //net.gaussian_matrix();
        let mut start_val = -0.9;
        for row in 0..net.w_in[0].len() {
            for column in 0..net.w_in.len() {
                net.w_in[column as usize][row as usize] = start_val + 0.1;
            }
            start_val += 0.1;
        }
        let mut start_val = -0.9;
        for row in 0..net.w_out[0].len() {
            for column in 0..net.w_out.len() {
                net.w_out[column as usize][row as usize] = start_val + 0.1;
            }
            start_val += 0.1;
        }
        let mut start_val = -0.9;
        if let Some(ref mut matrix) = net.w_hidden {
            for row in 0..matrix[0].len() {
                for column in 0..matrix.len() {
                    matrix[column as usize][row as usize] = start_val + 0.1;
                }
                start_val += 0.1;
            }
        }

        net
    }

    pub fn train(rounds: u8, piece_that_should_be_one: Piece) -> Self {
        let mut net = Self {
            w_in : vec![[0.0; 15]; 9],
            w_out : vec![[0.0; 9]; 15],
            w_hidden : None,
            piece_that_should_be_one : piece_that_should_be_one,
            test : Some(false)
        };
        net.gaussian_matrix();
        
        let mut train_board: Board;

        let mut loss_plot: DataToPlot = DataToPlot{ data : vec![], legend : "Loss".to_string()};

        for round in 1..=rounds {
            print!("\nTraining round {}, =>", round);
            train_board = Board {
                positions : [[Piece::None,Piece::None,Piece::None],
                            [Piece::None,Piece::X,Piece::None],
                            [Piece::None,Piece::None,Piece::None]],
                score : 0,                
                computer_piece : Piece::O,
            };
            let mut done : bool;
            let mut winner : Piece;
            print!(" loss : ");

            // Train with first obvious move
            let mut input_board :[i8; 9] = [0; 9];
            let mut output_board = train_board.flatten_board(Some(&Piece::X));
            for _ in 0..3 {
                net.back_prop(&input_board, &output_board, 0.1);
            }

            loop {
                input_board = train_board.flatten_board(Some(&Piece::X));
                get_next_move(&mut train_board, false);
                output_board = train_board.flatten_board(Some(&Piece::X));
                winner = check_status(&train_board);
                done = train_board.full();

                // Train on input and output boards
                if train_board.computer_piece == Piece::X { // Activate this to only train on one Piece
                    net.back_prop(&input_board, &output_board, 0.1);
                }
                // Display loss for last training round
                let out = net.forward(&input_board);
                let losss: f64 = loss(&output_board, &out);
                print!(" {:.2}", losss);
                loss_plot.data.push(losss);
                //train_board.display_board(done, &winner);
                if done || matches!(winner, Piece::O | Piece::X) { break };
                train_board.computer_piece = train_board.computer_piece.get_other_piece();
            } 
        } 
        println!("");
        let _ = plot_loss(&[loss_plot], "Loss function of tree-search training");

        net
    }

    /// Train by playing random moves. If a random move wins; use that series
    /// with the winning piece as value 1 to train a neural network.
    /// Stop when neural network can play draw against tree search. 
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn train_random(rounds: u16, piece_that_should_be_one: Piece, hidden_layers : Option<bool>) -> Self {
        let mut net = Self {
            w_in : vec![[0.0; 15]; 9],
            w_out : vec![[0.0; 9]; 15],
            w_hidden : match hidden_layers {
                Some(true) => vec![[0.0; 15]; 15].into(),
                _ => None
            },
            piece_that_should_be_one : piece_that_should_be_one,
            test : Some(false)
        };
        //let mut readkey_input = String::new();

        net.gaussian_matrix();
        
        let mut train_board: Board;
        let mut blocker_losses: DataToPlot = DataToPlot{ data : vec![], legend : "blocker loss".to_string()};
        let mut winner_losses: DataToPlot = DataToPlot{ data : vec![], legend : "winner loss".to_string()};
        let mut neural_wins: DataToPlot = DataToPlot { data: vec![], legend: "neural wins".to_string() };
        let mut random_wins: DataToPlot = DataToPlot { data: vec![], legend: "random wins".to_string() };
        for _ in 1..=rounds {
            train_board = Board {
                positions : [[Piece::None,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None]],
                score : 0,                
                computer_piece : Piece::O, // Random moves are Piece::O, which is mostly -1 below
            };
            let mut done : bool; // = false;
            let mut winner : Piece; // = Piece::None;
            let mut x_moves: Vec<[i8; 9]> = vec![]; // vec![[0; 9]];
            let mut o_moves: Vec<[i8; 9]> = vec![[0; 9]]; // vec![];
            loop {
                // O playing
                let _ = train_board.get_random_move(Some(&Piece::X));
                winner = check_status(&train_board);
                done = train_board.full();

                // Push both to the move arrays since one must
                // store the before and after boards for the training
                // In case O wins the o_moves have O made to 1 so they can be used to train the network
                x_moves.push(train_board.flatten_board(Some(&Piece::X)));
                o_moves.push(train_board.flatten_board(Some(&Piece::O)));

                //println!("Press enter to continue... to neural move");
                //let _ = std::io::stdin().read_line(&mut readkey_input);
                if done || matches!(winner, Piece::O | Piece::X) { break };

                //net.back_prop(&input_board, &output_board, 0.1);
                // forward_wrapped always uses TicTacToeNet struct piece_that_should_be_one
                // variable to play with which is set to X in the unit test function
                // neural_struct_random_train() when initializing the struct
                // X playing
                net.forward_wrapped(&mut train_board);
                winner = check_status(&train_board);
                done = train_board.full();
                //train_board.display_board(done, &winner);
                x_moves.push(train_board.flatten_board(Some(&Piece::X)));
                o_moves.push(train_board.flatten_board(Some(&Piece::O)));

                //println!("Press enter to continue...to random move");
                //let _ = std::io::stdin().read_line(&mut readkey_input);

                if done || matches!(winner, Piece::O | Piece::X) { break };
    
            }
            let winner_moves = match winner {
                Piece::O => o_moves,
                Piece::X => x_moves,
                Piece::None => vec![]
            }; 

            // Make array that accumulate wins for each round
            random_wins.data.push(random_wins.data.last().unwrap_or(&0.0) + {if winner == Piece::O {1.0} else {0.0}});
            neural_wins.data.push(neural_wins.data.last().unwrap_or(&0.0) + {if winner == Piece::X {1.0} else {0.0}});

            // Train with the winner moves, if O wins; -1 and 1 are switched
            for index in (0..winner_moves.len()).step_by(2) {
                let ones_before_move = winner_moves[index].iter().filter(|x| x.is_one()).count();
                let ones_after_move = winner_moves[index+1].iter().filter(|x| x.is_one()).count();
                // Assert that there are always one more 1 piece in the output node board
                // than the input node board
                assert!((ones_before_move + 1) == ones_after_move);
                // Train weights
                net.back_prop(&winner_moves[index], &winner_moves[index+1], 0.1);
            }

            // CHECK LOSS FUNCTION FOR A SERIES OF MOVES AND MAKE GRAPH
            //   1 2 3
            // 1| | |X|
            // 2| |O|O|
            // 3| | |X|
            // NEXT blocker move is X at 1,2
            let test_board = [0, 0, 1, 0, -1, -1, 0, 0, 1];
            let out = net.forward(&test_board);
            let blocker_losss: f64 = loss(&[0, 0, 1, 1, -1, -1, 0, 0, 1], &out);
            blocker_losses.data.push(blocker_losss);
            let test_board = [0, 0, -1, 0, 1, 1, 0, 0, -1];
            let out = net.forward(&test_board);
            let winner_losss: f64 = loss(&[0, 0, -1, 1, 1, 1, 0, 0, -1], &out);
            winner_losses.data.push(winner_losss);
        } 

        let _ = plot_loss(&[blocker_losses, winner_losses], "Random_neural training loss");
        let _ = plot_loss(&[random_wins, neural_wins], "Wins during training");
        net
    }

    /*
    Return a matrix of dimension X x Y with numbers
    in a gaussian distribution around 0 with standard deviation of 1
    Limit it to -2 to +2, i.e. generate a new number if outside
    */
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    fn gaussian_matrix(&mut self)
    {
        let mut rng = thread_rng();
        
        // Define a gaussian distribution around zero with stddev of 1
        let normal_dist = Normal::new(0.0, 1.0).unwrap();
        
        for row in 0..9 {
            for column in 0..15 {
                for matrix in 0..2 {
                // Generate a random number
                let mut random_number: f64;
                let mut iterations = 0;
                loop {
                    iterations += 1;
                    random_number = normal_dist.sample(&mut rng);
                    if (random_number < 2.0 && random_number > -2.0 && random_number != 0.0)|| iterations > 10 {
                        break;
                    }
                }
                if matrix == 0 {
                    self.w_out[column as usize][row as usize] = random_number;
                } 
                else {
                    self.w_in[row as usize][column as usize] = random_number;
                }
                }
            }
        }
        if self.w_hidden != None {
            let mut tmp = vec![[0.0; 15]; 15];
            for row in 0..15 {
                for column in 0..15 {
                // Generate a random number
                    let mut random_number: f64;
                    let mut iterations = 0;
                    loop {
                        iterations += 1;
                        random_number = normal_dist.sample(&mut rng);
                        if (random_number < 2.0 && random_number > -2.0 && random_number != 0.0)|| iterations > 10 {
                            break;
                        }
                    }
                    tmp[column as usize][row as usize] = random_number;
                }
            }
            self.w_hidden = Some(tmp);
        }
    }
    
    #[cfg_attr(not(test), allow(dead_code))]
    #[cfg_attr(test, allow(dead_code))]
    pub fn print_matrix<R>(&self, matrix: &[R])
    where 
        R : AsRef<[f64]>,
     {
        let cols = matrix[0].as_ref().len();
        print!("          ");
        for x in 0..cols {
            print!("col: {:2}  ", x+1);
        }
        println!();
        for (y, row) in matrix.iter().enumerate() {
            print!("row {:2} : ", y+1);
            for value in row.as_ref() {
                print!("{:8.5} ", value);
            }
            println!();
        }
    }

    /// Forward input data through neural network and create predicted output vector
    /// 
    /// Return: Predicted output vector
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn forward(&self, input: &[i8]) -> Vec<f64> {

        // Tranform the input vector from i8 to f64
        let input_f64: Vec<f64> = input.iter().map(|&number| number as f64).collect();

        // Scalar dot product of input vector and weigth matrix to create hidden node layer
        let columns = self.w_in[0].len();
        let mut z1: Vec<f64> = Vec::new();
        for col_index in 0..columns {
            // This is a workaround necessary because the python original code 
            // arranged the weigth matrix in one row for containing weigths for all output nodes
            // instead of all weights for one input node
            let synapse_column: Vec<f64>  = self.w_in.iter().map(|row|row[col_index]).collect();
            z1.push(scalar_dot_product(&input_f64, &synapse_column));
        }
        bipolar(&mut z1);

        let mut zh: Vec<f64> = Vec::new();
        if let Some(ref matrix) = self.w_hidden {
            assert!(matrix[5][5] != 0.0, "Hidden layer not initalized");
            let columns = matrix[0].len();  // Number of columns, i.e. output nodes
            for col_index in 0..columns {
                let synapse_column: Vec<f64>  = matrix.iter().map(|row|row[col_index]).collect();
                zh.push(scalar_dot_product(&z1, &synapse_column));
            }
            bipolar(&mut zh);
            println!("{:?}", zh);
        }


        // Scalar dot product of hidden node layer and output weigth matrix to create estimated 
        // output vector
        let columns = self.w_out[0].len();  // Number of columns, i.e. output nodes
        let mut z2: Vec<f64> = Vec::new();
        for col_index in 0..columns {
            let synapse_column: Vec<f64>  = self.w_out.iter().map(|row|row[col_index]).collect();
            if self.w_hidden.is_some() 
            {
                z2.push(scalar_dot_product(&zh, &synapse_column));
            }
            else
            {
                z2.push(scalar_dot_product(&z1, &synapse_column));
            }
        }
        bipolar(&mut z2);
        z2
    }

    /*
        Back propagate the difference between input and output 
        back to the weights
        
        Return:
                Modified weigth matrixes w1 and w2
    */
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn back_prop(&mut self, input: &[i8], output: &[i8], alpha: f64) {

        // Tranform the input vector from i8 to f64
        let input_f64: Vec<f64> = input.iter().map(|&number| number as f64).collect();

        // Scalar dot product of input vector and weigth matrix to create hidden node layer
        let columns = self.w_in[0].len();
        let mut z1: Vec<f64> = Vec::new();
        for col_index in 0..columns {
            // This is a workaround necessary because the python original code 
            // arranged the weigth matrix in one row for containing weigths for all output nodes
            // instead of all weights for one input node
            let synapse_column: Vec<f64>  = self.w_in.iter().map(|row|row[col_index]).collect();
            z1.push(scalar_dot_product(&input_f64, &synapse_column));
        }
        if self.test == Some(true) {
            // Scalar product, physical meaning: length of vector projected on another vector
            // [1, 0, -1, 0, 1, 0, 1, 0, -1] * [-0.8; 8] = -0.8 + 0.8 - 0.8 -0.8 + 0.8 = -0.8
            assert_eq!(z1[0], -0.8);
        }
        bipolar(&mut z1);
        if self.test == Some(true) {
            assert_abs_diff_eq!(z1[0], 0.31, epsilon=0.01);
            println!("Testing: first hidden node layer {:.2?}", z1);
        }
        let mut zh: Vec<f64> = Vec::new();
        if let Some(ref matrix) = self.w_hidden {
            assert!(matrix[5][5] != 0.0, "Hidden layer not initalized");
            let columns = matrix[0].len();  // Number of columns, i.e. output nodes
            for col_index in 0..columns {
                let synapse_column: Vec<f64>  = matrix.iter().map(|row|row[col_index]).collect();
                zh.push(scalar_dot_product(&z1, &synapse_column));
            }
            let temp_hidden = [0.31, 0.33, 0.35, 0.38, 0.40, 0.43, 0.45, 0.48, 0.50, 0.52, 0.55, 0.57, 0.60, 0.62, 0.65];
            let fasit: f64 = temp_hidden.iter().map(|value| value*-0.8).sum();
            if self.test == Some(true) {
                assert_abs_diff_eq!(zh[0], fasit, epsilon=0.01);
                println!("Testing: second hidden node layer {:.2?}", zh);
            }
            bipolar(&mut zh);
            if self.test == Some(true) {
                assert_abs_diff_eq!(zh[0], 1f64/(1f64+(-1f64*-5.71).exp()), epsilon=0.01);
                println!("Testing: sigmoid of second hidden node layer {:.5?}", zh);
            }
        }

        // Scalar dot product of hidden node layer and output weigth matrix to create estimated 
        // output vector
        let columns = self.w_out[0].len();  // Number of columns, i.e. output nodes
        let mut z2: Vec<f64> = Vec::new();
        for col_index in 0..columns {
            let synapse_column: Vec<f64>  = self.w_out.iter().map(|row|row[col_index]).collect();
            if self.w_hidden.is_some() 
            {
                z2.push(scalar_dot_product(&zh, &synapse_column));
            }
            else
            {
                z2.push(scalar_dot_product(&z1, &synapse_column));
            }
        }
        if self.test == Some(true) && self.w_hidden.is_none() {
            // [0.31, 0.33, 0.35, 0.38, 0.40, 0.43, 0.45, 0.48, 0.50, 0.52, 0.55, 0.57, 0.60, 0.62, 0.65] * [-0.8; 15] = -5.71
            let fasit = 0.31 * -0.8 + 0.33  * -0.8 + 0.35  * -0.8 + 0.38 * -0.8 + 0.40  * -0.8 + 0.43 * -0.8 + 0.45 * -0.8 +
                0.48 * -0.8 + 0.50 * -0.8 + 0.52 * -0.8 + 0.55 * -0.8 + 0.57  * -0.8 + 0.60 * -0.8 + 0.62  * -0.8 + 0.65 * -0.8;
            assert_abs_diff_eq!(z2[0], fasit, epsilon=0.01);
        }
        bipolar(&mut z2);
        if self.test == Some(true) && self.w_hidden == None {assert_abs_diff_eq!(z2[0], (1.0/(1.0 + (-1f64*-5.71f64).exp())), epsilon=0.01);}
        
        
        // Subtract estimated output vector with wanted output vector
        let mut d2: Vec<f64> = Vec::new();
        for (out, a2) in output.iter().zip(z2.iter()) {
            d2.push(a2 - (*out as f64));
        }
        if self.test == Some(true) && self.w_hidden.is_none() {assert_abs_diff_eq!(d2[0], (1.0/(1.0 + (-1f64*-5.71f64).exp())-1f64), epsilon=0.01);} // d2[0] = -0.99

        // Scalar dot product of output diff d2 and each of the rows in the weight matrix
        // Each row represents each of the output nodes
        // The result is a modified hidden node layer
        let mut temp_back_prop_hidden_layer: Vec<f64> = Vec::new();
        for row in self.w_out.iter() {
            temp_back_prop_hidden_layer.push(scalar_dot_product(row, &d2));
        }
        if self.test == Some(true) && self.w_hidden.is_none() {
            //self.print_matrix(&self.w_out);
            println!("{:.5?}", d2);
            //row  1 : -0.80000 -0.70000 -0.60000 -0.50000 -0.40000 -0.30000 -0.20000 -0.10000 -0.00000
            //[-0.99671, 0.00670, 1.01359, 0.02736, -0.94567, 0.10503, -0.80665, 0.32868, 1.50000]
            let fasit = -0.99671 * -0.8 + 0.00670 * -0.7 + 1.01359 * -0.6 + 0.02736 * -0.5 + 
            -0.94567 * -0.4 + 0.10503 * -0.3 + -0.80665 * -0.2 + 0.32868 * -0.1 + 1.50000 * 0.0;
            assert_abs_diff_eq!(temp_back_prop_hidden_layer[0], fasit, epsilon=0.01);
        }

        // Create a pass filtered version of the original hidden layer where small and large
        // values are dampened
        let mut wh_adj: Vec<Vec<f64>> = Vec::new();
        if self.w_hidden.is_some() 
        {
            let mut dtmp: Vec<f64> = Vec::new();
            let pass_filtered_hidden_layer: Vec<f64> = zh.iter().map(|value| value * (1.0-value)).collect();
            for (tmp_bp, filtered) in temp_back_prop_hidden_layer.iter().zip(pass_filtered_hidden_layer.iter()) {
                dtmp.push(*tmp_bp * filtered);
            }
            // Do the same to the original hidden layer and the diff output nodes d2
            for hidden_node_item in zh.iter() {
                let temp_row: Vec<f64> = dtmp.iter().map(|value| value*hidden_node_item).collect();
                wh_adj.push(temp_row);
            }
            temp_back_prop_hidden_layer.clear();
            if let Some(ref matrix) = self.w_hidden {
               for row in matrix.iter() {
                    temp_back_prop_hidden_layer.push(scalar_dot_product(row, &dtmp));
                }   
            }
        } 
        let pass_filtered_hidden_layer: Vec<f64> = z1.iter().map(|value| value * (1.0-value)).collect();
        if self.test == Some(true) && self.w_hidden.is_none() {assert_abs_diff_eq!(pass_filtered_hidden_layer[0], (0.31*(1.0 - 0.31)), epsilon=0.01);}
        let mut d1: Vec<f64> = Vec::new();
        // Create a new diff hidden layer by multiplying each node in the back_prop layer with the filtered
        for (tmp_bp, filtered) in temp_back_prop_hidden_layer.iter().zip(pass_filtered_hidden_layer.iter()) {
            d1.push(*tmp_bp * filtered);
        }
        if self.test == Some(true) && self.w_hidden.is_none() {
            assert_abs_diff_eq!(d1[0], temp_back_prop_hidden_layer[0] * (0.31*(1.0 - 0.31)), epsilon=0.01);
            println!("Testing: Diff hidden layer {:.4?}", d1);
        }

        // Take the new diff hidden layer and create a matrix by multiplying the
        // diff layer with the original input data and thus creating and new diff weight matrix
        let mut w1_adj: Vec<Vec<f64>> = Vec::new();
        for input_node_item in input_f64.iter() {
            let temp_row: Vec<f64> = d1.iter().map(|value| value*input_node_item).collect();
            w1_adj.push(temp_row);
        }
        if self.test == Some(true) && self.w_hidden.is_none() {
            assert_abs_diff_eq!(w1_adj[0][0], 0.13f64, epsilon=0.01);
            println!("Testing: First element input adjustment matrix {:?}", w1_adj[0][0]);
        }

        // Do the same to the original hidden layer and the diff output nodes d2
        let mut w2_adj: Vec<Vec<f64>> = Vec::new();
        for hidden_node_item in z1.iter() {
            let temp_row: Vec<f64> = d2.iter().map(|value| value*hidden_node_item).collect();
            w2_adj.push(temp_row);
        }

        // Adjust the original output matrix by subtracting each element with the diff matrix multiplied by alpha 
        for (row_index, w2_row_ref) in self.w_out.iter_mut().enumerate() {
            for (col_index, element) in w2_row_ref.iter_mut().enumerate() {
                *element -= alpha * w2_adj[row_index][col_index];
            }
        }

        if let Some(ref mut matrix) = self.w_hidden 
        {
            for (row_index, wh_row_ref) in matrix.iter_mut().enumerate() {
                for (col_index, element) in wh_row_ref.iter_mut().enumerate() {
                    *element -= alpha * wh_adj[row_index][col_index];
                }
            }
        }

        // Adjust the original input matrix by subtracting each element with the diff matrix multiplied by alpha 
        for (row_index, w1_row_ref) in self.w_in.iter_mut().enumerate() {
            for (col_index, element) in w1_row_ref.iter_mut().enumerate() {
                *element -= alpha * w1_adj[row_index][col_index];
            }
        }
        if self.test == Some(true) && self.w_hidden.is_none() {
            assert_abs_diff_eq!(self.w_in[0][0], -0.8-alpha*0.13f64, epsilon=0.01);
            println!("Testing: First element of new input matrix {:?}", self.w_in[0][0]);
        }
    }

    /// A wrapper around forward to remove the flattening and
    /// moving from main function
    /// Plays using struct piece_that_should_one
    pub fn forward_wrapped(&self, board: &mut Board) {

        let mut flattened_board = 
            board.flatten_board(Some(&self.piece_that_should_be_one));
        let out: Vec<f64> = self.forward(&flattened_board);
        let mut sorted_out: Vec<(f64,usize)> = out.into_iter().enumerate().map(|(i,v)| (v,i)).collect();
        sorted_out.sort_by(|a,b| b.0.partial_cmp(&a.0).unwrap());
        let sorted_out_indexes: Vec<usize> = sorted_out.into_iter().map(|(_,i)| i).collect();
        let mut move_ok = false;
        for index in sorted_out_indexes {
            if flattened_board[index] == 0 {
                flattened_board[index] = 1;
                move_ok = true;
                break;
            }
        }
        if move_ok == false { panic!("No move available, should not be possible"); }       

        board.reshape_board(flattened_board, Some(&self.piece_that_should_be_one));
    }
}
