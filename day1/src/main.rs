use std::fs;

fn part_one() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let mut cumsum_position: Vec<i32> = Vec::with_capacity(lines.lines().count() + 1);
    cumsum_position.push(50);

    for (i, line) in lines.lines().enumerate() {
        let sign: i32 = if line.starts_with('L') { -1 } else { 1 };
        let shift: i32 = line[1..].parse().unwrap();
        cumsum_position.push(cumsum_position[i] + (sign * shift));
    }

    let rem_of_zero_pos = cumsum_position
        .iter()
        .filter(|x| x.rem_euclid(100) == 0)
        .count();

    println!("Stops at zero: {}", rem_of_zero_pos);
}

fn part_two() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let mut full_shifts = 0;
    let mut last_position = 50;
    let mut at_zero_pos = 0;
    let mut through_zero = 0;
    for line in lines.lines() {
        let sign: i32 = if line.starts_with('L') { -1 } else { 1 };
        let shift: i32 = line[1..].parse().unwrap();
        let remainder = shift.rem_euclid(100);

        full_shifts += shift.div_euclid(100);

        let possible_new_position = last_position + (sign * remainder);
        let new_position = possible_new_position.rem_euclid(100);

        if last_position != 0 {
            if new_position == 0 {
                at_zero_pos += 1
            } else if new_position != possible_new_position {
                through_zero += 1
            }
        }
        last_position = new_position;
    }

    // println!("Full shifts: {}", full_shifts);
    // println!("At zero after step: {}", at_zero_pos);
    // println!("At zero during step: {}", through_zero);
    println!(
        "Overall zeros: {}",
        full_shifts + at_zero_pos + through_zero
    );
}

fn main() {
    println!("Day 1 solutions:");
    println!("# First part");
    let start_time = std::time::Instant::now();
    part_one();
    let duration = start_time.elapsed();
    println!("First part took: {:?}", duration);

    println!("\n# Second part");
    let start_time = std::time::Instant::now();
    part_two();
    let duration = start_time.elapsed();
    println!("Second part took: {:?}", duration);
}
