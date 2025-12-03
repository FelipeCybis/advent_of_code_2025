use std::fs;

fn part_one_and_two(n_batteries_per_bank: usize) {
    let content_b = fs::read("input.txt").unwrap();
    let lines = content_b.split(|b| b == &b"\n"[0]);
    let mut summed_batteries: u64 = 0;
    for line in lines {
        if line.is_empty() {
            // There is one empty line at the end after splitting
            continue;
        }
        let mut battery = Vec::with_capacity(n_batteries_per_bank);
        let mut line_slice = 0..(line.len() - (n_batteries_per_bank - 1));
        for _ in 0..n_batteries_per_bank {
            for (j, first_rate) in b"987654321".iter().enumerate() {
                if let Some(jolt_pos) = line[line_slice.clone()]
                    .iter()
                    .position(|&x| &x == first_rate)
                {
                    battery.push(b"987654321"[j]);
                    line_slice = (jolt_pos + line_slice.start + 1)..(line_slice.end + 1);
                    break;
                }
            }
        }
        assert_eq!(battery.len(), n_batteries_per_bank);
        let battery_joltage = String::from_utf8(battery).unwrap().parse::<u64>().unwrap();
        summed_batteries += battery_joltage;
    }
    println!("Summed joltage: {}", summed_batteries);
}

fn main() {
    println!("Day 3 solutions:");
    println!("# First part");
    let start_time = std::time::Instant::now();
    part_one_and_two(2);
    let duration = start_time.elapsed();
    println!("First part took: {:?}", duration);

    println!("\n# Second part");
    let start_time = std::time::Instant::now();
    part_one_and_two(12);
    let duration = start_time.elapsed();
    println!("Second part took: {:?}", duration);
}
