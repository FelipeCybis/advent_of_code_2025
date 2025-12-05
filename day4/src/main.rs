use std::fs;
fn part_one() {
    let mut content_b = fs::read("input.txt").unwrap();
    let cols = content_b.iter().position(|&x| x == b"\n"[0]).unwrap() + 1;
    let rows = content_b.len() / cols;
    content_b.insert(0, 0);
    for i in 1..rows {
        content_b.insert((i * cols) - 1 + i, 0);
    }

    let first_line_padding = vec![0; cols + 1];
    let diagram = [first_line_padding.clone(), content_b, first_line_padding].concat();
    let mut accessible_rolls: u32 = 0;
    for (i, roll) in diagram.iter().enumerate() {
        if roll != &b"@"[0] {
            continue;
        } else {
            let neighbors = [
                i - (cols + 1) - 1,
                i - (cols + 1),
                i - (cols + 1) + 1,
                i - 1,
                i + 1,
                i + (cols + 1) - 1,
                i + (cols + 1),
                i + (cols + 1) + 1,
            ];
            let mut n_neighbors: u8 = 0;
            for neighbor in neighbors {
                if diagram[neighbor] == b"@"[0] {
                    n_neighbors += 1;
                    if n_neighbors == 4 {
                        break;
                    }
                }
            }

            if n_neighbors < 4 {
                accessible_rolls += 1;
            }
        }
    }

    println!("Accessible rolls: {}", accessible_rolls);
}

fn part_two() {
    let mut content_b = fs::read("input.txt").unwrap();
    let cols = content_b.iter().position(|&x| x == b"\n"[0]).unwrap() + 1;
    let rows = content_b.len() / cols;
    content_b.insert(0, 0);
    for i in 1..rows {
        content_b.insert((i * cols) - 1 + i, 0);
    }

    let first_line_padding = vec![0; cols + 1];
    let mut diagram = [first_line_padding.clone(), content_b, first_line_padding].concat();
    let mut accessible_rolls: u32 = 0;
    let mut last_accessible_rols: u32 = 0;
    let mut moved_all_rolls = false;

    while !moved_all_rolls {
        for i in 0..diagram.len() {
            if diagram[i] != b"@"[0] {
                continue;
            } else {
                let neighbors = [
                    i - (cols + 1) - 1,
                    i - (cols + 1),
                    i - (cols + 1) + 1,
                    i - 1,
                    i + 1,
                    i + (cols + 1) - 1,
                    i + (cols + 1),
                    i + (cols + 1) + 1,
                ];
                let mut n_neighbors: u8 = 0;
                for neighbor in neighbors {
                    if diagram[neighbor] == b"@"[0] {
                        n_neighbors += 1;
                        if n_neighbors == 4 {
                            break;
                        }
                    }
                }

                if n_neighbors < 4 {
                    accessible_rolls += 1;
                    diagram[i] = 0;
                }
            }
        }
        moved_all_rolls = last_accessible_rols == accessible_rolls;
        last_accessible_rols = accessible_rolls;
    }

    println!("Accessible rolls: {}", accessible_rolls);
}

fn main() {
    println!("Day 4 solutions:");
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
