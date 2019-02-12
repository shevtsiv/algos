pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 1..array.len() {
            if array[i] < array[i - 1] {
                array.swap(i, i - 1);
                sorted = false;
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