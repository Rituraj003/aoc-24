use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    let mut total_safe: usize = 0;
    let mut total_safe_with_one_invalid: usize = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line_content) => {
                let tokens: Vec<&str> = line_content.split_whitespace().collect();
                let numbers: Vec<i32> = tokens.iter().map(|&s| s.parse::<i32>().unwrap()).collect();
                // Check if tokens are sorted in ascending or descending order
                if numbers.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] < 4)
                    || numbers.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] < 4)
                {
                    total_safe += 1;
                }

                if numbers
                    .windows(2)
                    .filter(|w| w[0] < w[1] && w[1] - w[0] < 4)
                    .count()
                    >= numbers.len() - 2
                    || numbers
                        .windows(2)
                        .filter(|w| w[0] > w[1] && w[0] - w[1] < 4)
                        .count()
                        >= numbers.len() - 2
                {
                    total_safe_with_one_invalid += 1;
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("Total safe: {}", total_safe);
    // Incorrect output : Correct output is 465
    println!(
        "Total safe with one invalid: {}",
        total_safe_with_one_invalid
    );
    Ok(())
}
