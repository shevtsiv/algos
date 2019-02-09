pub fn binary_search_linear<'a, T: PartialOrd>(array: &'a [T], value: &'a T) -> Option<(usize, &'a T)> {
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

pub fn binary_search_recursive<'a, T: PartialOrd>(array: &'a [T], start: usize, end: usize, value: &'a T) -> Option<(usize, &'a T)> {
    let middle_index = (start + end) / 2;
    if &array[middle_index] == value {
        return Option::from((middle_index, &array[middle_index]));
    } else if array.len() == 1 {
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
        assert_eq!(binary_search_linear(&random_vec, &5).unwrap(), (5, &5));
        // First
        assert_eq!(binary_search_linear(&random_vec, &0).unwrap(), (0, &0));
        // Last
        assert_eq!(binary_search_linear(&random_vec, &10).unwrap(), (10, &10));
    }

    use crate::search::binary::binary_search_recursive;

    #[test]
    fn random_vec_recursive_test() {
        let random_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Middle
        assert_eq!(binary_search_recursive(&random_vec, 0, random_vec.len(), &5).unwrap(), (5, &5));
        // First
        assert_eq!(binary_search_recursive(&random_vec, 0, random_vec.len(), &0).unwrap(), (0, &0));
        // Last
        assert_eq!(binary_search_recursive(&random_vec, 0, random_vec.len(), &10).unwrap(), (10, &10));
    }
}
