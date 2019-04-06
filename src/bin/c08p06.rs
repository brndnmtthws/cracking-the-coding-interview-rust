fn move_to_target(n: usize, source: &mut Vec<i32>, target: &mut Vec<i32>, aux: &mut Vec<i32>) {
    if n > 0 {
        move_to_target(n - 1, source, aux, target);
        target.push(source.pop().unwrap());
        move_to_target(n - 1, aux, target, source);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_towers_of_hanoi() {
        let mut source = vec![5, 4, 3, 2, 1];
        let mut target: Vec<i32> = vec![];
        let mut aux: Vec<i32> = vec![];
        move_to_target(source.len(), &mut source, &mut target, &mut aux);
        assert_eq!(source, vec![]);
        assert_eq!(target, vec![5, 4, 3, 2, 1]);
        assert_eq!(aux, vec![]);
    }
}

fn main() {
    let mut source = vec![5, 4, 3, 2, 1];
    let mut target: Vec<i32> = vec![];
    let mut aux: Vec<i32> = vec![];
    move_to_target(source.len(), &mut source, &mut target, &mut aux);
}
