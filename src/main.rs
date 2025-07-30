// The mod statements declares the files used in this project
mod data;
mod utils;
mod neural_utils;
mod neural_data;

mod tests;

use std::io;
use rand::Rng;

// Imports the elements so shorter names can be used
use crate::data::*;
use crate::utils::*;

fn main() {

    let my_piece: Piece;

    loop {
        println!("Select {} or {} : ", Piece::O.get_piece(), Piece::X.get_piece());

        let mut response = String::new();
    
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read selection"); 


        match response.trim().to_lowercase().as_str() {
            "x" => {
                my_piece = Piece::X;
                break;
            },
            "o" => {
                my_piece = Piece::O;
                break;
            },
            _ => {
                println!("Invalid selection, try again!");
            }
        } 
    }
    
    println!("Selected player {}", my_piece.get_piece());

    let mut start_board = Board {
        positions : [[Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None]],
        score : 0,
        current_piece : my_piece.get_other_piece(),
    };
    let mut done = false;
    let mut winner: Piece = Piece::None;

    let start: u8 = rand::thread_rng().gen_range(1..=2);
    match start {
        1 => println!("Computer starts!"),
        _ => {
            println!("You start!");
            start_board.display_board(done, &winner);
            let position: Position = get_input(&start_board);
            println!("Got position : {},{}", position.row, position.col);
            start_board.positions[position.row-1][position.col-1] = my_piece.clone(); // Needs clone due to iteration
        },
    }
    
    loop {
        done = get_next_move(&mut start_board);
        winner = check_status(&start_board);
        start_board.display_board(done, &winner);
        if done || matches!(winner, Piece::O | Piece::X) { break };
        let position: Position = get_input(&start_board);
        println!("Got position : {},{}", position.row, position.col);
        start_board.positions[position.row-1][position.col-1] = my_piece.clone(); // Needs clone due to iteration
        start_board.display_board(done, &winner);
    }   
}


