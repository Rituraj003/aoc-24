use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("input.txt")?;
    let initial_stones = input_data
        .lines()
        .map(|line| line.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    // Initialize a HashMap to store stone counts
    let mut stone_counts: HashMap<u128, u128> = HashMap::new();
    for stone in initial_stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    // Define a function to perform a single blink
    fn perform_blink(counts: &HashMap<u128, u128>) -> HashMap<u128, u128> {
        let mut new_counts: HashMap<u128, u128> = HashMap::new();
        for (&stone, &count) in counts.iter() {
            if stone == 0 {
                *new_counts.entry(1).or_insert(0) += count;
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let half = stone_str.len() / 2;
                    let left_split = stone_str[..half].parse::<u128>().unwrap();
                    let right_split = stone_str[half..].parse::<u128>().unwrap();
                    *new_counts.entry(left_split).or_insert(0) += count;
                    *new_counts.entry(right_split).or_insert(0) += count;
                } else {
                    let multiplied = stone * 2024;
                    *new_counts.entry(multiplied).or_insert(0) += count;
                }
            }
        }
        new_counts
    }

    for _ in 0..25 {
        stone_counts = perform_blink(&stone_counts);
    }

    let part1 = stone_counts.values().sum::<u128>();
    println!("Part 1: {}", part1);

    // Perform 50 more blinks for Part 2
    for _ in 0..50 {
        stone_counts = perform_blink(&stone_counts);
    }

    let part2 = stone_counts.values().sum::<u128>();
    println!("Part 2: {}", part2);

    Ok(())
}
