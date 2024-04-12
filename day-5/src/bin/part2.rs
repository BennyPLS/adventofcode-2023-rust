type Range = (i64, i64);

fn range_contains(container: Range, contains: Range, offset: i64) -> Option<(Vec<Range>, Range)> {
    dbg!(container, contains);
    if container.0 <= contains.0 && contains.1 <= container.1 {
        Some((vec![], range_offset(contains, offset)))
    } else if contains.0 < container.0 && container.1 < contains.1 {
        dbg!(contains.0 < container.0 && container.1 < contains.1);
        Some((
            vec![(contains.0, container.0 - 1), (container.1 + 1, contains.1)],
            range_offset(container, offset),
        ))
    } else if container.0 > contains.0 && contains.1 <= container.1 && contains.1 >= container.0 {
        dbg!(container.0 > contains.0 && contains.1 <= container.1 && contains.1 >= container.0);
        Some((
            vec![(contains.0, container.0 - 1)],
            range_offset((container.0, contains.1), offset),
        ))
    } else if container.0 <= contains.0 && contains.1 > container.1 && contains.0 <= container.1 {
        dbg!(container.0 <= contains.0 && contains.1 > container.1 && contains.0 <= container.1);
        Some((
            vec![(container.1 + 1, contains.1)],
            range_offset((contains.0, container.1), offset),
        ))
    } else {
        None
    }
}

fn range_offset(mut range: Range, offset: i64) -> Range {
    range.0 += offset;
    range.1 += offset;

    range
}

fn read_seeds(line: &str) -> Option<Vec<Range>> {
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
        seeds.push((initial_number, initial_number + length - 1));
        index += 2
    }

    Some(seeds)
}

fn read_map(line: &str) -> (Range, i64) {
    let numbers = line
        .split(' ')
        .map(|n| n.parse().expect("Number is not a number"))
        .collect::<Vec<i64>>();
    let range: Range = (numbers[1], numbers[1] + numbers[2]);
    (range, numbers[0] - numbers[1])
}

fn main() {
    let mut input = include_str!("sample.txt").lines();
    let mut seeds = read_seeds(input.next().unwrap()).unwrap();
    let mut mapping_stack = vec![];
    input.next();
    input.next();

    while let Some(line) = input.next() {
        if line.is_empty() {
            continue;
        }

        if line.contains("map") {
            mapping_stack.push(None);
            continue;
        }

        let mapping = read_map(line);
        mapping_stack.push(Some(mapping));
    }

    let mut mapped_seeds = vec![];

    dbg!(&seeds);

    for mapping in mapping_stack {
        if mapping.is_none() {
            seeds.append(&mut mapped_seeds);
            seeds.sort_unstable();
            dbg!(&seeds);
            continue;
        }

        let (range, offset) = mapping.unwrap();
        let mut iter_seeds: Vec<Range> = vec![];

        for num_range in seeds {
            if let Some((mut result, range_offset)) = range_contains(range, num_range, offset) {
                println!("match");
                mapped_seeds.push(range_offset);
                mapped_seeds.append(&mut result);
            } else {
                iter_seeds.push(num_range);
            }
        }

        seeds = iter_seeds;
        seeds.sort_unstable();
    }
    seeds.append(&mut mapped_seeds);
    seeds.sort_unstable();

    dbg!(&seeds);

    println!("Lowest : {}", seeds.first().unwrap().0)
}

// TODO Not Completed :c
