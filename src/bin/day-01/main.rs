const DIAL_MAX: u32 = 99;

#[derive(Debug)]
struct Dial {
    state: u32,
}

impl Dial {
    fn new(start: u32) -> Self {
        Self { state: start }
    }

    fn turn_right_once(&mut self) {
        if (self.state == DIAL_MAX) {
            self.state = 0;
            return;
        }
        self.state += 1;
    }

    fn turn_left_once(&mut self) {
        if (self.state == 0) {
            self.state = DIAL_MAX;
            return;
        }
        self.state -= 1;
    }

    fn turn_right(&mut self, count: u32) -> u32 {
        let mut zero_count: u32 = 0;
        for _ in 0..count {
            self.turn_right_once();
            if self.state == 0 {
                zero_count += 1;
            }
        }
        zero_count
    }

    fn turn_left(&mut self, count: u32) -> u32 {
        let mut zero_count: u32 = 0;
        for _ in 0..count {
            self.turn_left_once();
            if self.state == 0 {
                zero_count += 1;
            }
        }
        zero_count
    }
}

fn main() {
    let mut dial = Dial::new(50);
    let input = include_str!("input.txt");
    let mut end_on_zero_count = 0;
    let mut go_trough_zero_count = 0;
    for line in input.lines() {
        let dir = &line[0..1];
        let count: u32 = line[1..].parse().unwrap();
        match dir {
            "L" => {
                go_trough_zero_count += dial.turn_left(count);
            }
            "R" => {
                go_trough_zero_count += dial.turn_right(count);
            }
            &_ => todo!(),
        }
        println!("{}", dial.state);
        if dial.state == 0 {
            end_on_zero_count += 1;
        }
    }
    println!("end on zero count: {end_on_zero_count}");
    println!("go trough zero count: {go_trough_zero_count}");
}
