#[derive(Clone, Debug, Eq, PartialEq)]
struct BremBox {
    height: usize,
    width: usize,
    depth: usize,
}

fn can_be_placed(stack: &[BremBox], top_box: &BremBox) -> bool {
    if let Some(b) = stack.last() {
        top_box.height >= b.height && top_box.width >= b.width && top_box.depth >= b.depth
    } else {
        true
    }
}

fn get_max_stack_height(stack: &[BremBox], remaining_boxes: &[BremBox]) -> usize {
    if remaining_boxes.is_empty() {
        stack.iter().map(|b| b.height).sum()
    } else {
        let mut stacks: Vec<usize> = vec![];
        for (idx, b) in remaining_boxes.iter().enumerate() {
            if can_be_placed(stack, b) {
                let mut new_stack = stack.to_owned();
                new_stack.push(b.clone());
                let mut new_remaining_boxes = remaining_boxes.to_owned();
                new_remaining_boxes.remove(idx);
                stacks.push(get_max_stack_height(&new_stack, &new_remaining_boxes));
            }
        }
        if let Some(m) = stacks.iter().max() {
            *m
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_of_boxes() {
        let stack = vec![
            BremBox {
                height: 1,
                width: 1,
                depth: 1,
            },
            BremBox {
                height: 2,
                width: 2,
                depth: 2,
            },
            BremBox {
                height: 3,
                width: 3,
                depth: 3,
            },
        ];
        let result = get_max_stack_height(&[], &stack);
        assert_eq!(result, 6);
    }
}

fn main() {
    let stack = vec![
        BremBox {
            height: 1,
            width: 1,
            depth: 1,
        },
        BremBox {
            height: 2,
            width: 2,
            depth: 2,
        },
        BremBox {
            height: 3,
            width: 3,
            depth: 3,
        },
    ];
    get_max_stack_height(&[], &stack);
}
