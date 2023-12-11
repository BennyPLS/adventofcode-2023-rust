use std::collections::HashMap;

struct Card {
    winning: Vec<i32>,
    have: Vec<i32>,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.winning == other.winning && self.have == other.have
    }
}

impl Eq for Card {}

impl std::hash::Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.winning.hash(state);
        self.have.hash(state);
    }
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

    fn get_points(&self) -> usize {
        let mut points = 0;

        for number in &self.have {
            if self.winning.contains(number) {
                points += 1;
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
    assert_eq!(card.get_points(), 4);
}

#[test]
fn main_test() {
    let input = include_str!("./sample.txt");
    let mut cards: HashMap<usize, (Card, i32)> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        cards.insert(i, (Card::parse_card(line), 1));
    }

    let mut index = 0;
    while index < cards.len() {
        let (card, quantity) = cards.get(&index).unwrap();

        let card_quantity = quantity.clone();
        let points = card.get_points();
        let start = index + 1;

        for i in start..start + points {
            let (_, quantity) = cards.get_mut(&i).unwrap();
            *quantity += card_quantity;
        }

        index += 1;
    }

    let mut total = 0;
    for (_, quantity) in cards.values() {
        total += quantity;
    }

    assert_eq!(total, 30);
}

fn main() {
    let input = include_str!("input.txt");
    let mut cards: HashMap<usize, (Card, i32)> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        cards.insert(i, (Card::parse_card(line), 1));
    }

    let mut index = 0;
    while index < cards.len() {
        let (card, quantity) = cards.get(&index).unwrap();

        let card_quantity = quantity.clone();
        let points = card.get_points();
        let start = index + 1;

        for i in start..start + points {
            let (_, quantity) = cards.get_mut(&i).unwrap();
            *quantity += card_quantity;
        }

        index += 1;
    }

    let mut total = 0;
    for (_, quantity) in cards.values() {
        total += quantity;
    }

    println!("{}", total);
}
