use std::cmp::min;

pub fn jump_search<T: PartialOrd>(array: &[T], value: &T) -> Option<usize> {
    let step = (array.len() as f32).sqrt() as usize;
    let mut index = step;
    let mut prev_index = 0;
    while &array[min(index, array.len()) - 1] < value {
        prev_index = index;
        index += step;
        if prev_index > array.len() {
            return None;
        }
    }
    for i in prev_index..(min(index, array.len())) {
        if &array[i] == value {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::search::jump::jump_search;

    #[test]
    fn random_vec_test() {
        let random_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Middle
        assert_eq!(jump_search(&random_vec, &5).unwrap(), 5);
        // First
        assert_eq!(jump_search(&random_vec, &0).unwrap(), 0);
        // Last
        assert_eq!(jump_search(&random_vec, &10).unwrap(), 10);
    }

    #[test]
    fn not_sorted_vec_test() {
        let mut not_sorted_vec = vec![1, 2, 5, 3, 16, 4, 0];
        assert_eq!(jump_search(&not_sorted_vec, &0), None);
        use crate::sort::insertion::insertion_sort;
        insertion_sort(&mut not_sorted_vec);
        assert_eq!(jump_search(&not_sorted_vec, &0).unwrap(), 0);
    }
}
