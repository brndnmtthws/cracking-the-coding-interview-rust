static GRID_SIZE: usize = 8;

fn check_valid(columns: &mut Vec<usize>, row1: usize, column1: usize) -> bool {
    for (row2, column2) in columns.iter().take(row1).enumerate() {
        if column1 == *column2 {
            return false;
        }

        let column_distance = ((*column2 as i64) - (column1 as i64)).abs() as usize;

        let row_distance = row1 - row2;
        if column_distance == row_distance {
            return false;
        }
    }
    true
}

fn place_queens(row: usize, columns: &mut Vec<usize>, results: &mut Vec<Vec<usize>>) {
    if row == GRID_SIZE {
        results.push(columns.clone());
    } else {
        for col in 0..GRID_SIZE {
            if check_valid(columns, row, col) {
                columns[row] = col;
                place_queens(row + 1, columns, results);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight_queens() {
        let mut results: Vec<Vec<usize>> = vec![];
        let mut columns: Vec<usize> = vec![0; 8];
        place_queens(0, &mut columns, &mut results);
        assert_eq!(results.len(), 92);
    }
}

fn main() {
    let mut results: Vec<Vec<usize>> = vec![];
    let mut columns: Vec<usize> = vec![0; 8];
    place_queens(0, &mut columns, &mut results);
}
