use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = "input.txt";
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut row1: Vec<i32> = Vec::new();
    let mut row2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_content = line.unwrap();
        // let line: Vec<&str> = line_content.split("  ").collect::<Vec<&str>>();
        let tokens: Vec<&str> = line_content.split_whitespace().collect();
        row1.push(tokens[0].parse::<i32>().unwrap());
        row2.push(tokens[1].parse::<i32>().unwrap());
    }

    row1.sort();
    row2.sort();

    // --------------------------------------------------------------------------
    // Part 1
    // --------------------------------------------------------------------------
    let sum: i32 = row1.iter().zip(row2.iter()).map(|(a, b)| a - b).sum();

    // --------------------------------------------------------------------------
    // Part 2
    // --------------------------------------------------------------------------
    // Using two pointers to find the similarity score
    let mut similarity_score = 0;
    let mut i = 0;
    let mut j = 0;
    let mut temp = 0;
    while (i < row1.len()) && (j < row2.len()) {
        if i > 0 && row1[i] == row1[i - 1] {
            i += 1;
            similarity_score += temp;
            continue;
        }
        if row1[i] == row2[j] {
            similarity_score += row1[i];
            j += 1;
            temp+= row1[i];
        } else if row1[i] < row2[j] {
            i += 1;
            temp = 0;
        } else {
            j += 1;
            temp = 0;
        }
    }
    println!("Sum: {}", sum);
    println!("Similarity score: {}", similarity_score);
}
