trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        self.is_ascii_punctuation() && self != &'.'
    }
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    grid
}

struct PartNumber {
    number: i32,
    length: usize,
    x: usize,
    y: usize,
}

impl PartNumber {
    fn is_valid(&self, grid: &Vec<Vec<char>>) -> bool {
        let y_start = if self.y == 0 { 0 } else { self.y - 1 };
        let y_end = if self.y == grid.len() - 1 {
            grid.len() - 1
        } else {
            self.y + 1
        };

        let range = y_start..=y_end;

        for i in range {
            if self.scan_row(i, grid) {
                return true;
            }
        }

        false
    }

    fn scan_row(&self, y: usize, grid: &Vec<Vec<char>>) -> bool {
        let x_start = if self.x == 0 { 0 } else { self.x - 1 };
        let x_end = if self.x + self.length >= grid[y].len() - 1 {
            grid[y].len() - 1
        } else {
            self.x + self.length
        };
        let range = x_start..=x_end;

        for i in range {
            if grid[y][i].is_symbol() {
                return true;
            }
        }

        false
    }
}

#[test]
fn part_number_is_valid_test() {
    let grid = create_grid(include_str!("input.txt"));

    let part_number_true = PartNumber {
        number: 426,
        length: 3,
        x: 9,
        y: 0,
    };

    let part_number_false = PartNumber {
        number: 687,
        length: 3,
        x: 6,
        y: 12,
    };

    assert_eq!(part_number_true.is_valid(&grid), true);
    assert_eq!(part_number_false.is_valid(&grid), false);
}
fn scan_grid(grid: &Vec<Vec<char>>) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            let mut part_number = String::new();
            for i in x..row.len() {
                if !row[i].is_digit(10) {
                    break;
                }

                part_number.push(row[i]);
            }

            if part_number.len() > 0 {
                let part_number = PartNumber {
                    number: part_number.parse::<i32>().unwrap(),
                    length: part_number.len(),
                    x,
                    y,
                };

                x += part_number.length;
                part_numbers.push(part_number);
            } else {
                x += 1;
            }
        }
    }

    part_numbers
}

#[test]
fn scan_grid_test() {
    let grid = create_grid(include_str!("sample.txt"));
    let part_numbers = scan_grid(&grid);

    assert_eq!(part_numbers.len(), 10);
}

fn sum_valid_part_numbers(part_numbers: &Vec<PartNumber>, grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    for part_number in part_numbers {
        if part_number.is_valid(&grid) {
            sum += part_number.number;
        }
    }

    sum
}

#[test]
fn sum_valid_part_numbers_test() {
    let grid = create_grid(include_str!("sample.txt"));
    let part_numbers = scan_grid(&grid);
    let sum = sum_valid_part_numbers(&part_numbers, &grid);

    assert_eq!(sum, 4361);
}

fn main() {
    let grid = create_grid(include_str!("input.txt"));
    let part_numbers = scan_grid(&grid);
    let sum = sum_valid_part_numbers(&part_numbers, &grid);

    println!("Sum: {}", sum);
}
