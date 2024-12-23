use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(&path).expect("Could not open input.txt");
    let reader = io::BufReader::new(file);

    //--------------------------------------------------------------------------
    // Part 1: 
    //--------------------------------------------------------------------------
    let mut sum_part1: u32 = 0;
    // A simple regex for "mul(x,y)" ignoring spaces around comma
    let re_mul_p1 = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result.expect("Could not read line");

        for caps in re_mul_p1.captures_iter(&line) {
            let num1_str = &caps[1];
            let num2_str = &caps[2];

            let num1: u32 = match num1_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!(
                        "Failed to parse '{}' as a number on line {}",
                        num1_str, line_number + 1
                    );
                    continue;
                }
            };

            let num2: u32 = match num2_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!(
                        "Failed to parse '{}' as a number on line {}",
                        num2_str, line_number + 1
                    );
                    continue;
                }
            };

            sum_part1 += num1 * num2;
        }
    }

    println!("--- Part 1 Sum: {}", sum_part1);

    //--------------------------------------------------------------------------
    // Part 2
    //--------------------------------------------------------------------------
    let file2 = File::open(&path).expect("Could not open input.txt again for Part 2");
    let reader2 = io::BufReader::new(file2);

    //   don't(), do(), or mul(\d{1,3},\d{1,3})
    let re_instructions = Regex::new(
        r"(don't\(\)|do\(\)|mul\(\s*\d{1,3}\s*,\s*\d{1,3}\s*\))"
    ).expect("Invalid instruction pattern");

    let mut sum_part2: u32 = 0;
    let mut is_enabled = true;  

    for (line_number, line_result) in reader2.lines().enumerate() {
        let line = line_result.expect("Could not read line");

        for instruction_match in re_instructions.find_iter(&line) {
            let instruction = instruction_match.as_str();

            if instruction == "do()" {
                is_enabled = true;
            } else if instruction == "don't()" {
                is_enabled = false;
            } else if instruction.starts_with("mul(") {
                if is_enabled {
                    let re_mul_p2 = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();
                    if let Some(caps) = re_mul_p2.captures(instruction) {
                        let num1_str = &caps[1];
                        let num2_str = &caps[2];

                        let num1: u32 = match num1_str.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                eprintln!(
                                    "Failed to parse '{}' as a number on line {}",
                                    num1_str, line_number + 1
                                );
                                continue;
                            }
                        };

                        let num2: u32 = match num2_str.parse() {
                            Ok(n) => n,
                            Err(_) => {
                                eprintln!(
                                    "Failed to parse '{}' as a number on line {}",
                                    num2_str, line_number + 1
                                );
                                continue;
                            }
                        };

                        sum_part2 += num1 * num2;
                    }
                }
            }
        }
    }

    println!("--- Part 2 Sum: {}", sum_part2);

    Ok(())
}

