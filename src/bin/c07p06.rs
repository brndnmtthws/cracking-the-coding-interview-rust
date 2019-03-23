use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
enum Side {
    Top,
    Left,
    Right,
    Bottom,
    None,
}

#[derive(Clone, Eq, PartialEq)]
struct Piece {
    top: Option<PieceRef>,
    left: Option<PieceRef>,
    right: Option<PieceRef>,
    bottom: Option<PieceRef>,
    w: usize,
    h: usize,
}

type PieceRef = Rc<RefCell<Piece>>;

#[derive(Clone)]
struct Puzzle {
    pieces: Vec<PieceRef>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn new(width: usize, height: usize) -> Self {
        let mut pieces: Vec<Vec<PieceRef>> = vec![];
        for w in 0..width {
            let mut row: Vec<PieceRef> = vec![];
            for h in 0..height {
                row.push(Rc::new(RefCell::new(Piece {
                    top: None,
                    left: None,
                    right: None,
                    bottom: None,
                    w,
                    h,
                })));
            }
            pieces.push(row);
        }
        let mut final_pieces: Vec<PieceRef> = vec![];
        for w in 0..width {
            for h in 0..height {
                if h > 0 {
                    pieces[w][h].borrow_mut().top = Some(pieces[w][h - 1].clone());
                }
                if w > 0 {
                    pieces[w][h].borrow_mut().left = Some(pieces[w - 1][h].clone());
                }
                if h < height - 1 {
                    pieces[w][h].borrow_mut().bottom = Some(pieces[w][h + 1].clone());
                }
                if w < width - 1 {
                    pieces[w][h].borrow_mut().right = Some(pieces[w + 1][h].clone());
                }
                final_pieces.push(pieces[w][h].clone());
            }
        }
        let mut rng = thread_rng();
        final_pieces.shuffle(&mut rng);
        Puzzle {
            pieces: final_pieces,
            width,
            height,
        }
    }

    fn get_fitting_piece(&mut self, cur_piece: &PieceRef) -> Option<PieceRef> {
        // find a fitting piece
        let mut idx: usize = 0;
        let mut ret: Option<PieceRef> = None;
        for (i, piece) in self.pieces.iter().enumerate() {
            if cur_piece.borrow().fits_with(piece) != Side::None {
                idx = i;
                ret = Some(piece.clone());
                break;
            }
        }
        if ret.is_some() {
            self.pieces.remove(idx);
        }
        ret
    }

    fn place_piece(
        &mut self,
        pieces: &mut Vec<Vec<PieceRef>>,
        cur_piece: &PieceRef,
        new_piece: &PieceRef,
    ) {
        match cur_piece.borrow().fits_with(new_piece) {
            Side::Bottom => {
                if pieces.len() < new_piece.borrow().w + 1 {
                    pieces.push(vec![]);
                }
                pieces[new_piece.borrow().w].push(new_piece.clone());
            }
            Side::Right => {
                if pieces.len() < new_piece.borrow().w + 1 {
                    pieces.push(vec![]);
                }
                pieces[new_piece.borrow().w].push(new_piece.clone());
            }
            _ => (),
        }
    }

    fn find_bottom_and_right_piece(
        &mut self,
        pieces: &mut Vec<Vec<PieceRef>>,
        cur_piece: &PieceRef,
    ) {
        if let Some(first_piece) = self.get_fitting_piece(cur_piece) {
            self.place_piece(pieces, cur_piece, &first_piece);
        }
        if let Some(second_piece) = self.get_fitting_piece(cur_piece) {
            self.place_piece(pieces, cur_piece, &second_piece);
        }
    }

    fn solve(&mut self) -> Vec<Vec<PieceRef>> {
        let mut pieces: Vec<Vec<PieceRef>> = vec![];

        // find top left corner piece
        let mut top_left_idx: usize = 0;
        for (i, piece) in self.pieces.iter().enumerate() {
            if piece.borrow().top.is_none() && piece.borrow().left.is_none() {
                pieces.push(vec![piece.clone()]);
                top_left_idx = i;
                break;
            }
        }
        self.pieces.remove(top_left_idx);
        let mut w_idx: usize = 0;
        let mut h_idx: usize = 0;
        while w_idx < self.width {
            while h_idx < self.height {
                let piece = pieces[w_idx][h_idx].clone();
                self.find_bottom_and_right_piece(&mut pieces, &piece);
                h_idx += 1;
            }
            h_idx = 0;
            w_idx += 1;
        }
        pieces
    }
}

impl Piece {
    fn fits_with_inner(&self, first: &Option<PieceRef>, second: &PieceRef) -> bool {
        if let Some(first_ptr) = first {
            return Rc::ptr_eq(first_ptr, second);
        }
        false
    }

    fn fits_with(&self, other: &PieceRef) -> Side {
        if self.fits_with_inner(&self.top, other) {
            Side::Top
        } else if self.fits_with_inner(&self.bottom, other) {
            Side::Bottom
        } else if self.fits_with_inner(&self.left, other) {
            Side::Left
        } else if self.fits_with_inner(&self.right, other) {
            Side::Right
        } else {
            Side::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jigsaw_puzzle() {
        let mut puzzle = Puzzle::new(10, 5);
        let solution = puzzle.solve();
        let mut c = 0;
        for row in solution {
            for piece in row {
                assert_eq!(c, (piece.borrow().w * 5) + piece.borrow().h);
                c += 1;
            }
        }
    }
}

fn main() {
    let mut puzzle = Puzzle::new(10, 5);
    puzzle.solve();
}
