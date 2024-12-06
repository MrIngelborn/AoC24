use std::fs;

const INPUT_PATH: &str = "src/day1/input_task1.txt";

pub fn main() {
    println!("DAY 1!!!");
    let task1_res = task1(INPUT_PATH);
    println!("Task1: {}", task1_res);
}

fn task1(path: &str) -> i32 {
    let file_data = read_file(path);
    let arrays = parse_file_data(file_data);
    let sorted_left = sort(arrays.0);
    let sorted_right = sort(arrays.1);
    let diff_array = diff_array(sorted_left, sorted_right);
    sum_array(diff_array)
}

fn read_file(path: &str) -> String {
    let data = fs::read(&path).expect("Could not read file");
    String::from_utf8(data).expect("Could not parse file")
}

fn parse_file_data(content: String) -> (Vec<i32>, Vec<i32>) {
    content
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip()
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
    const TEST_INPUT_PATH: &str = "src/day1/test_input_task1.txt";

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
    fn task1_should_sum_diffs_of_sorted_arrays_from_file() {
        fs::read_dir(".").unwrap()
            .for_each(|a| println!("{:?}", a));
        fs::read_dir("src").unwrap()
            .for_each(|a| println!("{:?}", a));
        fs::read_dir("src/day1").unwrap()
            .for_each(|a| println!("{:?}", a));

        let res = task1(TEST_INPUT_PATH);
        assert_eq!(11, res)
    }

    #[test]
    fn read_file_should_read_file() {
        let expected = "\
                3   4\n\
                4   3\n\
                2   5\n\
                1   3\n\
                3   9\n\
                3   3";
        let result = read_file(TEST_INPUT_PATH);
        assert_eq!(expected, result)
    }
    #[test]
    fn parse_file_data_should_create_arrays() {
        let data = "\
                3   4\n\
                4   3\n\
                2   5\n\
                1   3\n\
                3   9\n\
                3   3";
        let res = parse_file_data(data.parse().unwrap());
        assert_eq!([3, 4, 2, 1, 3, 3].to_vec(), res.0);
        assert_eq!([4, 3, 5, 3, 9, 3].to_vec(), res.1);
    }
}
