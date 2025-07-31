use std::io;
use crate::data::*;

pub fn get_input(board: &Board) -> Position {
    let mut response = String::new();
    let mut col:usize;
    let mut row:usize;

    loop {
        io::stdin()
        .read_line(&mut response)
        .expect("Failed to read selection");
        match response.trim().split_once(',') {
            Some(v) => {
                row = match v.0.parse() {
                    Ok(n) => n,
                    Err(_) => 0,
                };
                col = match v.1.parse() {
                    Ok(n) => n,
                    Err(_) => 0,
                };
                if row>0 && row<=3 && col>0 && col<=3 && 
                    matches!(board.positions[row-1][col-1],Piece::None) {
                    break;
                }
                else {
                    println!("Incorrect row, col or location is not available, try again!");
                }
            },  
            None => {
                println!("Failed to interprete row and column, try again!");
            },
        };
        response.clear();
    }
    Position { row: row, col: col }
}

pub fn dive(board: &Board, piece: &Piece, y: usize, x: usize, in_level: i32) -> i32
{
    let level_score = ((9.0/(in_level as f32))*100.0) as i32;
    let mut new_board = (*board).clone();
    new_board.positions[y][x] = piece.clone();
    let winner = check_status(&new_board);
    if board.computer_piece.get_piece() == winner.get_piece() {
        return level_score;
    } else if winner.get_piece() == piece.get_piece() {
        return -level_score;
    }
    if check_blocker(&new_board, y, x) {
        return level_score;
    }
    let mut temp_score: i32;
    let mut first: bool = false;
    let next_piece = piece.get_other_piece();

    for (y, row) in new_board.positions.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if matches!(new_board.positions[y][x], Piece::None) {
                temp_score = dive(&new_board, &next_piece, y, x, in_level+1);
                if first == false {
                    new_board.score = temp_score;
                    first = true;
                } else if temp_score > new_board.score {
                    new_board.score = temp_score;
                }
            }
        }
    }
    return new_board.score;
}

/*
    Find which of the level open positions have the highest score

    Return done: 
            false : not done, more space available
            true : done, not more space on board
 */
pub fn get_next_move(board: &mut Board) {
    let mut made_new_moves = false;

    let mut top_score_y = 0;
    let mut top_score_x = 0;
    
    for (y, row) in board.positions.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if matches!(board.positions[y][x], Piece::None) {
                let mut topscore = dive(&board, &board.computer_piece, y, x, 1);
                if x==1 && y==1  {
                    topscore = topscore * 2; // Center square is important
                }
                if made_new_moves == false || topscore > board.score {
                    board.score = topscore;
                    made_new_moves = true;
                    top_score_y = y;
                    top_score_x = x;
                }
            }
        }
    }
    if made_new_moves {
        board.positions[top_score_y][top_score_x] = board.computer_piece.clone();
    }
}

/*
    Check if board has three in a row anywhere
    Return:
            Piece::X or Piece::O if any of these has three in a row
            Piece::None if there are no winners at the moment
*/
pub fn check_status(board: &Board) -> Piece {

    /* Check rows */
    for (_, row) in board.positions.iter().enumerate() {
        if matches!(row[0], Piece::X | Piece::O) && 
            row[0].get_piece() == row[1].get_piece() &&
            row[1].get_piece() == row[2].get_piece()
        {
            return row[0].clone();
        }
    }
    for x in 0..2 {
        if matches!(board.positions[0][x], Piece::O | Piece::X) &&
            board.positions[0][x].get_piece() == board.positions[1][x].get_piece() &&
            board.positions[1][x].get_piece() == board.positions[2][x].get_piece() {
                return board.positions[0][x].clone();
            }
    }
    for c in [(0,2,2,0),(0,0,2,2)] {
        if matches!(board.positions[1][1], Piece::X | Piece::O) && 
            board.positions[c.0][c.1].get_piece() == board.positions[1][1].get_piece() &&
            board.positions[1][1].get_piece() == board.positions[c.2][c.3].get_piece()
        {
            return board.positions[1][1].clone();
        }
    }
    
    return Piece::None;
}

/*
    Inverses the last placed piece and checks if this generates a win
    If so, it is actually a blocker and has a high score
    It is not necessary to propagate further when a blocker is found
*/
pub fn check_blocker(board: &Board, y: usize, x: usize) -> bool {
    let mut new_board = (*board).clone();
    let temp_piece = new_board.positions[y][x].clone();
    new_board.positions[y][x] = temp_piece.get_other_piece();
    
    let winner = check_status(&new_board);
    if matches!(winner, Piece::O | Piece::X) 
    {
        return true;
    }
    return false; 
}

