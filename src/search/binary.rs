pub fn binary_search<'a, T: PartialOrd>(array: &'a Vec<T>, value: &'a T) -> Option<(usize, &'a T)> {
    let mut array_size = array.len();
    for index in 0..array_size {
        let middle_index = (index + array_size) / 2;
        if &array[middle_index] == value {
            return Option::from((middle_index, &array[middle_index]));
        } else if &array[middle_index] > value {
            array_size = middle_index - 1;
        } else {
            array_size = middle_index + 1;
        }
    }
    // Ad-hoc solution for the first element of array
    // Since loop from above does not care about the first element
    if &array[0] == value {
        return Option::from((0, &array[0]));
    }
    None
}

#[cfg(test)]
mod test {
    use crate::search::binary::binary_search;

    #[test]
    fn random_vec_test() {
        let random_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Middle
        assert_eq!(binary_search(&random_vec, &5).unwrap(), (5, &5));
        // First
        assert_eq!(binary_search(&random_vec, &0).unwrap(), (0, &0));
        // Last
        assert_eq!(binary_search(&random_vec, &10).unwrap(), (10, &10));
    }
}
