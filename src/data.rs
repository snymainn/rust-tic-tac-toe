
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub struct Board {
    pub positions: [[Piece; 3];3],
    pub score: i32,
    pub current_piece : Piece,
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
}