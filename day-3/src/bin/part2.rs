trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        self.is_ascii_punctuation() && self != &'.'
    }
}

type Grid = Vec<Vec<char>>;

fn create_grid(input: &str) -> Grid {
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    grid
}

#[derive(Debug)]
struct GearRatio {
    numbers: Option<(i32, i32)>,
    x: usize,
    y: usize,
}

impl GearRatio {
    fn scan(&mut self, grid: &Grid) -> bool {
        let y_start = if self.y == 0 { 0 } else { self.y - 1 };
        let y_end = {
            if self.y == grid.len() - 1 {
                grid.len() - 1
            } else {
                self.y + 1
            }
        };

        let x_start = if self.x == 0 { 0 } else { self.x - 1 };
        let x_end = {
            if self.x == grid[0].len() - 1 {
                grid[0].len() - 1
            } else {
                self.x + 1
            }
        };

        let range = y_start..=y_end;
        let mut numbers: Vec<i32> = Vec::new();

        for y in range {
            let row = &grid[y];
            let mut x = x_start;
            while x <= x_end {
                if row[x].is_digit(10) {
                    if let Some((number, last_index)) = get_number(row, x) {
                        numbers.push(number);
                        x = last_index;
                    }
                }

                x += 1;
            }
        }

        if numbers.len() != 2 {
            return false;
        }

        self.numbers = Some((numbers[0], numbers[1]));
        return true;
    }

    fn get_ratio(&self) -> Option<i32> {
        if let Some((a, b)) = self.numbers {
            Some(a * b)
        } else {
            None
        }
    }
}

#[test]
fn scan_test() {
    let mut gear_ratio = GearRatio {
        numbers: None,
        x: 3,
        y: 1,
    };

    let grid = create_grid(include_str!("./sample.txt"));

    assert_eq!(gear_ratio.scan(&grid), true);

    if let Some((a, b)) = gear_ratio.numbers {
        assert_eq!(a, 467);
        assert_eq!(b, 35);
    }

    gear_ratio = GearRatio {
        numbers: None,
        x: 5,
        y: 8,
    };

    assert_eq!(gear_ratio.scan(&grid), true);
    if let Some((a, b)) = gear_ratio.numbers {
        assert_eq!(a, 755);
        assert_eq!(b, 598);
    }
}

fn get_number(row: &Vec<char>, x: usize) -> Option<(i32, usize)> {
    let mut part_number = String::new();
    let mut last_digit_index = 0;
    let mut x = x;

    if !row[x].is_digit(10) {
        return None;
    }

    while x > 0 && row.get(x - 1).is_some() && row[x - 1].is_digit(10) {
        x -= 1;
    }

    for i in x..row.len() {
        if !row[i].is_digit(10) {
            break;
        }

        part_number.push(row[i]);

        last_digit_index = i;
    }

    Some((part_number.parse::<i32>().unwrap(), last_digit_index))
}

#[test]
fn get_number_test() {
    let row = vec!['1', 'a', '3', '4', 'a', '6', '7', 'a', '9'];
    assert_eq!(get_number(&row, 0), Some((1, 0)));
    assert_eq!(get_number(&row, 2), Some((34, 3)));
    assert_eq!(get_number(&row, 3), Some((34, 3)));
    assert_eq!(get_number(&row, 5), Some((67, 6)));
    assert_eq!(get_number(&row, 7), None);
    assert_eq!(get_number(&row, 8), Some((9, 8)));
}

fn get_gears(grid: &Grid) -> Vec<GearRatio> {
    let mut gears: Vec<GearRatio> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char.is_symbol() {
                let mut gear = GearRatio {
                    numbers: None,
                    x,
                    y,
                };

                if gear.scan(&grid) {
                    gears.push(gear);
                }
            }
        }
    }

    gears
}

fn sum_gear_ratios(gears: &Vec<GearRatio>) -> i32 {
    let mut sum = 0;

    for gear in gears {
        if let Some(ratio) = gear.get_ratio() {
            sum += ratio;
        }
    }

    sum
}

#[test]
fn sum_gear_ratios_test() {
    let grid = create_grid(include_str!("./sample.txt"));
    let gears = get_gears(&grid);
    let sum = sum_gear_ratios(&gears);

    assert_eq!(sum, 467835);
}

fn main() {
    let grid = create_grid(include_str!("./input2.txt"));
    let gears = get_gears(&grid);
    let sum = sum_gear_ratios(&gears);

    println!("Sum of gear ratios: {}", sum);
}
