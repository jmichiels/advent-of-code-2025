struct Map {
    grid: Vec<Vec<bool>>,
}

impl Map {
    fn parse(input: &str) -> Map {
        Self {
            grid: input
                .lines()
                .map(|line| line.chars().map(|c| c == '@').collect())
                .collect(),
        }
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn roll_count(&self) -> usize {
        let mut roll_count = 0;
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.contains_roll(row as isize, col as isize) {
                    roll_count += 1;
                }
            }
        }
        roll_count
    }

    fn contains_roll(&self, row: isize, col: isize) -> bool {
        if row < 0 || col < 0 || row >= self.height() as isize || col >= self.width() as isize {
            return false;
        }
        self.grid[row as usize][col as usize]
    }

    fn is_accessible(&self, row: usize, col: usize) -> bool {
        let mut roll_count = 0;
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if !(row_offset == 0 && col_offset == 0) {
                    if self.contains_roll(row as isize + row_offset, col as isize + col_offset) {
                        roll_count += 1;
                    }
                }
            }
        }
        roll_count < 4
    }

    fn remove_accessible_rolls(&self) -> Self {
        Map {
            grid: self
                .grid
                .iter()
                .enumerate()
                .map(|(row, row_vec)| {
                    row_vec
                        .iter()
                        .enumerate()
                        .map(|(col, _)| {
                            self.contains_roll(row as isize, col as isize)
                                && !self.is_accessible(row, col)
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut map = Map::parse(input);
    let mut accessible_count = 0;
    for row in 0..map.height() {
        for col in 0..map.width() {
            if map.contains_roll(row as isize, col as isize) {
                if map.is_accessible(row, col) {
                    accessible_count += 1;
                }
            }
        }
    }
    println!("Accessible: {}", accessible_count);
    let original_count = map.roll_count();
    let mut prev_count = original_count;
    loop {
        map = map.remove_accessible_rolls();
        let new_count = map.roll_count();
        println!("{} -> {} = {}", prev_count, new_count, prev_count - new_count);
        if (new_count == prev_count) {
            println!("Removed total: {}", original_count - new_count);
            break;
        }
        prev_count = new_count;
    }
}
