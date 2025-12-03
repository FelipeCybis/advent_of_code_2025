use std::fs;
use std::str::FromStr;

fn part_one() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let ranges = lines.split(",");
    let mut sum_invalid_id: u64 = 0;
    for range in ranges {
        let (first_id_str, last_id_str) = range.split_once("-").unwrap();
        let first_id = usize::from_str(first_id_str).unwrap();
        let last_id = usize::from_str(last_id_str.trim_end()).unwrap();
        let first_id_count = first_id_str.chars().count();
        let last_id_count = last_id_str.trim_end().chars().count();

        let first_id_is_odd = first_id_count % 2 == 1;
        let last_id_is_odd = last_id_count % 2 == 1;

        if first_id_is_odd & last_id_is_odd {
            continue;
        }

        let (half_first_id, half_last_id) = if first_id_is_odd {
            (
                10usize.pow((last_id_count.div_euclid(2) - 1).try_into().unwrap()),
                usize::from_str(&last_id_str[..last_id_count.div_euclid(2)]).unwrap(),
            )
        } else if last_id_is_odd {
            (
                usize::from_str(&first_id_str[..first_id_count.div_euclid(2)]).unwrap(),
                10usize.pow(first_id_count.div_euclid(2).try_into().unwrap()) - 1usize,
            )
        } else {
            (
                usize::from_str(&first_id_str[..first_id_count.div_euclid(2)]).unwrap(),
                usize::from_str(&last_id_str[..last_id_count.div_euclid(2)]).unwrap(),
            )
        };

        for id_in_range in half_first_id..(half_last_id + 1) {
            let invalid_id: usize = (id_in_range.to_string() + &id_in_range.to_string())
                .parse()
                .unwrap();
            if (first_id..last_id).contains(&invalid_id) {
                sum_invalid_id += invalid_id as u64;
            }
        }
    }
    println!("Sum of invalid IDs: {}", sum_invalid_id);
}

fn part_two() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let ranges = lines.split(",");
    let mut sum_invalid_id: u64 = 0;
    for range in ranges {
        let (first_id_str, last_id_str) = range.split_once("-").unwrap();
        let first_id = usize::from_str(first_id_str).unwrap();
        let last_id = usize::from_str(last_id_str.trim_end()).unwrap();

        // This is the code for the second and final part of day 2 challenge
        for id_in_range in first_id..(last_id + 1) {
            let id_str = id_in_range.to_string();
            let id_chars = id_str.chars().collect::<Vec<char>>();
            let id_len = id_chars.len();

            for i in 2..=id_len {
                let chunk_size = id_len.div_euclid(i);
                if id_len % i != 0 {
                    continue;
                }
                let chunks = id_chars.chunks_exact(chunk_size).collect::<Vec<_>>();
                let first_chunk = chunks[0];
                if chunks.iter().all(|chunk| chunk == &first_chunk) {
                    sum_invalid_id += id_in_range as u64;
                    // println!("Invalid ID found: {:?}", chunks);
                    break;
                }
            }
        }
    }
    println!("Sum of invalid IDs: {}", sum_invalid_id);
}

fn main() {
    println!("Day 2 solutions:");
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
