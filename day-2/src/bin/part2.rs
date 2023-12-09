use std::i32;

#[derive(Clone)]
struct Set {
    blue: i32,
    green: i32,
    red: i32,
}

impl Set {
    fn parse(input: &str) -> Self {
        let cubes: Vec<&str> = input.split(',').collect();
        let mut set = Set {
            blue: 0,
            green: 0,
            red: 0,
        };

        for cube in cubes {
            let color_count: Vec<&str> = cube.trim().split(' ').collect();

            if color_count.len() != 2 {
                panic!("Invalid input");
            }

            let color = color_count[1];
            let count = color_count[0]
                .parse::<i32>()
                .expect(format!("Invalid Input : `{}`", cube).as_str());

            match color {
                "red" => set.red = count,
                "green" => set.green = count,
                "blue" => set.blue = count,
                _ => panic!("Invalid color"),
            }
        }

        set
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

#[test]
fn test_parse() {
    let input = " 1 red, 2 green, 3 blue ";
    let set = Set::parse(input);

    assert_eq!(set.red, 1);
    assert_eq!(set.green, 2);
    assert_eq!(set.blue, 3);
}

struct Game {
    sets: Vec<Set>,
}
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
impl Game {
    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split(':').collect();
        let mut sets = Vec::new();

        for set in parts[1].split(';') {
            sets.push(Set::parse(set));
        }

        Game { sets }
    }

    fn min_playable_set(&self) -> Set {
        let mut min_set = (&self.sets[0]).clone();

        for set in &self.sets {
            if set.blue > min_set.blue {
                min_set.blue = set.blue;
            }

            if set.green > min_set.green {
                min_set.green = set.green;
            }

            if set.red > min_set.red {
                min_set.red = set.red;
            }
        }

        min_set
    }
}

#[test]
fn test_game_parse() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = Game::parse(input);

    assert_eq!(game.id, 1);
    assert_eq!(game.sets.len(), 3);
    assert_eq!(game.sets[0].red, 4);
    assert_eq!(game.sets[0].blue, 3);
    assert_eq!(game.sets[1].red, 1);
    assert_eq!(game.sets[1].green, 2);
    assert_eq!(game.sets[1].blue, 6);
    assert_eq!(game.sets[2].green, 2);
}

#[test]
fn test_min_playable_set() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = Game::parse(input);

    let min_set = game.min_playable_set();

    assert_eq!(min_set.red, 4);
    assert_eq!(min_set.green, 2);
    assert_eq!(min_set.blue, 6);
}

fn main() {
    let content = include_str!("./input2.txt");

    println!(
        "Sum of min viable power : {}",
        get_min_viable_power_sum(content)
    );
}

fn get_min_viable_power_sum(input: &str) -> i32 {
    let mut sum = 0;

    for game in input.lines() {
        let game = Game::parse(game);
        let min_set = game.min_playable_set();

        sum += min_set.power();
    }

    sum
}
