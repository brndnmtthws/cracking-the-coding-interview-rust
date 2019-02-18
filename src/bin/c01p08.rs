type Mat = Vec<Vec<u32>>;

fn zero_out(mat: &mut Mat, x: usize, y: usize) {
    // Zero out row
    for row in mat.iter_mut() {
        row[y] = 0;
    }

    // Zero out column
    for i in 0..mat[x].len() {
        mat[x][i] = 0;
    }
}

fn zero_matrix_where_zero(mat: &[Vec<u32>]) -> Mat {
    let mut result = mat.to_owned();

    for (x, row) in mat.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == 0 {
                zero_out(&mut result, x, y);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix_where_zeros() {
        assert_eq!(
            zero_matrix_where_zero(&[vec![0, 1], vec![1, 1]]),
            [vec![0, 0], vec![0, 1]]
        );
        assert_eq!(
            zero_matrix_where_zero(&[
                vec![0, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
            ]),
            [
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 1],
                vec![0, 1, 1, 1],
                vec![0, 1, 1, 1],
            ]
        );
        assert_eq!(
            zero_matrix_where_zero(&[
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 0, 1],
                vec![1, 1, 1, 1],
            ]),
            [
                vec![1, 1, 0, 1],
                vec![1, 1, 0, 1],
                vec![0, 0, 0, 0],
                vec![1, 1, 0, 1],
            ]
        );
    }
}

fn main() {
    zero_matrix_where_zero(&[vec![0]]);
}
