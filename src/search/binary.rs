pub fn binary_search_linear<T: PartialOrd>(array: &[T], value: &T) -> Option<usize> {
    let mut array_size = array.len();
    for index in 0..array_size {
        let middle_index = (index + array_size) / 2;
        if &array[middle_index] == value {
            return Some(middle_index);
        } else if &array[middle_index] > value {
            array_size = middle_index - 1;
        } else {
            array_size = middle_index + 1;
        }
    }
    // Ad-hoc solution for the first element of array
    // Since loop from above does not care about the first element
    if &array[0] == value {
        return Some(0);
    }
    None
}

pub fn binary_search_recursive<T: PartialOrd>(array: &[T], start: usize, end: usize, value: &T) -> Option<usize> {
    let middle_index = (start + end) / 2;
    if &array[middle_index] == value {
        return Some(middle_index);
    } else if middle_index == 0 {
        return None;
    } else if &array[middle_index] > value {
        return binary_search_recursive(&array, 0, middle_index, value);
    } else {
        return binary_search_recursive(&array, middle_index, array.len(), value);
    }
}

#[cfg(test)]
mod test {
    use crate::search::binary::binary_search_linear;

    #[test]
    fn random_vec_test() {
        let random_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Middle
        assert_eq!(binary_search_linear(&random_vec, &5).unwrap(), 5);
        // First
        assert_eq!(binary_search_linear(&random_vec, &0).unwrap(), 0);
        // Last
        assert_eq!(binary_search_linear(&random_vec, &10).unwrap(), 10);
    }

    use crate::search::binary::binary_search_recursive;

    #[test]
    fn random_vec_recursive_test() {
        let random_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Middle
        assert_eq!(binary_search_recursive(&random_vec, 0, random_vec.len(), &5).unwrap(), 5);
        // First
        assert_eq!(binary_search_recursive(&random_vec, 0, random_vec.len(), &0).unwrap(), 0);
        // Last
        assert_eq!(binary_search_recursive(&random_vec, 0, random_vec.len(), &10).unwrap(), 10);
    }

    #[test]
    fn not_sorted_vec_test() {
        let mut not_sorted_vec = vec![1, 2, 5, 3, 16, 4, 0];
        assert_eq!(binary_search_linear(&not_sorted_vec, &0), None);
        assert_eq!(binary_search_recursive(&not_sorted_vec, 0, not_sorted_vec.len(), &0), None);
        use crate::sort::insertion::insertion_sort;
        insertion_sort(&mut not_sorted_vec);
        assert_eq!(binary_search_linear(&not_sorted_vec, &0).unwrap(), 0);
        assert_eq!(binary_search_recursive(&not_sorted_vec, 0, not_sorted_vec.len(), &0).unwrap(), 0);
    }
}
