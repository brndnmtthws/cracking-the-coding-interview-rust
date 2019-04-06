fn paint_fill(x: usize, y: usize, canvas: &mut Vec<Vec<i32>>, new_colour: i32) {
    let prev_colour = canvas[x][y];
    canvas[x][y] = new_colour;

    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            if x_offset == 0 && y_offset == 0 {
                continue;
            }
            let x = ((x as i64) + x_offset) as usize;
            let y = ((y as i64) + y_offset) as usize;
            if x < canvas.len() && y < canvas[0].len() && canvas[x][y] == prev_colour {
                paint_fill(x, y, canvas, new_colour);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paint_fill() {
        let mut canvas = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        paint_fill(0, 0, &mut canvas, 2);
        assert_eq!(
            canvas,
            vec![
                vec![2, 2, 1, 0, 0],
                vec![2, 2, 1, 0, 0],
                vec![1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
            ]
        );
    }
}

fn main() {
    let mut canvas = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    paint_fill(0, 0, &mut canvas, 2);
}
