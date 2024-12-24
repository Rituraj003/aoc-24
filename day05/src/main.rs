// adopted from https://github.com/noahpro99/advent-of-code/blob/main/2024/day5/day5.ipynb
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("input.txt")?;

    let mut sections = input_data.split("\n\n");
    let rules_section = sections.next().ok_or("No rules section found")?;
    let orders_section = sections.next().ok_or("No orders section found")?;

    let rules = rules_section.lines().collect::<Vec<&str>>();

    let orders = orders_section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>();

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        let parts: Vec<&str> = rule.split('|').collect();
        if parts.len() != 2 {
            eprintln!("Invalid rule format: {}", rule);
            continue; // Skip invalid rules
        }
        let x = match parts[0].trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number in rule: {}", parts[0]);
                continue; // Skip invalid numbers
            }
        };
        let y = match parts[1].trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number in rule: {}", parts[1]);
                continue; // Skip invalid numbers
            }
        };

        rules_map.entry(y).or_insert_with(Vec::new).push(x);
    }

    // Initialize sum counters
    let mut part1 = 0;
    let mut part2 = 0;
    for order_str in orders {
        // Parse the order into a vector of i32
        let order_result = order_str
            .split(',')
            .map(|x| x.trim().parse::<i32>())
            .collect::<Result<Vec<i32>, _>>();

        let order = match order_result {
            Ok(vec) => vec,
            Err(_) => {
                eprintln!("Invalid number in order: {}", order_str);
                continue; // Skip invalid orders
            }
        };

        // Check if the order is valid
        if valid_order(&order, &rules_map) {
            // If valid, add the middle page to part1
            if !order.is_empty() {
                part1 += order[order.len() / 2];
            }
        }
        else {
            // If invalid, reorder the order topologically
            if let Some(reordered) = reorder_topological(&order, &rules_map) {
                if valid_order(&reordered, &rules_map) {
                    if !reordered.is_empty() {
                        part2 += reordered[reordered.len() / 2];
                    }
                }
            }
        }

    }

    println!("Part1 : {}", part1);
    println!("Part2 : {}", part2);

    Ok(())
}

fn valid_order(order: &Vec<i32>, rules_map: &HashMap<i32, Vec<i32>>) -> bool {
    let mut cant_print: HashSet<i32> = HashSet::new();
    for &item in order {
        if cant_print.contains(&item) {
            return false;
        }
        if let Some(dependencies) = rules_map.get(&item) {
            for &dep in dependencies {
                cant_print.insert(dep);
            }
        }
    }
    true
}

fn reorder_topological(order: &Vec<i32>, rules_map: &HashMap<i32, Vec<i32>>) -> Option<Vec<i32>> {
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

    for &node in order {
        in_degree.entry(node).or_insert(0);
        adj.entry(node).or_insert_with(Vec::new);
    }

    for &y in order {
        if let Some(deps) = rules_map.get(&y) {
            for &x in deps {
                if order.contains(&x) {
                    adj.get_mut(&x).unwrap().push(y);
                    *in_degree.get_mut(&y).unwrap() += 1;
                }
            }
        }
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    for (&node, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(node);
        }
    }

    let mut sorted: Vec<i32> = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = adj.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    if sorted.len() == order.len() {
        Some(sorted)
    } else {
        None
    }
}
