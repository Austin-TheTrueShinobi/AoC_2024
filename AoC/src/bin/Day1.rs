use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_total_distance(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) -> i32 {
    // Sort both lists
    left_list.sort_unstable();
    right_list.sort_unstable();

    // Calculate distances between paired elements
    left_list.iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
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

    // Calculate and print total distance
    let total_distance = calculate_total_distance(&mut left_list, &mut right_list);
    println!("Total distance between lists: {}", total_distance);

    Ok(())
}