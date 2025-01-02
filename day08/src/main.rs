use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("input.txt")?;
    let grid: Vec<Vec<char>> = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let frequencies: HashMap<char, Vec<(usize, usize)>> =
        grid.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, row)| {
                row.iter().enumerate().for_each(|(x, &c)| {
                    if c != '.' {
                        acc.entry(c).or_insert(Vec::new()).push((x, y));
                    }
                });
                acc
            });

    // Part 1
    let mut antinode_positions_part1: HashSet<(usize, usize)> = HashSet::new();

    frequencies.iter().for_each(|(_, nodes)| {
        for i in 0..nodes.len() {
            for j in i+1..nodes.len() {
                let (x1, y1) = nodes[i];
                let (x2, y2) = nodes[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;
    
                // Calculate antinode 1 (extend from first antenna)
                let antinode1_x = x1 as i32 -  dx ;
                let antinode1_y = y1 as i32 - dy ;
    
                // Calculate antinode 2 (extend from second antenna)
                let antinode2_x = x2 as i32 + dx ;
                let antinode2_y = y2 as i32 + dy ;
    
                if antinode1_x >= 0 && antinode1_x < grid[0].len() as i32 &&
                   antinode1_y >= 0 && antinode1_y < grid.len() as i32 {
                       antinode_positions_part1.insert((antinode1_x as usize, antinode1_y as usize));
                }
    
                if antinode2_x >= 0 && antinode2_x < grid[0].len() as i32 &&
                   antinode2_y >= 0 && antinode2_y < grid.len() as i32 {
                       antinode_positions_part1.insert((antinode2_x as usize, antinode2_y as usize));
                }
            }
        }
    });

    // Part 2
    let mut antinode_positions_part2: HashSet<(usize, usize)> = HashSet::new();
    frequencies.iter().for_each(|(_, nodes)| {
        for i in 0..nodes.len() {
            for j in i+1..nodes.len() {
                let (x1, y1) = nodes[i];
                let (x2, y2) = nodes[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                let mut antinode1_x = x1 as i32;
                let mut antinode1_y = y1 as i32;
                let mut antinode2_x = x2 as i32;
                let mut antinode2_y = y2 as i32;
    
                // Calculate antinode 1 (extend from first antenna)
                antinode_positions_part2.insert((antinode1_x as usize, antinode1_y as usize));
                antinode_positions_part2.insert((antinode2_x as usize, antinode2_y as usize));
                loop {
                    
                    antinode1_x = antinode1_x -  dx ;
                    antinode1_y = antinode1_y  - dy ;

                    if antinode1_x < 0 || antinode1_x >= grid[0].len() as i32 ||
                       antinode1_y < 0 || antinode1_y >= grid.len() as i32 {
                           break;
                    }
                    antinode_positions_part2.insert((antinode1_x as usize, antinode1_y as usize));
                    
                }

                loop {
                    antinode2_x = antinode2_x + dx ;
                    antinode2_y = antinode2_y + dy ;

                    if antinode2_x < 0 || antinode2_x >= grid[0].len() as i32 ||
                       antinode2_y < 0 || antinode2_y >= grid.len() as i32 {
                           break;
                    }
                    antinode_positions_part2.insert((antinode2_x as usize, antinode2_y as usize));
                }
            }
        }
    });

    print!("Part 1: {}", antinode_positions_part1.len());
    print!("Part 2: {}", antinode_positions_part2.len());

    Ok(())
}
