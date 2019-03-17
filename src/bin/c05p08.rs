fn draw_line(bytes: &mut Vec<u8>, width: usize, x1: usize, x2: usize, y: usize) {
    let start_pos = y * width + x1;
    let end_pos = y * width + x2;
    for i in start_pos..end_pos {
        bytes[i / 8] |= 1 << (i % 8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_line() {
        let v: u8 = 0;
        let mut vec = vec![v; 8];
        draw_line(&mut vec, 8, 0, 8, 1);
        assert_eq!(vec, vec![0, 255, 0, 0, 0, 0, 0, 0]);
    }
}

fn main() {
    let v: u8 = 0;
    let mut vec = vec![v; 8];
    draw_line(&mut vec, 8, 0, 8, 1);
}
