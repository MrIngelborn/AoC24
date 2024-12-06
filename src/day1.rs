pub fn main() {
    println!("DAY 1!!!")
}

fn task1(array1: Vec<i32>, array2: Vec<i32>) -> i32 {
    sum_array(diff_array(sort(array1), sort(array2)))
}

fn sort(array: Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::new();
    sorted.clone_from(&array);
    sorted.sort();
    sorted
}

fn diff(a: i32, b: i32) -> i32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn diff_array(left_array: Vec<i32>, right_array: Vec<i32>) -> Vec<i32> {
    left_array
        .iter()
        .zip(right_array)
        .map(|(a, b)| diff(*a, b))
        .collect()
}

fn sum_array(array: Vec<i32>) -> i32 {
    array.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_should_sort_array() {
        let array = [3, 4, 2, 1, 3, 3].to_vec();
        let result = sort(array);
        assert_eq!([1, 2, 3, 3, 3, 4].to_vec(), result);
    }

    #[test]
    fn diff_should_return_gap_between() {
        let diff1 = diff(3, 1);
        assert_eq!(2, diff1);
        let diff2 = diff(1, 4);
        assert_eq!(3, diff2);
    }

    #[test]
    fn diff_array_should_collect_diffs() {
        let res = diff_array([3, 4, 2, 1, 3, 3].to_vec(), [4, 3, 5, 3, 9, 3].to_vec());
        assert_eq!([1, 1, 3, 2, 6, 0].to_vec(), res)
    }

    #[test]
    fn sum_array_should_sum() {
        let res = sum_array([1, 1, 3, 2, 6, 0].to_vec());
        assert_eq!(13, res)
    }

    #[test]
    fn task1_should_sum_diffs_of_sorted_arrays() {
        let res = task1(
            [3, 4, 2, 1, 3, 3].to_vec(),
            [4, 3, 5, 3, 9, 3].to_vec()
        );
        assert_eq!(
            11,
            res
        )
    }
}
