use std::iter::once_with;
use std::ops::RangeInclusive;

fn read_seeds(line: &str) -> Option<Vec<i64>> {
    let mut seeds = Vec::new();

    let numbers: Vec<i64> = line
        .split(':')
        .last()?
        .trim()
        .split(' ')
        .map(|seed| seed.parse().expect("Seed is not a number"))
        .collect();

    let mut index = 0;
    while index < numbers.len() - 1 {
        let initial_number = numbers[index];
        let length = numbers[index + 1];

        for i in initial_number..initial_number + length {
            seeds.push(i);
        }

        index += 2
    }

    Some(seeds)
}

fn read_map(line: &str) -> (RangeInclusive<i64>, i64) {
    let numbers = line
        .split(' ')
        .map(|n| n.parse().expect("Number is not a number"))
        .collect::<Vec<i64>>();
    let range = numbers[1]..=numbers[1] + numbers[2];
    (range, numbers[0] - numbers[1])
}

fn flatten_fill(seeds: Vec<i64>, mapped_seeds: &Vec<Option<i64>>) -> Vec<i64> {
    mapped_seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| {
            if let Some(number) = seed {
                *number
            } else {
                seeds[index]
            }
        })
        .collect()
}

fn main() {
    let mut input = include_str!("input.txt").lines();
    let mut seeds = read_seeds(input.next().unwrap()).unwrap();
    let mut mapped_seeds = vec![None; seeds.len()];
    input.next();
    input.next();

    while let Some(line) = input.next() {
        if line.is_empty() {
            continue;
        }

        if line.contains("map") {
            seeds = flatten_fill(seeds, &mapped_seeds);
            mapped_seeds = vec![None; seeds.len()];
            continue;
        }

        let (range, offset) = read_map(line);

        for (index, number) in seeds.iter().enumerate() {
            if mapped_seeds[index].is_none() {
                if range.contains(number) {
                    mapped_seeds[index] = Some(number + offset);
                }
            }
        }
    }

    seeds = flatten_fill(seeds, &mapped_seeds);
    seeds.sort_unstable();

    println!("Lowest {}", seeds.first().expect("Something..."))
}
