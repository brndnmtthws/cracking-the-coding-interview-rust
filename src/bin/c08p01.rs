fn triple_step(steps: i32) -> i32 {
    if steps == 1 {
        1
    } else if steps == 2 {
        triple_step(steps - 1) + 1
    } else if steps == 3 {
        triple_step(steps - 1) + triple_step(steps - 2) + 1
    } else {
        triple_step(steps - 1) + triple_step(steps - 2) + triple_step(steps - 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple_step() {
        assert_eq!(triple_step(1), 1);
        assert_eq!(triple_step(2), 2);
        assert_eq!(triple_step(3), 4);
        assert_eq!(triple_step(4), 7);
        assert_eq!(triple_step(5), 13);
    }
}

fn main() {
    triple_step(1);
}
