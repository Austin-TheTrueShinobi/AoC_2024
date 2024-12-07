/*use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

fn parse_mul_instructions(input: &str) -> i32 {
    // Regex for different instruction types
    let mul_regex = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    // Track multiplication state
    let mut mul_enabled = true;

    // Collect and sum all valid multiplication results
    mul_regex.captures_iter(input)
        .fold(0, |acc, cap| {
            // Check for state change instructions before processing mul
            for state_match in do_regex.find_iter(input).chain(dont_regex.find_iter(input)) {
                if state_match.start() < cap.get(0).unwrap().start() {
                    mul_enabled = state_match.as_str() == "do()";
                }
            }

            // Process multiplication if enabled
            if mul_enabled {
                match (
                    cap.get(1)?.as_str().parse::<i32>().ok(),
                    cap.get(2)?.as_str().parse::<i32>().ok()
                ) {
                    (Some(x), Some(y)) => acc + (x * y),
                    _ => acc
                }
            } else {
                acc
            }
        })
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
            // Parse and calculate the sum of enabled mul instructions
            let result = parse_mul_instructions(&contents);
            println!("Sum of enabled mul instructions: {}", result);
        },
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}*/
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn parse_and_evaluate(input: &str) -> i64 {
    let mul_regex = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut result = 0;
    let mut mul_enabled = true;

    // Iterate through matches, handling different instruction types
    let mut last_index = 0;
    while let Some(mul_match) = mul_regex.find_at(input, last_index) {
        // Check for do() or don't() instructions before this multiplication
        let pre_slice = &input[last_index..mul_match.start()];
        
        if do_regex.is_match(pre_slice) {
            mul_enabled = true;
        }
        
        if dont_regex.is_match(pre_slice) {
            mul_enabled = false;
        }

        // Process multiplication if enabled
        if mul_enabled {
            if let Some(caps) = mul_regex.captures(&input[mul_match.start()..mul_match.end()]) {
                let a: i64 = caps[1].parse().unwrap();
                let b: i64 = caps[2].parse().unwrap();
                result += a * b;
            }
        }

        last_index = mul_match.end();
    }
    result
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
            // Parse and calculate the sum of enabled mul instructions
            let result = parse_and_evaluate(&contents);
            println!("Sum of enabled mul instructions: {}", result);
        },
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}