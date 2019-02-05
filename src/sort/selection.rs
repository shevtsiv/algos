pub fn selection_sort(array: &mut Vec<i32>) {
    for index in 0..array.len() - 1 {
        let lowest_index = get_lowest_index(array, index + 1);
        if array[index] > array[lowest_index] {
            let temp = array[lowest_index];
            array[lowest_index] = array[index];
            array[index] = temp;
        }
    }
}

fn get_lowest_index(array: &Vec<i32>, start_from: usize) -> usize {
    let mut smallest_index = start_from;
    for i in start_from..array.len() {
        if array[i] < array[smallest_index] {
            smallest_index = i;
        }
    }
    smallest_index
}

#[cfg(test)]
mod tests {
    use crate::sort::selection::selection_sort;

    #[test]
    fn random_vec_test() {
        let mut random_vec = vec![5, 6, 3, 1, 2, 6, 7, 8, 9, 0, 4, 3, 25, 6, 7, 23, 67, 5, 10];
        selection_sort(&mut random_vec);
        assert_eq!(random_vec, vec![0, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 7, 7, 8, 9, 10, 23, 25, 67]);
    }

    #[test]
    fn descendant_vec_test() {
        let mut descendant_vec = vec![10, 9 , 8, 7, 6, 5, 4, 3, 2, 1, 0];
        selection_sort(&mut descendant_vec);
        assert_eq!(descendant_vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn not_changed_vec_test() {
        let mut not_changed_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        selection_sort(&mut not_changed_vec);
        assert_eq!(not_changed_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn first_elem_test() {
        let mut first_elem_vec = vec![5, 1, 2, 3, 4];
        selection_sort(&mut first_elem_vec);
        assert_eq!(first_elem_vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn last_elem_test() {
        let mut last_elem_vec = vec![1, 2, 3, 4, 5, 0];
        selection_sort(&mut last_elem_vec);
        assert_eq!(last_elem_vec, vec![0, 1, 2, 3, 4, 5]);
    }
}
