use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("input.txt")?;
    let mut layout: Vec<Vec<char>> = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut current_location = layout
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|j| (i as isize, j as isize))
        })
        .expect("No starting '^' found in the input");

    let initial_location = current_location;

    let mut total_steps = 0;
    loop {
        let (i, j) = current_location;
        let current_char = layout[i as usize][j as usize];
        let new_location = match current_char {
            '^' => (i - 1, j),
            'v' => (i + 1, j),
            '<' => (i, j - 1),
            '>' => (i, j + 1),
            _ => break,
        };


        // Check boundaries
        if new_location.0 < 0
            || new_location.0 >= layout.len() as isize
            || new_location.1 < 0
            || new_location.1 >= layout[0].len() as isize
        {
            total_steps += 1;
            layout[i as usize][j as usize] = 'X';
            break;
        }

        let new_i = new_location.0 as usize;
        let new_j = new_location.1 as usize;

        if layout[new_i][new_j] == '#' {
            layout[i as usize][j as usize] = match current_char {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => current_char,
            };
            continue;
        }

        if layout[new_i][new_j] == '.' {
            total_steps += 1;
        }

        layout[new_i][new_j] = current_char;
        layout[i as usize][j as usize] = 'X';
        current_location = new_location;
    }

    // Part 2
    layout[initial_location.0 as usize][initial_location.1 as usize] = '^';

    // get all X locations
    let all_x: Vec<(isize, isize)> = layout
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &c)| {
                if c == 'X' {
                    Some((i as isize, j as isize))
                } else {
                    None
                }
            })
        })
        .collect();
    
    let mut total_loops = 0;
    for &(x_i, x_j) in &all_x {
        
        let mut visited: HashSet<(usize, usize, char)> = HashSet::new();
        current_location = initial_location;
        layout = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();
        layout[x_i as usize][x_j as usize] = '#';

        loop {
            let (i, j) = current_location;
            let current_char = layout[i as usize][j as usize];
            let new_location: (isize, isize) = match current_char {
                '^' => (i - 1, j),
                'v' => (i + 1, j),
                '<' => (i, j - 1),
                '>' => (i, j + 1),
                _ => break,
            };



            // Check boundaries
            if new_location.0 < 0
                || new_location.0 >= layout.len() as isize
                || new_location.1 < 0
                || new_location.1 >= layout[0].len() as isize
            {
                layout[i as usize][j as usize] = 'X';
                break;
            }

            let new_i = new_location.0 as usize;
            let new_j = new_location.1 as usize;

            if layout[new_i][new_j] == '#' {
                layout[i as usize][j as usize] = match current_char {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => current_char,
                };
                continue;
            }

            if layout[new_i][new_j] == 'X' {
                if visited.contains(&(i as usize, j as usize, current_char)) {
                    total_loops += 1;
                    break;
                } else {
                    visited.insert((i as usize, j as usize, current_char));
                }
            }

            layout[new_i][new_j] = current_char;
            layout[i as usize][j as usize] = 'X';
            current_location = new_location;
        }
    }

    println!("Part 1: {}", total_steps);
    print!("Part 2: {}", total_loops);
    Ok(())
}
