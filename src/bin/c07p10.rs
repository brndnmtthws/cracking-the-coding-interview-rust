use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SquareState {
    Unexposed,
    Flagged,
    Exposed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameResult {
    Unfinished,
    Lost,
    Won,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Square {
    state: SquareState,
    is_mine: bool,
    value: char,
    nearby_mines: i32,
}

impl Square {
    fn set_display_value(&mut self) {
        self.value = match self.state {
            SquareState::Exposed => {
                if self.is_mine {
                    '🧨'
                } else {
                    match self.nearby_mines {
                        0 => ' ',
                        _ => std::char::from_digit(self.nearby_mines as u32, 10).unwrap(),
                    }
                }
            }
            SquareState::Flagged => '🚩',
            SquareState::Unexposed => '?',
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    squares: Vec<Vec<Square>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            squares: vec![
                vec![
                    Square {
                        state: SquareState::Unexposed,
                        value: '?',
                        nearby_mines: 0,
                        is_mine: false,
                    };
                    height
                ];
                width
            ],
        }
    }

    fn visit_adjacent_squares<F>(&mut self, x: usize, y: usize, mut f: F)
    where
        F: FnMut(&mut Square, usize, usize),
    {
        for x_offset in 0..3 {
            let xi = x as i64 + (x_offset - 1);
            if xi < 0 || xi >= self.squares.len() as i64 {
                continue;
            }
            for y_offset in 0..3 {
                let yj = y as i64 + (y_offset - 1);
                if yj < 0 || yj >= self.squares[0].len() as i64 {
                    continue;
                }
                f(
                    &mut self.squares[xi as usize][yj as usize],
                    xi as usize,
                    yj as usize,
                );
            }
        }
    }

    fn mark_nearby_squares(&mut self, x: usize, y: usize) {
        self.visit_adjacent_squares(x, y, |square, _, _| square.nearby_mines += 1);
    }

    fn place_mines(&mut self, count: usize) -> &mut Self {
        let mut mines: Vec<usize> = (0..(self.squares.len() * self.squares[0].len())).collect();
        let mut rng = thread_rng();
        mines.shuffle(&mut rng);
        for location in mines.iter().take(count) {
            let x = location / self.squares.len();
            let y = location % self.squares.len();
            self.squares[x][y].is_mine = true;
            self.mark_nearby_squares(x, y);
        }
        self
    }

    fn get_result(&self) -> GameResult {
        let mut result = GameResult::Won;
        for row in &self.squares {
            for square in row {
                if square.is_mine && square.state == SquareState::Exposed {
                    return GameResult::Lost;
                } else if !square.is_mine && square.state == SquareState::Unexposed {
                    result = GameResult::Unfinished;
                }
            }
        }
        result
    }

    fn expose_neighbours(&mut self, x: usize, y: usize) {
        let mut expose_these: Vec<(usize, usize)> = vec![];
        self.visit_adjacent_squares(x, y, |square, xi, yj| {
            if x == xi && y == yj {
                // skip this square
                return;
            } else if !square.is_mine && square.state != SquareState::Exposed {
                expose_these.push((xi, yj));
            }
        });
        expose_these.iter().for_each(|(xi, yj)| {
            self.expose_square(*xi, *yj);
        });
    }

    fn expose_square(&mut self, x: usize, y: usize) -> GameResult {
        self.squares[x][y].state = SquareState::Exposed;
        self.squares[x][y].set_display_value();
        if self.squares[x][y].is_mine {
            return GameResult::Lost;
        } else if self.squares[x][y].nearby_mines == 0 {
            self.expose_neighbours(x, y);
        }
        self.get_result()
    }

    fn flag_square(&mut self, x: usize, y: usize) {
        if self.squares[x][y].state == SquareState::Unexposed {
            self.squares[x][y].state = SquareState::Flagged;
            self.squares[x][y].set_display_value();
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&vec!["_"; 1 + 4 * self.squares.len()].join(""));
        result.push('\n');
        for row in &self.squares {
            let mut squares: Vec<String> = vec![];
            for cell in row {
                squares.push(format!(" {} |", cell.value));
            }
            result.push_str(&format!("|{}\n", squares.join("")));
            result.push_str(&vec!["-"; 1 + 4 * self.squares.len()].join(""));
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minesweeper() {
        let mut board = Board::new(10, 10);
        board.place_mines(8);
        println!("{}", board);
        board.flag_square(0, 1);
        println!("{}", board);
        let result = board.expose_square(3, 4);
        println!("{}", board);
        println!("Game result -> {:?}", result);
    }
}

fn main() {
    let mut board = Board::new(10, 10);
    board.place_mines(8);
    println!("{}", board);
    board.flag_square(0, 1);
    println!("{}", board);
    board.expose_square(3, 4);
    println!("{}", board);;
}
