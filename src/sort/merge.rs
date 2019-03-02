pub fn merge_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }
    let middle = array.len() / 2;
    let mut first_part = Vec::from(&array[..middle]);
    let mut second_part = Vec::from(&array[middle..]);
    merge_sort(&mut first_part);
    merge_sort(&mut second_part);
    let mut insertion_index = 0;
    let i = 0;
    let j = 0;
    while i < first_part.len() && j < second_part.len() {
        if first_part[i] < second_part[j] {
            array[insertion_index] = first_part.remove(i);
            insertion_index += 1;
        } else {
            array[insertion_index] = second_part.remove(j);
            insertion_index += 1;
        }
    }
    while i < first_part.len() {
        array[insertion_index] = first_part.remove(i);
        insertion_index += 1;
    }
    while j < second_part.len() {
        array[insertion_index] = second_part.remove(j);
        insertion_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::merge::merge_sort;

    #[test]
    fn random_vec_test() {
        let mut random_vec = vec![5, 6, 3, 1, 2, 6, 7, 8, 9, 0, 4, 3, 25, 6, 7, 23, 67, 5, 10];
        merge_sort(&mut random_vec);
        assert_eq!(random_vec, vec![0, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 7, 7, 8, 9, 10, 23, 25, 67]);
    }

    #[test]
    fn descendant_vec_test() {
        let mut descendant_vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        merge_sort(&mut descendant_vec);
        assert_eq!(descendant_vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn not_changed_vec_test() {
        let mut not_changed_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        merge_sort(&mut not_changed_vec);
        assert_eq!(not_changed_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn first_elem_test() {
        let mut first_elem_vec = vec![5, 1, 2, 3, 4];
        merge_sort(&mut first_elem_vec);
        assert_eq!(first_elem_vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn last_elem_test() {
        let mut last_elem_vec = vec![1, 2, 3, 4, 5, 0];
        merge_sort(&mut last_elem_vec);
        assert_eq!(last_elem_vec, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn float_elem_test() {
        let mut float_vec = vec![0.2, 0.1, 0.6, 0.5, 0.5, 0.7, 0.2];
        merge_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.1, 0.2, 0.2, 0.5, 0.5, 0.6, 0.7]);
    }

    #[test]
    fn char_elem_test() {
        let mut char_vec = vec!['c', 'a', 'b', 'h', 'y', 'z', 'x'];
        merge_sort(&mut char_vec);
        assert_eq!(char_vec, vec!['a', 'b', 'c', 'h', 'x', 'y', 'z']);
    }

    #[derive(Clone, PartialOrd, PartialEq, Debug)]
    struct ExampleStruct {
        some_string: String
    }

    #[test]
    fn custom_type_elem_test() {
        let mut custom_type_vec = vec![
            ExampleStruct { some_string: String::from("B") },
            ExampleStruct { some_string: String::from("A") },
            ExampleStruct { some_string: String::from("F") },
            ExampleStruct { some_string: String::from("C") }
        ];
        merge_sort(&mut custom_type_vec);
        assert_eq!(custom_type_vec, vec![
            ExampleStruct { some_string: String::from("A") },
            ExampleStruct { some_string: String::from("B") },
            ExampleStruct { some_string: String::from("C") },
            ExampleStruct { some_string: String::from("F") }
        ]);
    }
}