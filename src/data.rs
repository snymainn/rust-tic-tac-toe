use rand::Rng;

pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct DataToPlot {
    pub data: Vec<f64>,
    pub legend: String,
}

// Debug trait is a simple method to get print
// to output a string form of the enum definition
// A better way is to implement Display trait
#[derive(Debug,Clone,PartialEq)]
pub enum Piece { None, X, O}

#[derive(Debug,Clone,PartialEq)]
pub enum ComputerPlayerType { TreeSearch, Neural }

impl Piece {
    pub fn get_piece(&self) -> String {
        match self {
            Piece::None => " ".to_string(),
            Piece::X => "X".to_string(),
            Piece::O => "O".to_string(),
        }
    }
    pub fn get_other_piece(&self) -> Piece {
        match self {
            Piece::O => Piece::X,
            Piece::X => Piece::O,
            Piece::None => Piece::None,
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Board {
    pub positions: [[Piece; 3];3],
    pub score: i32,
    pub computer_piece : Piece,
}

impl Board {
    pub fn display_board(&self, done: bool, winner: &Piece)
    {
        println!("  1 2 3");

        for (y, row) in self.positions.iter().enumerate() {
            print!("{}", y+1);
            for (_, col) in row.iter().enumerate() {
                print!("|{}", col.get_piece());
                //col.display_piece();
            }
            if (y + 1) == 2 && (matches!(winner, Piece::O) || matches!(winner, Piece::X)) {
                println!("|    Winner: {}", winner.get_piece());
            } else if (y + 1) == 2 && done == true && matches!(winner, Piece::None) {
                println!("|    Draw, no winners");
            } else {
                println!("|");
            }
        }
        println!("Score {}\n", self.score);
    }
    pub fn full(&self) -> bool {
        self.positions.iter().flatten().find(|&x| *x == Piece::None).is_none()
    }
    /// Flatten 3x3 matrix to 1x9 vector.
    /// 1 is default computer_piece, but can be given in arguments
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn flatten_board(&self, piece_that_should_be_one_flattened: Option<&Piece>) -> [i8; 9] {
        let one_piece = piece_that_should_be_one_flattened.clone().unwrap_or(&self.computer_piece);
        let mut flattened_board: [i8; 9] = [0; 9]; 
        for (y, row) in self.positions.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == *one_piece {
                    flattened_board[y*3+x] = 1;
                } 
                else if *col == one_piece.get_other_piece()
                {
                    flattened_board[y*3+x] = -1;
                }
                else {
                    flattened_board[y*3+x] = 0;
                } 
            }
        }
        return flattened_board;
    }

    /// Reshape 1x9 to 3x3 matrix
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn reshape_board(&mut self, input_vector: [i8; 9], what_piece_one_should_be: Option<&Piece>) {
        let one_piece = what_piece_one_should_be.clone().unwrap_or(&self.computer_piece);
        for (y, row) in self.positions.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                // Change value of flatting depending on who is playing
                // This is done to avoid self learning (back propagation) 
                // to be reversed every second round when the player changes
                if input_vector[y*3+x] == 1 {
                        *col = one_piece.clone();
                }
                else if input_vector[y*3+x] == -1 {
                    *col = one_piece.get_other_piece();
                }
                else {
                    *col = Piece::None;
                } 
            }
        }
    }

    /// Get random move
    /// Return Some(index) found or None
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn get_random_move(&mut self, what_piece_one_should_be: Option<&Piece>) -> Option<usize> {
        let one_piece = what_piece_one_should_be.clone().unwrap_or(&self.computer_piece);
        let mut board_array = self.flatten_board(what_piece_one_should_be);
        let open_indexses: Vec<usize> = board_array.iter().enumerate().filter_map(|(i, &v)| if v == 0 { Some(i) } else { None }).collect();
        //println!("{}", open_indexses.len());
        let number_of_open_positions = open_indexses.len();
        if number_of_open_positions > 0 {
            let mut rng = rand::thread_rng(); 
            let index = rng.gen_range(0..number_of_open_positions);
            if self.computer_piece == *one_piece {
                board_array[open_indexses[index]] = 1; 
            } else {
                board_array[open_indexses[index]] = -1;
            }
            self.reshape_board(board_array, what_piece_one_should_be);
            return Some(open_indexses[index]);
        } else {
            return None;
        }
    }
}