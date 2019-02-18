type Image = Vec<Vec<u32>>;

fn rotate_image_90_degrees(image: &[Vec<u32>]) -> Image {
    let mut rotated_image = image.to_owned();

    let x_len = rotated_image.len();
    for x in 0..(x_len / 2) {
        let y_len = rotated_image[x].len();
        for y in 0..y_len {
            rotated_image[x][y] ^= rotated_image[x_len - x - 1][y];
            rotated_image[x_len - x - 1][y] ^= rotated_image[x][y];
            rotated_image[x][y] ^= rotated_image[x_len - x - 1][y];
        }
    }

    for x in 0..x_len {
        let y_len = rotated_image[x].len();
        for y in 0..y_len {
            if y <= x {
                continue;
            }
            rotated_image[x][y] ^= rotated_image[y][x];
            rotated_image[y][x] ^= rotated_image[x][y];
            rotated_image[x][y] ^= rotated_image[y][x];
        }
    }

    rotated_image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_image_90_degrees() {
        let image = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let rotated_image = rotate_image_90_degrees(&image);

        let expected_rotated_image = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];

        assert_eq!(rotated_image, expected_rotated_image);
    }
}

fn main() {
    rotate_image_90_degrees(&[vec![1]]);
}
