#[test]
fn find_7() {
    assert_eq!(search(&vec![1, 2, 3, 4, 5, 6, 7, 8], 7), Some(6))
}

#[test]
fn not_find_7() {
    assert_eq!(search(&vec![0, 1, 2, 3, 4, 8, 10, 25], 7), None)
}

#[test]
fn not_get_stuck() {
    assert_eq!(search(&vec![0, 1, 2, 3, 4, 8, 10], 7), None)
}

fn search(haystack: &[i64], needle: i64) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = haystack.len() - 1;

    loop {
        let center = left + ((right - left) / 2);
        let current = haystack[center];

        println!("{:?}", (left, right, center, current));

        if left == right {
            return None;
        }

        if current == needle {
            return Some(center);
        }

        if current > needle {
            right = center - 1;
        }

        if current < needle {
            left = center + 1;
        }
    }
}
