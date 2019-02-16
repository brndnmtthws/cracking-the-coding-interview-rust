type Mat = Vec<Vec<u32>>;

fn zero_out(mat: &mut Mat, x: usize, y: usize) {
    // Zero out column
    for i in 0..mat.len() {
        mat[i][y] = 0;
    }

    // Zero out row
    for i in 0..mat[x].len() {
        mat[x][i] = 0;
    }
}

fn zero_matrix_where_zero(mat: &Mat) -> Mat {
    let mut result = mat.clone();

    let x_len = mat.len();
    for x in 0..mat.len() {
        let y_len = mat[x].len();
        for y in 0..y_len {
            if mat[x][y] == 0 {
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
            zero_matrix_where_zero(&vec![vec![0, 1], vec![1, 1]]),
            vec![vec![0, 0], vec![0, 1]]
        );
        assert_eq!(
            zero_matrix_where_zero(&vec![
                vec![0, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
            ]),
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 1],
                vec![0, 1, 1, 1],
                vec![0, 1, 1, 1],
            ]
        );
        assert_eq!(
            zero_matrix_where_zero(&vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 0, 1],
                vec![1, 1, 1, 1],
            ]),
            vec![
                vec![1, 1, 0, 1],
                vec![1, 1, 0, 1],
                vec![0, 0, 0, 0],
                vec![1, 1, 0, 1],
            ]
        );
    }
}

fn main() {
    zero_matrix_where_zero(&vec![vec![0]]);
}
