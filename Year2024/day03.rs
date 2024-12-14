use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input/day03input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total_sum = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        total_sum += x * y;
    }

    println!("Part 1: Sum of results of multiplications: {}", total_sum);
}

fn part2(input: &str) {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    
    let mut total_sum = 0;
    let mut mul_enabled = true;
    
    for cap in re.captures_iter(input) {
        match cap.get(0).map(|m| m.as_str()) {
            Some("do()") => {
                mul_enabled = true;
            },
            Some("don't()") => {
                mul_enabled = false;
            },
            Some(mul_str) if mul_enabled => {
                if let Some(num_cap) = re.captures(mul_str) {
                    let x: i32 = num_cap[2].parse().unwrap();
                    let y: i32 = num_cap[3].parse().unwrap();
                    total_sum += x * y;
                }
            },
            _ => {}
        }
    }
    
    println!("Part 2: Sum of results of enabled multiplications: {}", total_sum);
}