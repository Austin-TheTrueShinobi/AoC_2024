use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn calculate_similarity_score(left_list: &[i32], right_list: &[i32]) -> i32 {
    // Count occurrences of numbers in right list
    let right_counts: HashMap<i32, usize> = right_list
        .iter()
        .fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    // Calculate similarity score
    left_list.iter()
        .map(|&num| num * right_counts.get(&num).cloned().unwrap_or(0) as i32)
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the input file
    let file = File::open("C:\\Users\\mvppr\\Documents\\AoC_2024\\AoC\\src\\bin\\input.txt")?;
    let reader = BufReader::new(file);

    // Parse input into left and right lists
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        if parts.len() == 2 {
            left_list.push(parts[0]);
            right_list.push(parts[1]);
        }
    }

    // Calculate and print similarity score
    let similarity_score = calculate_similarity_score(&left_list, &right_list);
    println!("Similarity Score: {}", similarity_score);

    Ok(())
}