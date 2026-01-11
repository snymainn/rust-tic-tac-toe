use rand::thread_rng;
use rand_distr::{Normal, Distribution};
use crate::data::*;
use crate::utils::*;
use crate::neural_utils::*;

pub struct TicTacToeNeuralNet {
    pub w_in: Vec<[f64; 15]>,
    pub w_out: Vec<[f64; 9]>,
    pub piece_that_should_be_one: Piece
}

impl TicTacToeNeuralNet {    
    pub fn train(rounds: u8, piece_that_should_be_one: Piece) -> Self {
        let mut net = Self {
            w_in : vec![[0.0; 15]; 9],
            w_out : vec![[0.0; 9]; 15],
            piece_that_should_be_one : piece_that_should_be_one
        };
        net.gaussian_matrix();
        
        let mut train_board: Board;

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
                //train_board.display_board(done, &winner);
                if done || matches!(winner, Piece::O | Piece::X) { break };
                train_board.computer_piece = train_board.computer_piece.get_other_piece();
            } 
        } 
        println!("");

        net
    }

    /// Train by playing random moves. If a random move wins; use that series
    /// with the winning piece as value 1 to train a neural network.
    /// Stop when neural network can play draw against tree search. 
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn train_random(rounds: u16, piece_that_should_be_one: Piece) -> Self {
        let mut net = Self {
            w_in : vec![[0.0; 15]; 9],
            w_out : vec![[0.0; 9]; 15],
            piece_that_should_be_one : piece_that_should_be_one
        };
        let mut readkey_input = String::new();

        net.gaussian_matrix();
        
        let mut train_board: Board;

        for round in 1..=rounds {
            print!("\nTraining round {}, =>", round);
            train_board = Board {
                positions : [[Piece::None,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None],
                            [Piece::None,Piece::None,Piece::None]],
                score : 0,                
                computer_piece : Piece::O,
            };
            let mut done : bool = false;
            let mut winner : Piece = Piece::None;
            print!(" loss : ");
            let mut x_moves: Vec<[i8; 9]> = vec![]; // vec![[0; 9]];
            let mut o_moves: Vec<[i8; 9]> = vec![[0; 9]]; // vec![];
            loop {
                let _ = train_board.get_random_move(Some(&Piece::X));
                winner = check_status(&train_board);
                done = train_board.full();
                train_board.display_board(done, &winner);
                //if train_board.computer_piece == Piece::X {
                // Push both to the move arrays since one must
                // store the before and after boards for the training
                    x_moves.push(train_board.flatten_board(Some(&Piece::X)));
                //} else {
                    // O must be set to one here for the training to work on these boards, if used for training
                    o_moves.push(train_board.flatten_board(Some(&Piece::O)));
                //}

                println!("Press enter to continue... to neural move");
                let _ = std::io::stdin().read_line(&mut readkey_input);
                if done || matches!(winner, Piece::O | Piece::X) { break };

                //net.back_prop(&input_board, &output_board, 0.1);
                // Forward wrapper always uses TicTacToeNet struct piece_that_should_be_one
                // variable to play with 
                net.forward_wrapped(&mut train_board);
                winner = check_status(&train_board);
                done = train_board.full();
                train_board.display_board(done, &winner);
                    x_moves.push(train_board.flatten_board(Some(&Piece::X)));
                //} else {
                    // O must be set to one here for the training to work on these boards, if used for training
                    o_moves.push(train_board.flatten_board(Some(&Piece::O)));

                println!("Press enter to continue...to random move");
                let _ = std::io::stdin().read_line(&mut readkey_input);

                //let losss: f64 = loss(&output_board, &out);
                //print!(" {:.2}", losss);
                
                if done || matches!(winner, Piece::O | Piece::X) { break };
                //train_board.computer_piece = train_board.computer_piece.get_other_piece();
            }
            //println!("{} : {:?}", x_moves.len(), x_moves);
            //println!("{} : {:?}", o_moves.len(), o_moves);
            let winner_moves = match winner {
                Piece::O => o_moves,
                Piece::X => x_moves,
                Piece::None => vec![]
            }; 
            println!("{} : {:?}", winner_moves.len(), winner_moves);
            for index in (0..winner_moves.len()).step_by(2) {
                print!("Using index : {} and {}", index, index +1);
                net.back_prop(&winner_moves[index], &winner_moves[index+1], 0.1);
                let out = net.forward(&winner_moves[index]);
                let losss: f64 = loss(&winner_moves[index+1], &out);
                println!(", loss: {:.2}", losss);
            }
        } 
        println!("");

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

    /*
    Forward input data through neural network and create predicted output vector
    
    Return:
            Predicted output vector
    */
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    fn forward(&self, input: &[i8]) -> Vec<f64> {

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
        sigmoid(&mut z1);

        // Scalar dot product of hidden node layer and output weigth matrix to create estimated 
        // output vector
        let columns = self.w_out[0].len();  // Number of columns, i.e. output nodes
        let mut z2: Vec<f64> = Vec::new();
        for col_index in 0..columns {
            let synapse_column: Vec<f64>  = self.w_out.iter().map(|row|row[col_index]).collect();
            z2.push(scalar_dot_product(&z1, &synapse_column));
        }
        sigmoid(&mut z2);
        z2
    }

    /*
        Back propagate the difference between input and output 
        back to the weights
        
        Return:
                Modified weigth matrixes w1 and w2
    */
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    fn back_prop(&mut self, input: &[i8], output: &[i8], alpha: f64) {

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
        sigmoid(&mut z1);

        // Scalar dot product of hidden node layer and output weigth matrix to create estimated 
        // output vector
        let columns = self.w_out[0].len();  // Number of columns, i.e. output nodes
        let mut z2: Vec<f64> = Vec::new();
        for col_index in 0..columns {
            let synapse_column: Vec<f64>  = self.w_out.iter().map(|row|row[col_index]).collect();
            z2.push(scalar_dot_product(&z1, &synapse_column));
        }
        sigmoid(&mut z2);
        
        // Subtract estimated output vector with wanted output vector
        let mut d2: Vec<f64> = Vec::new();
        for (out, a2) in output.iter().zip(z2.iter()) {
            d2.push(a2 - (*out as f64));
        }
        
        // Scalar dot product of output diff d2 and each of the rows in the weight matrix
        // Each row represents each of the output nodes
        // The result is a modified hidden node layer
        let mut temp_back_prop_hidden_layer: Vec<f64> = Vec::new();
        for row in self.w_out.iter() {
            temp_back_prop_hidden_layer.push(scalar_dot_product(row, &d2));
        }

        // Create a pass filtered version of the original hidden layer where small and large
        // values are dampened
        let pass_filtered_hidden_layer: Vec<f64> = z1.iter().map(|value| value * (1.0-value)).collect();
        
        // Create a new diff hidden layer by multiplying each node in the back_prop layer with the filtered
        let mut d1: Vec<f64> = Vec::new();
        for (tmp_bp, filtered) in temp_back_prop_hidden_layer.iter().zip(pass_filtered_hidden_layer.iter()) {
            d1.push(*tmp_bp * filtered);
        }

        // Take the new diff hidden layer and create a matrix by multiplying the
        // diff layer with the original input data and thus creating and new diff weight matrix
        let mut w1_adj: Vec<Vec<f64>> = Vec::new();
        for input_node_item in input_f64.iter() {
            let temp_row: Vec<f64> = d1.iter().map(|value| value*input_node_item).collect();
            w1_adj.push(temp_row);
        }

        // Do the same to the original hidden layer and the diff output nodes d2
        let mut w2_adj: Vec<Vec<f64>> = Vec::new();
        for hidden_node_item in z1.iter() {
            let temp_row: Vec<f64> = d2.iter().map(|value| value*hidden_node_item).collect();
            w2_adj.push(temp_row);
        }

        for (row_index, w2_row_ref) in self.w_out.iter_mut().enumerate() {
            for (col_index, element) in w2_row_ref.iter_mut().enumerate() {
                *element -= alpha * w2_adj[row_index][col_index];
            }
        }

        for (row_index, w1_row_ref) in self.w_in.iter_mut().enumerate() {
            for (col_index, element) in w1_row_ref.iter_mut().enumerate() {
                *element -= alpha * w1_adj[row_index][col_index];
            }
        }
    }

    /// A wrapper around forward to remove the flattening and
    /// moving from main function
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
