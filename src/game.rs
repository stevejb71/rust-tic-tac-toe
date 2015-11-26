#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Square {
    Empty, Cross, Nought
}

#[derive(Debug)]
pub struct Board {
    pub squares: [Square; 9]
}

impl Board {
    pub fn new() -> Board {
        Board {squares: [Square::Empty; 9]}
    }

    pub fn is_empty_at(&self, index: usize) -> bool {
        self.squares[index] == Square::Empty
    }

    pub fn is_full(&self) -> bool {
        self.squares.iter().all(|&x| x != Square::Empty)
    }
}

pub fn play(board: &Board) -> Option<usize> {
    let squares = [4, 0, 2, 6, 8, 1, 3, 5, 7];
    squares.iter().find(|&i| board.is_empty_at(*i)).map(|i| *i)
}
