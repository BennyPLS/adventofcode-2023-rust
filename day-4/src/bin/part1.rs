struct Card {
    winning: Vec<i32>,
    have: Vec<i32>,
}

impl Card {
    fn parse_card(input: &str) -> Self {
        let numbers: Vec<Vec<i32>> = input
            .trim()
            .split(":")
            .nth(1)
            .expect("No numbers found")
            .split("|")
            .map(|x| x.trim().split(' '))
            .map(|x| {
                x.filter(|x| !x.is_empty())
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        Card {
            winning: numbers[0].clone(),
            have: numbers[1].clone(),
        }
    }

    fn get_points(&self) -> i32 {
        let mut points = 0;

        for number in &self.have {
            if self.winning.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        points
    }
}

#[test]
fn parse_card_test() {
    let str_card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let card = Card::parse_card(str_card);
    assert_eq!(card.winning, vec![41, 48, 83, 86, 17]);
    assert_eq!(card.have, vec![83, 86, 6, 31, 17, 9, 48, 53]);
}

#[test]
fn get_points_test() {
    let str_card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let card = Card::parse_card(str_card);
    assert_eq!(card.get_points(), 8);
}

#[test]
fn main_test() {
    let input = include_str!("./sample.txt");
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        cards.push(Card::parse_card(line));
    }

    let mut points = 0;

    for card in cards {
        points += card.get_points();
    }

    assert_eq!(points, 13);
}

fn main() {
    let input = include_str!("input.txt");
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        cards.push(Card::parse_card(line));
    }

    let mut points = 0;

    for card in cards {
        points += card.get_points();
    }

    println!("Points: {}", points);
}
