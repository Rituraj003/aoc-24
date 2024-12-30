use std::error::Error;
use std::fs;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("input.txt")?;
    let input_data = binding.lines().collect::<Vec<_>>();
    let mut total_sum_part1: i64 = 0;
    let mut total_sum_part2: i64 = 0;
    for line in input_data {
        let split_line = line.split(":").collect::<Vec<_>>();
        let rule = split_line[0].parse::<i64>().unwrap();
        let number = split_line[1]
            .trim()
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        // Part 1
        let mut set = HashSet::new();
        set.insert(number[0] as i64);
        for i in 1..number.len() {
            let mut temp_set = HashSet::new();
            for j in set.clone() {
                temp_set.insert(j + number[i]);
                temp_set.insert(j * number[i]);
            }
            set = temp_set;
        }
        if set.contains(&(rule as i64)) {
            total_sum_part1 += rule;
        }

        // Part 2
        set = HashSet::new();
        set.insert(number[0] as i64);
        for i in 1..number.len() {
            let mut temp_set = HashSet::new();
            for j in set.clone() {
                temp_set.insert(j + number[i]);
                temp_set.insert(j * number[i]);
                temp_set.insert((j.to_string() + &number[i].to_string()).parse::<i64>().unwrap());
            }
            set = temp_set;
        }
        if set.contains(&(rule as i64)) {
            total_sum_part2 += rule;
        }
    }


    println!("Total sum part 1: {}", total_sum_part1);
    println!("Total sum part 2: {}", total_sum_part2);
    Ok(())
}
