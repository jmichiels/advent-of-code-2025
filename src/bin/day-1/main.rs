const DIAL_COUNT: u32 = 100;

#[derive(Debug)]
struct Dial {
    state: u32,
}

impl Dial {
    fn new(start: u32) -> Self {
        Self { state: start }
    }

    fn turn_right(&mut self, count: u32) {
        self.state = (self.state + count) % DIAL_COUNT;
    }

    fn turn_left(&mut self, count: u32) {
        self.turn_right(DIAL_COUNT - (count % DIAL_COUNT));
    }
}

fn main() {
    let mut dial = Dial::new(50);
    let input = include_str!("input.txt");
    let mut zero_count = 0;
    for line in input.lines() {
        let dir = &line[0..1];
        let count: u32 = line[1..].parse().unwrap();
        match dir {
            "L" => {
                dial.turn_left(count);
            }
            "R" => {
                dial.turn_right(count);
            }
            &_ => todo!(),
        }
        println!("{}", dial.state);
        if dial.state == 0 {
            zero_count += 1;
        }
    }
    println!("zero count: {zero_count}");
}
