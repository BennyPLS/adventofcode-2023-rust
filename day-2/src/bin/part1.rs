const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

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

    fn is_valid(&self) -> bool {
        self.blue <= MAX_BLUE && self.green <= MAX_GREEN && self.red <= MAX_RED
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
    id: i32,
    sets: Vec<Set>,
}
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
impl Game {
    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split(':').collect();

        let id = parts[0]
            .split(' ')
            .nth(1)
            .expect(format!("Invalid Input : `{}`", parts[0]).as_str())
            .parse()
            .expect(format!("Invalid Input : `{}`", parts[0]).as_str());

        let mut sets = Vec::new();

        for set in parts[1].split(';') {
            sets.push(Set::parse(set));
        }

        Game { id, sets }
    }

    fn is_valid(&self) -> bool {
        for set in &self.sets {
            if !set.is_valid() {
                return false;
            }
        }

        true
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

fn main() {
    let content = include_str!("input.txt");

    println!("Sum of valid games : {}", sum_valid_games(content));
}

fn sum_valid_games(input: &str) -> i32 {
    let mut sum = 0;

    for game in input.lines() {
        let game = Game::parse(game);

        if game.is_valid() {
            sum += game.id;
        }
    }

    sum
}

#[test]
fn test_sum_valid_games() {
    let expected = 1 + 2 + 3 + 4 + 5;
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 red, 1 green, 2 blue; 2 red, 3 green, 1 blue; 1 red, 2 green, 1 blue
    Game 3: 1 red, 1 green, 2 blue; 2 red, 3 green, 1 blue; 1 red, 2 green, 1 blue
    Game 4: 1 red, 1 green, 2 blue; 2 red, 3 green, 1 blue; 1 red, 2 green, 1 blue
    Game 5: 1 red, 1 green, 2 blue; 2 red, 3 green, 1 blue; 1 red, 2 green, 1 blue";

    assert_eq!(expected, sum_valid_games(input));
}
