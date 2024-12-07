/*use std::fs::File;
use std::io::{Read};

fn parse_mul_instructions(memory: &str) -> Vec<i32> {
    let mut results = Vec::new();
    let mut chars = memory.chars().peekable();

    while let Some(_) = chars.peek() {
        if let Some(result) = find_valid_mul(&mut chars) {
            results.push(result);
        } else {
            chars.next();
        }
    }

    results
}

fn find_valid_mul(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<i32> {
    // Check for 'mul(' precisely
    if chars.clone().take(4).collect::<String>() != "mul(" {
        return None;
    }

    // Consume 'mul('
    for _ in 0..4 { chars.next(); }

    // Parse first number
    let first_num = parse_number(chars)?;

    // Skip potential whitespace or invalid characters
    while chars.peek().map_or(false, |&c| !c.is_digit(10)) {
        chars.next();
    }

    // Check for comma
    if chars.peek() != Some(&',') { return None; }
    chars.next();

    // Parse second number
    let second_num = parse_number(chars)?;

    // Find closing ')'
    while chars.peek().map_or(false, |&c| c != ')') {
        chars.next();
    }

    chars.next(); // Consume ')'

    Some(first_num * second_num)
}

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<i32> {
    let mut num_str = String::new();
    while chars.peek().map_or(false, |&c| c.is_digit(10)) {
        num_str.push(chars.next()?);
    }
    num_str.parse().ok()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("C:\\Users\\mvppr\\Documents\\AoC_2024\\AoC\\src\\bin\\inputs\\test.txt")?;
    let mut memory = String::new();
    file.read_to_string(&mut memory)?;

    let mul_results = parse_mul_instructions(&memory);
    let total_sum: i32 = mul_results.iter().sum();
    
    println!("Sum of mul instruction results: {}", total_sum);
    println!("Individual mul results: {:?}", mul_results);

    Ok(())
}
*/
use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

fn parse_mul_instructions(input: &str) -> i32 {
    // Regex to match valid mul instructions with optional whitespace and no invalid characters
    let mul_regex = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();
    
    // Collect and sum all valid multiplication results
    mul_regex.captures_iter(input)
        .filter_map(|cap| {
            // Safely extract and parse the two numbers
            let x = cap.get(1)?.as_str().parse::<i32>().ok()?;
            let y = cap.get(2)?.as_str().parse::<i32>().ok()?;
            Some(x * y)

        })
        .sum()
}

fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    // Open the file
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    
    // Read the entire contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn main() {
    // File path can be passed as an argument or hardcoded
    let file_path = "C:\\Users\\mvppr\\Documents\\AoC_2024\\AoC\\src\\bin\\inputs\\Day3.txt";
    
    // Read the file contents
    match read_file_to_string(file_path) {
        Ok(contents) => {
            // Parse and calculate the sum of mul instructions
            let result = parse_mul_instructions(&contents);
            println!("Sum of valid mul instructions: {}", result);
        },
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}