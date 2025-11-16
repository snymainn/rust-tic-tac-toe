// The mod statements declares the files used in this project
mod data;
mod neural_data;
mod neural_struct;
mod neural_utils;
mod utils;

mod tests;

use rand::Rng;
use std::io;

// Imports the elements so shorter names can be used
use crate::data::*;
use crate::neural_struct::TicTacToeNeuralNet;
use crate::utils::*;

fn main() {
    let computer_player: ComputerPlayerType;
    println!("Select if you want computer to use Tree Search (T) or Neural network (N).");
    let response = select_option(
        &vec![
            ("N", Some(ComputerPlayerType::Neural)),
            ("T", Some(ComputerPlayerType::TreeSearch)),
        ],
        io::stdin().lock(),
    );
    computer_player = response.expect("Expected response of type ComputerPlayerType");

    println!("Selected computer player type :  {:?}\n", computer_player);

    let my_piece: Piece;
    print!("Select which piece you want. ");

    let response = select_option(
        &vec![("X", Some(Piece::X)), ("O", Some(Piece::O))],
        io::stdin().lock(),
    );
    my_piece = response.expect("Expected response of type Piece");

    println!("Selected player {:?}", my_piece);

    let mut neural_player: Option<TicTacToeNeuralNet> = None;
    if computer_player == ComputerPlayerType::Neural {
        println!("\nTraining neural network");
        neural_player = Some(TicTacToeNeuralNet::train(5));
    }

    let mut board = Board {
        positions: [
            [Piece::None, Piece::None, Piece::None],
            [Piece::None, Piece::None, Piece::None],
            [Piece::None, Piece::None, Piece::None],
        ],
        score: 0,
        computer_piece: my_piece.get_other_piece(),
    };
    let mut done = false;
    let mut winner: Piece = Piece::None;
    let mut human_player: bool = false;

    let start: u8 = rand::thread_rng().gen_range(1..=2);
    match start {
        1 => println!("Computer starts!"),
        _ => {
            println!("You start!");
            board.display_board(done, &winner);
            human_player = true;
        }
    }

    loop {
        if human_player {
            let position: Position = get_input(&board);
            println!("Got position : {},{}", position.row, position.col);
            board.positions[position.row - 1][position.col - 1] = my_piece.clone(); // Needs clone due to iteration
            human_player = false;
        } else {
            match computer_player {
                ComputerPlayerType::Neural => {
                    if let Some(player) = &neural_player {
                        player.forward_wrapped(&mut board);
                    } else {
                        panic!("Neural network not trained");
                    }
                }
                ComputerPlayerType::TreeSearch => get_next_move(&mut board, false),
            }
            human_player = true;
        }
        winner = check_status(&board);
        done = board.full();
        board.display_board(done, &winner);
        if done || matches!(winner, Piece::O | Piece::X) {
            break;
        };
    }
}
