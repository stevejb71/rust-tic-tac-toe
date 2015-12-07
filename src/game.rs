#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Square {
    Empty, Cross, Nought
}

impl Square {
    pub fn other(&self) -> Square {
        match *self {
            Square::Empty => Square::Empty,
            Square::Cross => Square::Nought,
            Square::Nought => Square::Cross
        }
    }
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

    pub fn first_blocker(&self, square: Square) -> Option<usize> {
        let line_indices = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 4, 8], [2, 4, 6],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
        ];
        line_indices.iter()
                    .flat_map(|is| self.blank_if_2_on_a_line(is, square))
                    .nth(0)
    }

    fn blank_if_2_on_a_line(&self, line_indices: &[usize; 3], square: Square) -> Option<usize> {
        let squares_count = line_indices.iter().filter(|&i| self.squares[*i] == square).count();
        if squares_count == 2 {
            line_indices.iter().find(|&i| self.squares[*i] == Square::Empty).map(|&x| x)
        } else {
            None
        }
    }
}

pub fn play(board: &Board, player: Square) -> Option<usize> {
    let win = board.first_blocker(player);
    if win.is_some() {
        win
    } else {
        let blocker = board.first_blocker(player.other());
        if blocker.is_some() {
            blocker
        } else {
            let squares = [4, 0, 2, 6, 8, 1, 3, 5, 7];
            squares.iter().find(|&i| board.is_empty_at(*i)).map(|i| *i)
        }
    }
}
