pub fn insertion_sort<T: PartialOrd>(array: &mut Vec<T>) {
    for mut index in 0..array.len() - 1 {
        while array[index] > array[index + 1] {
            array.swap(index, index + 1);
            if index > 0 {
                index -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::insertion::insertion_sort;

    #[test]
    fn random_vec_test() {
        let mut random_vec = vec![5, 6, 3, 1, 2, 6, 7, 8, 9, 0, 4, 3, 25, 6, 7, 23, 67, 5, 10];
        insertion_sort(&mut random_vec);
        assert_eq!(random_vec, vec![0, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 7, 7, 8, 9, 10, 23, 25, 67]);
    }

    #[test]
    fn descendant_vec_test() {
        let mut descendant_vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        insertion_sort(&mut descendant_vec);
        assert_eq!(descendant_vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn not_changed_vec_test() {
        let mut not_changed_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        insertion_sort(&mut not_changed_vec);
        assert_eq!(not_changed_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn first_elem_test() {
        let mut first_elem_vec = vec![5, 1, 2, 3, 4];
        insertion_sort(&mut first_elem_vec);
        assert_eq!(first_elem_vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn last_elem_test() {
        let mut last_elem_vec = vec![1, 2, 3, 4, 5, 0];
        insertion_sort(&mut last_elem_vec);
        assert_eq!(last_elem_vec, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn float_elem_test() {
        let mut float_vec = vec![0.2, 0.1, 0.6, 0.5, 0.5, 0.7, 0.2];
        insertion_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.1, 0.2, 0.2, 0.5, 0.5, 0.6, 0.7]);
    }

    #[test]
    fn char_elem_test() {
        let mut char_vec = vec!['c', 'a', 'b', 'h', 'y', 'z', 'x'];
        insertion_sort(&mut char_vec);
        assert_eq!(char_vec, vec!['a', 'b', 'c', 'h', 'x', 'y', 'z']);
    }

    #[derive(PartialOrd, PartialEq, Debug)]
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
        insertion_sort(&mut custom_type_vec);
        assert_eq!(custom_type_vec, vec![
            ExampleStruct { some_string: String::from("A") },
            ExampleStruct { some_string: String::from("B") },
            ExampleStruct { some_string: String::from("C") },
            ExampleStruct { some_string: String::from("F") }
        ]);
    }
}