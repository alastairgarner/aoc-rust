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

fn count_increasing(values: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previous_value: i32 = values[0].clone();

    for (i, current_value) in values.iter().enumerate() {
        let direction;

        if i == 0 {
            direction = "N/A - no previous measurement"
        } else if current_value > &previous_value {
            direction = "increased";
            count += 1
        } else if current_value < &previous_value {
            direction = "decreased";
        } else {
            direction = "no change"
        }
        println!("{} ({})", current_value, direction);

        previous_value = values[i]
    }

    count
}

// fn simplified() {
//     let file_name = "data/test/day_1.txt";
//     let content = fs::read_to_string(file_name)
//         .unwrap()
//         .lines()
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect::<Vec<i32>>()
//         .windows(2)
//         .filter(|a| a[0] < a[1])
//         .count();

//     println!("{content:?}");
// }

pub fn solve(data_source: &str) {
    let values = read_data(data_source);
    println!("{values:?}");

    let count = count_increasing(values);
    println!("Count {count}");
}
