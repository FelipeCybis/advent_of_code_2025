use std::{cmp::max, fs};
fn part_one() {
    let content_b = fs::read("input.txt").unwrap();
    let ranges: Vec<_> = content_b
        .split(|b| b == &b"\n"[0])
        .filter(|&x| x.contains(&b"-"[0]))
        .map(|x| {
            let range = x
                .split(|&b| b == b"-"[0])
                .map(|s| String::from_utf8(s.into()).unwrap().parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            range[0]..=range[1]
        })
        .collect();

    let ids: Vec<u64> = content_b
        .split(|b| b == &b"\n"[0])
        .filter(|&x| !x.contains(&b"-"[0]) & !x.is_empty())
        .map(|x| {
            String::from_utf8(x.into())
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap()
        })
        .collect();

    let mut valid_ingredients = 0;
    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                valid_ingredients += 1;
                break;
            }
        }
    }
    println!("Total valid ingredients: {}", valid_ingredients);
}

fn part_two() {
    let content_b = fs::read("input.txt").unwrap();
    let mut ranges: Vec<_> = content_b
        .split(|b| b == &b"\n"[0])
        .filter(|&x| x.contains(&b"-"[0]))
        .map(|x| {
            let range = x
                .split(|&b| b == b"-"[0])
                .map(|s| String::from_utf8(s.into()).unwrap().parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            range[0]..=range[1]
        })
        .collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut combined_ranges: Vec<std::ops::RangeInclusive<u64>> = Vec::new();
    combined_ranges.push(ranges[0].clone());
    for range in ranges[1..].iter() {
        let last_range = combined_ranges.last_mut().unwrap();
        if range.start() <= last_range.end() {
            let new_end = *max(range.end(), last_range.end());
            *last_range = *last_range.start()..=new_end;
        } else {
            combined_ranges.push(range.clone());
        }
    }

    let n_ingredients = combined_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>();

    println!("Total valid ingredients: {:?}", n_ingredients);
}

fn main() {
    println!("Day 5 solutions:");
    println!("# First part");
    let start_time = std::time::Instant::now();
    part_one();
    let duration = start_time.elapsed();
    println!("First part took: {:?}", duration);

    println!("# Second part");
    let start_time = std::time::Instant::now();
    part_two();
    let duration = start_time.elapsed();
    println!("Second part took: {:?}", duration);
}
