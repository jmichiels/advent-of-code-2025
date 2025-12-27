fn is_valid_id(id: &str) -> bool {
    let id_length = id.len();
    let half_id_length = id_length / 2;
    for pattern_length in 1..=half_id_length {
        if id_length % pattern_length != 0 {
            continue;
        }
        let pattern = &id[0..pattern_length];
        let repeat_count = id_length / pattern_length;
        if pattern.repeat(repeat_count) == id {
            return false;
        }
    }
    return true;
}

fn main() {
    let input = include_str!("input.txt");
    let ranges = input.split(",");
    let mut total: u64 = 0;
    for range in ranges {
        println!("Range: {range}");
        let parts: Vec<u64> = range.split("-").map(|x| x.parse().unwrap()).collect();
        let min = parts[0];
        let max = parts[1];
        for id in min..=max {
            let id_str = id.to_string();
            // PART 1
            // if id_str.len() % 2 != 0 {
            //     // Odd ID length, cannot be repeated.
            //     continue;
            // }
            // let middle = id_str.len() / 2;
            // if id_str[0..middle] == id_str[middle..] {
            //     println!("\tInvalid ID: {id_str}");
            //     total += id;
            // }
            // PART 2
            if !is_valid_id(&id_str) {
                println!("\tInvalid ID: {id_str}");
                total += id;
            }
        }
    }
    println!("Total: {total}");
}
