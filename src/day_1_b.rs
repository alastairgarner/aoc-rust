use std::fs;

fn read_data(path: &str) -> Vec<i32> {
    let file_path = format!("data/{path}/day_1.txt");
    let content: Vec<i32> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    content
}

fn count_window_increasing(values: Vec<i32>) -> i32 {
    let mut count = 0;
    let window_sums: Vec<i32> = values.windows(3).map(|s| s.iter().sum::<i32>()).collect();

    for pair in window_sums.windows(2) {
        if pair[0] < pair[1] {
            count += 1
        }
    }

    count
}

pub fn solve(data_source: &str) {
    let values = read_data(data_source);
    let count = count_window_increasing(values);
    println!("Count {count}");
}
