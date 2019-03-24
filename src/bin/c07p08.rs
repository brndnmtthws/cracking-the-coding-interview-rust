#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Piece {
    Black,
    White,
    Empty,
}

#[derive(Clone, Debug)]
struct Board {
    squares: Vec<Vec<Piece>>,
}

#[derive(Clone, Debug)]
struct GameResult {
    winner: Piece,
    black_count: usize,
    white_count: usize,
}

impl Piece {
    fn is_opposite(self, other: Piece) -> bool {
        if self == Piece::Black {
            other == Piece::White
        } else if self == Piece::White {
            other == Piece::Black
        } else {
            false
        }
    }
}

impl Board {
    fn new() -> Self {
        Board {
            squares: vec![vec![Piece::Empty; 8]; 8],
        }
    }

    fn start_game(&mut self) {
        self.squares[3][3] = Piece::White;
        self.squares[4][4] = Piece::White;
        self.squares[3][4] = Piece::Black;
        self.squares[4][3] = Piece::Black;
    }

    fn place_piece(&mut self, piece: Piece, x: usize, y: usize) {
        self.squares[x][y] = piece;
        self.check_for_captured(x, y);
    }

    fn check_for_captured(&mut self, x: usize, y: usize) {
        // up
        if x >= 2
            && self.squares[x - 1][y].is_opposite(self.squares[x][y])
            && !self.squares[x - 2][y].is_opposite(self.squares[x][y])
        {
            self.squares[x - 1][y] = self.squares[x][y];
        }
        // down
        if x < self.squares.len() - 2
            && self.squares[x + 1][y].is_opposite(self.squares[x][y])
            && !self.squares[x + 2][y].is_opposite(self.squares[x][y])
        {
            self.squares[x + 1][y] = self.squares[x][y];
        }
        // left
        if y >= 2
            && self.squares[x][y - 1].is_opposite(self.squares[x][y])
            && !self.squares[x][y - 2].is_opposite(self.squares[x][y])
        {
            self.squares[x][y - 1] = self.squares[x][y];
        }
        // right
        if y < self.squares[0].len() - 2
            && self.squares[x][y + 1].is_opposite(self.squares[x][y])
            && !self.squares[y + 2][y].is_opposite(self.squares[x][y])
        {
            self.squares[x][y + 1] = self.squares[x][y];
        }
    }

    fn get_result(&self) -> GameResult {
        let black_count = self
            .squares
            .iter()
            .flat_map(|vec| vec.iter())
            .filter(|&&square| square == Piece::Black)
            .count();
        let white_count = self
            .squares
            .iter()
            .flat_map(|vec| vec.iter())
            .filter(|&&square| square == Piece::White)
            .count();
        let board_empty = self
            .squares
            .iter()
            .flat_map(|vec| vec.iter())
            .filter(|&&square| square == Piece::Empty)
            .count()
            == 0;
        let winner = if board_empty && black_count > white_count {
            Piece::Black
        } else if board_empty && white_count > black_count {
            Piece::White
        } else {
            Piece::Empty
        };
        GameResult {
            winner,
            black_count,
            white_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_othello() {
        let mut board = Board::new();
        board.start_game();
        assert_eq!(board.squares[3][3], Piece::White);
        assert_eq!(board.squares[4][4], Piece::White);
        assert_eq!(board.squares[3][4], Piece::Black);
        assert_eq!(board.squares[4][3], Piece::Black);
        board.place_piece(Piece::White, 2, 4);
        assert_eq!(board.squares[2][4], Piece::White);
        assert_eq!(board.squares[3][3], Piece::White);
        assert_eq!(board.squares[4][4], Piece::White);
        assert_eq!(board.squares[3][4], Piece::White);
        assert_eq!(board.squares[4][3], Piece::Black);
        let _result = board.get_result();
    }
}

fn main() {
    let mut board = Board::new();
    board.start_game();
    board.place_piece(Piece::White, 2, 4);
    let _result = board.get_result();
}
