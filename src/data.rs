
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Piece { None, X, O}

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
    /// Flatten 3x3 matrix to 1x9 vector
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn flatten_board(&self) -> [i8; 9] {
        let mut flattened_board: [i8; 9] = [0; 9]; 
        for (y, row) in self.positions.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                // Change value of flatting depending on who is playing
                // This is done to avoid self learning (back propagation) 
                // to be reversed every second round when the player changes
                if *col == self.computer_piece {
                    flattened_board[y*3+x] = 1;
                } 
                else if *col != self.computer_piece && *col != Piece::None
                {
                    flattened_board[y*3+x] = -1;
                } 
            }
        }
        return flattened_board;
    }

    /// Reshape 1x9 to 3x3 matrix
    #[cfg_attr(not(test), allow(dead_code))] // Allow dead code for prod build because only in test currently
    pub fn reshape_board(&mut self, input_vector: [i8; 9]) {
        for (y, row) in self.positions.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                // Change value of flatting depending on who is playing
                // This is done to avoid self learning (back propagation) 
                // to be reversed every second round when the player changes
                if input_vector[y*3+x] == 1 {
                    if self.computer_piece == Piece::X {
                        *col = Piece::X;
                     } 
                     else { 
                        *col = Piece::O; 
                    }
                }
                else if input_vector[y*3+x] == -1 {
                    *col = if self.computer_piece == Piece::X { Piece::O } else { Piece::X };
                }
                else {
                    *col = Piece::None;
                } 
            }
        }
    }
    
}