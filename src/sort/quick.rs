pub fn quick_sort<T: PartialOrd>(mut array: &mut [T], start: usize, end: usize) {
    if start >= end {
        return;
    }
    let mid = partition(&mut array, start, end);
    quick_sort(&mut array, start, mid);
    quick_sort(&mut array, mid + 1, end);
}

fn partition<T: PartialOrd>(array: &mut [T], mut start: usize, end: usize) -> usize {
    for index in start..end - 1 {
        if array[index] <= array[end - 1] {
            array.swap(index, start);
            start += 1;
        }
    }
    array.swap(start, end - 1);
    start
}

#[cfg(test)]
mod tests {
    use crate::sort::quick::quick_sort;

    #[test]
    fn random_vec_test() {
        let mut random_vec = vec![5, 6, 3, 1, 2, 6, 7, 8, 9, 0, 4, 3, 25, 6, 7, 23, 67, 5, 10];
        let len = random_vec.len();
        quick_sort(&mut random_vec, 0, len);
        assert_eq!(random_vec, vec![0, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 7, 7, 8, 9, 10, 23, 25, 67]);
    }

    #[test]
    fn descendant_vec_test() {
        let mut descendant_vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let len = descendant_vec.len();
        quick_sort(&mut descendant_vec, 0, len);
        assert_eq!(descendant_vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn not_changed_vec_test() {
        let mut not_changed_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let len = not_changed_vec.len();
        quick_sort(&mut not_changed_vec, 0, len);
        assert_eq!(not_changed_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn first_elem_test() {
        let mut first_elem_vec = vec![5, 1, 2, 3, 4];
        let len = first_elem_vec.len();
        quick_sort(&mut first_elem_vec, 0, len);
        assert_eq!(first_elem_vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn last_elem_test() {
        let mut last_elem_vec = vec![1, 2, 3, 4, 5, 0];
        let len = last_elem_vec.len();
        quick_sort(&mut last_elem_vec, 0, len);
        assert_eq!(last_elem_vec, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn float_elem_test() {
        let mut float_vec = vec![0.2, 0.1, 0.6, 0.5, 0.5, 0.7, 0.2];
        let len = float_vec.len();
        quick_sort(&mut float_vec, 0, len);
        assert_eq!(float_vec, vec![0.1, 0.2, 0.2, 0.5, 0.5, 0.6, 0.7]);
    }

    #[test]
    fn char_elem_test() {
        let mut char_vec = vec!['c', 'a', 'b', 'h', 'y', 'z', 'x'];
        let len = char_vec.len();
        quick_sort(&mut char_vec, 0, len);
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
        let len = custom_type_vec.len();
        quick_sort(&mut custom_type_vec, 0, len);
        assert_eq!(custom_type_vec, vec![
            ExampleStruct { some_string: String::from("A") },
            ExampleStruct { some_string: String::from("B") },
            ExampleStruct { some_string: String::from("C") },
            ExampleStruct { some_string: String::from("F") }
        ]);
    }
}
