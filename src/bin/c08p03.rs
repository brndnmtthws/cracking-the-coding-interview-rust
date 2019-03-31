fn find_magic_index(arr: &[i64]) -> Option<usize> {
    let middle_index = arr.len() / 2;
    let middle_value = arr[middle_index];
    if (middle_index as i64) == middle_value {
        Some(middle_index)
    } else if (middle_index as i64) < middle_value && arr.len() >= 2 {
        // search left
        find_magic_index(&arr[..middle_index])
    } else if arr.len() >= 2 {
        // search right
        find_magic_index(&arr[middle_index..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_index() {
        assert_eq!(find_magic_index(&[0, 2, 3, 5]), Some(0));
        assert_eq!(find_magic_index(&[-10, 2, 3, 5]), None);
    }
}

fn main() {
    find_magic_index(&[0, 2, 3, 5]);
}
