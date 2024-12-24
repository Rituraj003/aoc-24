use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("input.txt")?;
    let grid: Vec<Vec<char>> = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let to_search = vec!['X', 'M', 'A', 'S'];

    let mut count_part1 = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == to_search[0] {
                count_part1 += second_level_part1(&grid, &to_search, i, j)
            }
        }
    }

    let mut count_part2 = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if i == 0 || i == grid.len() - 1 || j == 0 || j == row.len() - 1 {
                continue;
            }
            if cell == 'A' {
                count_part2 += second_level_part2(&grid, i, j);
            }
        }
    }

    println!("The number of paths is for part 1: {}", count_part1);
    println!("The number of paths is for part 2: {}", count_part2);
    Ok(())
}

fn second_level_part1(grid: &Vec<Vec<char>>, to_search: &[char], i: usize, j: usize) -> i32 {
    let mut count = 0;
    for a in -1..=1 {
        for b in -1..=1 {
            if a == 0 && b == 0 {
                continue;
            }
            
            let new_i = i as isize + a;
            let new_j = j as isize + b;
            if !valid_index(grid, new_i, new_j) {
                continue;
            }
            let (new_i, new_j) = (new_i as usize, new_j as usize);
            if grid[new_i][new_j] != to_search[1] {
                continue;
            }
            let new_i2 = new_i as isize + a;
            let new_j2 = new_j as isize + b;
            if !valid_index(grid, new_i2, new_j2) {
                continue;
            }
            let (new_i2, new_j2) = (new_i2 as usize, new_j2 as usize);
            if grid[new_i2][new_j2] != to_search[2] {
                continue;
            }
            let new_i3 = new_i2 as isize + a;
            let new_j3 = new_j2 as isize + b;
            if !valid_index(grid, new_i3, new_j3) {
                continue;
            }
            let (new_i3, new_j3) = (new_i3 as usize, new_j3 as usize);
            if grid[new_i3][new_j3] == to_search[3] {
                count += 1;
            }
        }
    }
    count
}

fn second_level_part2(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut proceed = false;
    if grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S' {
        proceed = true;
    }
    else if grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M' {
        proceed = true;
    }

    if proceed{
        if grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S' {
            return 1;
        }
        else if grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M' {
            return 1;
        }
    }
    0
}

fn valid_index(grid: &Vec<Vec<char>>, i: isize, j: isize) -> bool {
    if i < 0 || j < 0 {
        return false;
    }
    let (i, j) = (i as usize, j as usize);
    if i >= grid.len() {
        return false;
    }
    if j >= grid[i].len() {
        return false;
    }
    true
}
