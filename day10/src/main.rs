use std::collections::HashSet;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("input.txt")?;
    let trail: Vec<Vec<i32>> = input_data
        .lines()
        .map(|line| line.chars()
                        .map(|c| c.to_digit(10).unwrap() as i32)
                        .collect())
        .collect();

    let starting_points = trail.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, row_data)| {
            row_data.iter().enumerate().for_each(|(col, &c)| {
                if c == 0 {
                    acc.push((row as i32, col as i32));
                }
            });
            acc
        });
    
    // Part 1: Calculate scores (number of distinct 9s reachable)
    let mut total_score_sum = 0;
    for &starting_point in &starting_points {
        total_score_sum += count_unique_nines(&trail, starting_point);
    }
    
    // Part 2: Calculate ratings (number of distinct trails)
    let mut memo = vec![vec![None; trail[0].len()]; trail.len()];
    let mut total_rating_sum = 0;
    for &starting_point in &starting_points {
        let rating = count_trails(&trail, starting_point, &mut memo);
        total_rating_sum += rating;
    }
    println!("Part 1: {}", total_score_sum);
    println!("Part 2: {}", total_rating_sum);

    Ok(())
}

// Part 1
fn count_unique_nines(trail: &Vec<Vec<i32>>, start: (i32, i32)) -> usize {
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    let mut found_nines = HashSet::new();

    while let Some((row, col)) = stack.pop() {
        if !visited.insert((row, col)) {
            continue;
        }

        let current_height = trail[row as usize][col as usize];
        if current_height == 9 {
            found_nines.insert((row, col));
            continue; 
        }

        let directions = [
            (1i32, 0i32),  
            (-1, 0),        
            (0, 1),        
            (0, -1),        
        ];

        for &(dr, dc) in &directions {
            let new_row = row + dr;
            let new_col = col + dc;

            if new_row < 0 || new_row >= trail.len() as i32 || new_col < 0 || new_col >= trail[0].len() as i32 {
                continue;
            }

            if trail[new_row as usize][new_col as usize] == current_height + 1 {
                stack.push((new_row, new_col));
            }
        }
    }

    found_nines.len()
}

// Part 2
fn count_trails(trail: &Vec<Vec<i32>>, position: (i32, i32), memo: &mut Vec<Vec<Option<usize>>>) -> usize {
    let (row, col) = position;

    if row < 0 || row >= trail.len() as i32 || col < 0 || col >= trail[0].len() as i32 {
        return 0;
    }

    let row_usize = row as usize;
    let col_usize = col as usize;

    if let Some(count) = memo[row_usize][col_usize] {
        return count;
    }

    let current_height = trail[row_usize][col_usize];

    if current_height == 9 {
        memo[row_usize][col_usize] = Some(1);
        return 1;
    }

    let mut path_count = 0;

    let directions = [
        (1i32, 0i32),  
        (-1, 0),        
        (0, 1),         
        (0, -1),        
    ];

    for &(dr, dc) in &directions {
        let new_row = row + dr;
        let new_col = col + dc;

        if new_row < 0 || new_row >= trail.len() as i32 || new_col < 0 || new_col >= trail[0].len() as i32 {
            continue;
        }

        let new_row_usize = new_row as usize;
        let new_col_usize = new_col as usize;

        if trail[new_row_usize][new_col_usize] == current_height + 1 {
            path_count += count_trails(trail, (new_row, new_col), memo);
        }
    }
    memo[row_usize][col_usize] = Some(path_count);
    path_count
}
