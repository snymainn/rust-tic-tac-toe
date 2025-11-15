use std::io::{self, BufRead};
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

pub fn dive(board: &Board, piece: &Piece, y: usize, x: usize, in_level: i32, debug: bool) -> (i32, i32)
{
    let level_score = ((9.0/(in_level as f32))*100.0) as i32;
    let mut new_board = (*board).clone();
    new_board.positions[y][x] = piece.clone();
    let winner = check_status(&new_board);
    if board.computer_piece.get_piece() == winner.get_piece() {
        return (level_score, 0);
    } else if winner.get_piece() == piece.get_piece() {
        return (-level_score, 1);
    }
    if check_blocker(&new_board, y, x) {
        return (level_score-5, 0); // Let blocker have slightly less score to avoid it overriding instant win
    }
    let mut temp_score: i32;
    let mut temp_losses: i32;
    let mut losses = 0;
    let mut first: bool = false;
    let next_piece = piece.get_other_piece();

    for (y, row) in new_board.positions.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if matches!(new_board.positions[y][x], Piece::None) {
                (temp_score, temp_losses) = dive(&new_board, &next_piece, y, x, in_level+1, debug);
                if first == false {
                    new_board.score = temp_score;
                    first = true;
                } else if temp_score > new_board.score {
                    new_board.score = temp_score;
                }
                losses += temp_losses;
                if debug && in_level < 3 { println!("{0:>tab$} : {1}, score: {2}, new_board.score : {3}, losses : {4}", "Level ", 
                    in_level, temp_score, new_board.score, losses, tab=((in_level*10) as usize)); };
            }
        }
    }
    return (new_board.score, losses);
}

/*
    Find which of the level open positions have the highest score

    Return done: 
            false : not done, more space available
            true : done, not more space on board
 */
pub fn get_next_move(board: &mut Board, debug: bool) {
    let mut made_new_moves = false;

    let mut top_score_y = 0;
    let mut top_score_x = 0;
    
    for (y, row) in board.positions.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if matches!(board.positions[y][x], Piece::None) {
                if debug { println!("Analysing position row={}, x={}", y, x); };
                let (mut topscore, losses) = dive(&board, &board.computer_piece, y, x, 1, debug);
                if x==1 && y==1  {
                    topscore = topscore * 2; // Center square is important
                }
                topscore -= losses;
                if made_new_moves == false || topscore > board.score {
                    if debug { println!("    *** New top score: {}, row:{}, x={}", topscore, y, x); }
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
    for x in 0..=2 {
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
///
/// Print two dimensional matrix
/// 
/// Inner slices must be mutable since they are often weigth matrixes 
/// that must be updated in other functions, but the outer slice is not mutable
/// and thus it is not possible to iter_mut over it, not thus not possible 
/// to iter_mut over the inner slice either. 
/// But it could be that the construct here locks other from borrowing the matrix
/// sent in. 
/// 
//#[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
#[cfg_attr(test,allow(dead_code))]
#[cfg_attr(not(test),allow(dead_code))]
pub fn print_matrix(matrix: &[&mut[f64]]) {
        print!("          ");
        for (x, _) in matrix[0].iter().enumerate() {
            print!("col: {:2}  ", x+1);
        }
        println!();
        for (y, row) in matrix.iter().enumerate() {
            print!("row {:2} : ", y+1);
            for (_, col) in row.iter().enumerate() {
                print!("{:8.5} ", col);
            }
            println!();
        }
}

#[cfg_attr(not(test),allow(dead_code))]
pub fn select_option<T,R>(options: &Vec<(&str, Option<T>)>, mut reader: R) -> Option<T> 
where 
T: Clone,
R: BufRead,
{

    let vars: Vec<&str> = options.iter().map(|(s, _)| *s).collect();
    let vars = vars.join(", ");
    
    let result = loop {
        println!("Select from {} : ", vars);

        let mut response = String::new();
    
        match reader.read_line(&mut response) {
            Ok(0) => {
                break None;
            }
            Ok(_) => {
                let trimmed_response = response.trim().to_lowercase();                
                for tuple in options {
                    println!("{}", tuple.0);
                    if trimmed_response.as_str() == tuple.0.to_lowercase() {
                        return tuple.1.clone();        
                    }
                }
            }
            Err(e) => {
                eprint!("Error reading keyboard input: {}", e);
                break None;
            }
        }           
        println!("Invalid selection, try again!");
    };
    result
}