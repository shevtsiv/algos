pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    while !array.is_sorted() {
        for i in 1..array.len() {
            if array[i] < array[i - 1] {
                array.swap(i, i - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::bubble::bubble_sort;

    #[test]
    fn random_vec_test() {
        let mut random_vec = vec![6, 2, 7, 5, 0, -2, 5];
        bubble_sort(&mut random_vec);
        assert_eq!(random_vec, vec![-2, 0, 2, 5, 5, 6, 7]);
    }
}