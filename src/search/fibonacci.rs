pub fn fibonacci_search<T: PartialOrd>(array: &[T], value: &T) -> Option<usize> {
    let (mut second_prev, mut prev, mut closest) = get_fib_closest_to(array.len() as i32);
    if closest == 1 {
        if &array[closest as usize] == value {
            return Some(closest as usize);
        }
    }
    let mut offset: i32 = -1;
    while closest > 1 {
        let index = std::cmp::min((offset + second_prev) as usize, array.len() - 1);
        if &array[index] < value {
            closest = prev;
            prev = second_prev;
            second_prev = closest - prev;
            offset = index as i32;
        } else if &array[index] > value {
            closest = second_prev;
            prev -= second_prev;
            second_prev = closest - prev;
        } else {
            return Some(index);
        }
    }
    if closest != 0 && &array[(offset + 1) as usize] == value {
        return Some(offset as usize + 1);
    }
    None
}

fn get_fib_closest_to(number: i32) -> (i32, i32, i32) {
    if number == 1 {
        return (0, 1, 1);
    }
    let mut last = 1;
    let mut penultimate = 1;
    loop {
        let current = last + penultimate;
        if current >= number {
            return (penultimate, last, current);
        }
        penultimate = last;
        last = current;
    }
}

#[cfg(test)]
mod test {
    use crate::search::fibonacci::fibonacci_search;

    #[test]
    fn random_vec_test() {
        let random_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Middle
        assert_eq!(fibonacci_search(&random_vec, &5).unwrap(), 5);
        // First
        assert_eq!(fibonacci_search(&random_vec, &0).unwrap(), 0);
        // Last
        assert_eq!(fibonacci_search(&random_vec, &10).unwrap(), 10);
    }

    #[test]
    fn not_sorted_vec_test() {
        let mut not_sorted_vec = vec![1, 2, 5, 3, 16, 4, 0];
        assert_eq!(fibonacci_search(&not_sorted_vec, &0), None);
        use crate::sort::insertion::insertion_sort;
        insertion_sort(&mut not_sorted_vec);
        assert_eq!(fibonacci_search(&not_sorted_vec, &0).unwrap(), 0);
    }
}
