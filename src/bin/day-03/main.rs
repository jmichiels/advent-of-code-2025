fn get_max_joltage(battery_joltages: &[u64]) -> Option<(usize, u64)> {
    battery_joltages
        .iter()
        .rev()
        .enumerate()
        .max_by_key(|&(_, value)| value)
        .map(|(index, value)| (battery_joltages.len() - index - 1, value.clone()))
}

fn get_bank_output_joltage(battery_joltages: &[u64], battery_count: usize) -> u64 {
    if battery_count == 0 {
        return 0;
    }
    let (max_index, max_value) =
        get_max_joltage(&battery_joltages[..battery_joltages.len() - (battery_count - 1)]).unwrap();
    if battery_count == 1 {
        return max_value;
    }
    return max_value * 10_u64.pow((battery_count - 1) as u32)
        + get_bank_output_joltage(&battery_joltages[max_index + 1..], battery_count - 1);
}

fn main() {
    let input = include_str!("input.txt");
    let mut output_joltage: u64 = 0;
    for line in input.lines() {
        let bank_joltages = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();
        println!("{}", line);
        // PART 1.
        // let (index_1, value_1) =
        //     get_max_joltage(&bank_joltages[..bank_joltages.len() - 1]).unwrap();
        // let (_, value_2) = get_max_joltage(&bank_joltages[index_1 + 1..]).unwrap();
        // let bank_voltage = 10 * value_1 + value_2;
        //let bank_output_joltage = get_bank_output_joltage(bank_joltages.as_slice(), 2);
        // PART 2.
        let bank_output_joltage = get_bank_output_joltage(bank_joltages.as_slice(), 12);
        output_joltage += bank_output_joltage;
    }
    println!("{}", output_joltage);
}
