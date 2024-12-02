use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_safe_report(levels: &[i32]) -> bool {
    let is_trend_safe = |report: &[i32]| {
        let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();
        let is_increasing = diffs.iter().all(|&d| d > 0 && d <= 3);
        let is_decreasing = diffs.iter().all(|&d| d < 0 && d >= -3);
        is_increasing || is_decreasing
    };
    
    // Check if original report is safe
    if is_trend_safe(levels) {
        return true;
    }
    
    // Try removing each level to see if report becomes safe
    for i in 0..levels.len() {
        let mut dampened_report = levels.to_vec();
        dampened_report.remove(i);
        
        if is_trend_safe(&dampened_report) {
            return true;
        }
    }
    
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("C:\\Users\\mvppr\\Documents\\AoC_2024\\AoC\\src\\bin\\inputs\\Day2.txt")?;
    let reader = BufReader::new(file);
    
    let safe_reports = reader
        .lines()
        .filter_map(|line| {
            line.ok().map(|l| 
                l.split_whitespace()
                 .map(|s| s.parse::<i32>().unwrap())
                 .collect::<Vec<i32>>()
            )
        })
        .filter(|report| is_safe_report(report))
        .count();
    
    println!("Number of safe reports with Problem Dampener: {}", safe_reports);
    
    Ok(())
}