use std::u32;

#[derive(PartialEq, Eq, Debug)]
struct Race {
    time: u16,
    record_distance: u16,
}

impl Race {
    fn new(time: u16, record_distance: u16) -> Race {
        Race {
            time,
            record_distance,
        }
    }

    fn calc_ways_to_beat_record(&self) -> u32 {
        let mut time = self.time / 2;
        let mut runtime = time;

        if self.time % 2 != 0 {
            time += 1;
        }

        let mut ways = 0;
        let mut distance = time * runtime;
        while self.record_distance < distance {
            time -= 1;
            runtime += 1;
            ways += 1;
            distance = time * runtime;
        }

        ways *= 2;

        if self.time % 2 == 0 {
            ways -= 1;
        } else {
            ways -= 2;
        }

        ways
    }
}

#[test]
fn test_cal_ways() {
    let race = Race::new(30, 200);
    let expected_ways = 9;

    assert_eq!(expected_ways, race.calc_ways_to_beat_record());

    let race = Race::new(7, 9);
    let expected_ways = 4;

    assert_eq!(expected_ways, race.calc_ways_to_beat_record());
}

fn parse_races(mut time: &str, mut distance: &str) -> Vec<Race> {
    time = time.split(":").last().expect("'Time:' Not Found").trim();
    distance = distance
        .split(":")
        .last()
        .expect("'Distance:' Not Found")
        .trim();

    let times: Vec<u16> = time
        .split_whitespace()
        .map(|numb| numb.parse().expect(format!("{numb} Not a number").as_str()))
        .collect();
    let distances: Vec<u16> = distance
        .split_whitespace()
        .map(|numb| numb.parse().expect(format!("{numb} Not a number").as_str()))
        .collect();

    let mut races: Vec<Race> = Vec::with_capacity(times.len());

    for i in 0..times.len() {
        races.push(Race::new(times[i], distances[i]));
    }

    races
}

#[test]
fn test_parse_races() {
    let time = "Time: 7  15   30";
    let distance = "Distance:  9  40  200";

    let expected_races = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];

    let races = parse_races(time, distance);

    assert_eq!(expected_races, races);
}

fn main() {
    let mut input = include_str!("input.txt").lines();
    let time = input.next().expect("Time");
    let distance = input.next().expect("Distance");
    let races = parse_races(time, distance);

    let ways: Vec<u32> = races
        .iter()
        .map(|race| race.calc_ways_to_beat_record())
        .collect();

    let mut result = 1;
    for x in ways {
        result *= x;
    }

    println!("Result : {result}");
}
