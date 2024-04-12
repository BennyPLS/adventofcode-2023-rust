use std::u32;

#[derive(PartialEq, Eq, Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn new(time: u64, record_distance: u64) -> Race {
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

fn parse_race(mut time: &str, mut distance: &str) -> Race {
    time = time.split(":").last().expect("'Time:' Not Found").trim();
    distance = distance
        .split(":")
        .last()
        .expect("'Distance:' Not Found")
        .trim();

    let time: u64 = time
        .replace(" ", "")
        .parse()
        .expect("Time var is not a number");
    let distance: u64 = distance
        .replace(" ", "")
        .parse()
        .expect("Distance Not a number");

    Race::new(time, distance)
}

#[test]
fn test_parse_race() {
    let time = "Time: 7  15   30";
    let distance = "Distance:  9  40  200";

    let expected_race = Race::new(71530, 940200);

    let race = parse_race(time, distance);

    assert_eq!(expected_race, race);
}

fn main() {
    let mut input = include_str!("input.txt").lines();
    let time = input.next().expect("Time");
    let distance = input.next().expect("Distance");
    let race = parse_race(time, distance);

    println!("Result : {}", race.calc_ways_to_beat_record());
}
