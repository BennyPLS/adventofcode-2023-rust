use std::iter::Iterator;

fn main() {
    let result = calibrate(include_str!("input.txt"));

    println!("Calibration: {}", result);
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
    let expected_result = 142;
    let content = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

    assert_eq!(expected_result, calibrate(content));
}

fn get_number(line: &str) -> i32 {
    let mut number = String::new();
    let chars = line.chars().into_iter();

    number.push(chars.clone().find(is_number).unwrap_or(' '));
    number.push(chars.clone().rfind(is_number).unwrap_or(' '));

    number.parse::<i32>().unwrap_or(0)
}

#[test]
fn get_number_test() {
    let expected_results = vec![12, 38, 15, 77];
    let lines = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines.iter() {
        numbers.push(get_number(line));
    }

    assert_eq!(expected_results, numbers);
}

fn is_number(c: &char) -> bool {
    return c.is_digit(10);
}
