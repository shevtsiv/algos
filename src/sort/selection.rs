pub fn selection_sort(array: &mut Vec<i32>) {
    for index in 0..array.len() - 1 {
        let lowest_index = get_lowest_index(array, index + 1);
        if array[index] > array[lowest_index] {
            let temp = array[lowest_index];
            array[lowest_index] = array[index];
            array[index] = temp;
        }
    }
}

fn get_lowest_index(array: &Vec<i32>, start_from: usize) -> usize {
    let mut smallest_index = start_from;
    for i in start_from..array.len() {
        if array[i] < array[smallest_index] {
            smallest_index = i;
        }
    }
    smallest_index
}
