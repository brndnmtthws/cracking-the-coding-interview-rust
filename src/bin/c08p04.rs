fn power_set(arr: &[i64]) -> Vec<Vec<i64>> {
    if arr.is_empty() {
        vec![vec![]]
    } else {
        let mut subsets = power_set(&arr[1..]);
        let item = arr[0];
        let mut new_subsets: Vec<Vec<i64>> = subsets
            .iter()
            .map(|subset| {
                let mut new_subset = subset.clone();
                new_subset.insert(0, item);
                new_subset
            })
            .collect();
        subsets.append(&mut new_subsets);
        subsets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_set() {
        assert_eq!(
            power_set(&[0, 1, 2]),
            vec![
                vec![],
                vec![2],
                vec![1],
                vec![1, 2],
                vec![0],
                vec![0, 2],
                vec![0, 1],
                vec![0, 1, 2],
            ]
        );
    }
}

fn main() {
    power_set(&[0, 1, 2]);
}
