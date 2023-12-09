use std::iter::Iterator;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let content = include_str!("./input2.txt");
    let result = calibrate(content);
    println!("Calibration: {}", result)
}

fn calibrate(content: &str) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();

    for line in content.lines() {
        numbers.push(get_number(line));
    }

    numbers.iter().sum::<i32>()
}

#[test]
fn calibrate_test() {
    let expected_result = 281;
    let content =
        "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    assert_eq!(expected_result, calibrate(content));
}

fn get_number(line: &str) -> i32 {
    let mut number = String::new();

    number.push(first_number(line));
    number.push(last_number(line));

    number.parse::<i32>().unwrap_or(0)
}

fn first_number(line: &str) -> char {
    let mut char = ' ';
    let mut first_text = line.len();

    for (index, search) in NUMBERS.iter().enumerate() {
        if let Some(search_result) = line.rfind(search) {
            if search_result < first_text {
                first_text = search_result;
                char = (index + 1).to_string().chars().next().unwrap_or('0');
            }
        }
    }

    let chars: Vec<char> = line.chars().collect();
    for i in 0..first_text {
        if chars[i].is_digit(10) {
            char = chars[i];
            break;
        }
    }

    return char;
}

fn last_number(line: &str) -> char {
    let mut char = ' ';
    let mut first_text = 0;

    for (index, search) in NUMBERS.iter().enumerate() {
        if let Some(search_result) = line.find(search) {
            if search_result > first_text {
                first_text = search_result;
                char = (index + 1).to_string().chars().next().unwrap_or('0');
            }
        }
    }

    let chars: Vec<char> = line.chars().collect();
    for i in (first_text..line.len()).rev() {
        if chars[i].is_digit(10) {
            char = chars[i];
            break;
        }
    }

    return char;
}

#[test]
fn get_number_test() {
    let expected_results = vec![29, 83, 13, 24, 42, 14, 76];
    let lines = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines.iter() {
        numbers.push(get_number(line));
    }

    assert_eq!(expected_results, numbers);
}
