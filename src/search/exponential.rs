use crate::search::binary::binary_search_recursive;

pub fn exponential_search<T: PartialOrd>(array: &[T], value: &T) -> Option<usize> {
    let mut index = 1;
    while index < array.len() && &array[index] < value {
        index *= 2;
    }
    return binary_search_recursive(array, index / 2, std::cmp::min(index, array.len()), value);
}

#[cfg(test)]
mod test {
    use crate::search::exponential::exponential_search;

    #[test]
    fn random_vec_test() {
        let random_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Middle
        assert_eq!(exponential_search(&random_vec, &5).unwrap(), 5);
        // First
        assert_eq!(exponential_search(&random_vec, &0).unwrap(), 0);
        // Last
        assert_eq!(exponential_search(&random_vec, &10).unwrap(), 10);
    }

    #[test]
    fn not_sorted_vec_test() {
        let mut not_sorted_vec = vec![1, 2, 5, 3, 16, 4, 0];
        assert_eq!(exponential_search(&not_sorted_vec, &0), None);
        use crate::sort::insertion::insertion_sort;
        insertion_sort(&mut not_sorted_vec);
        assert_eq!(exponential_search(&not_sorted_vec, &0).unwrap(), 0);
    }
}
