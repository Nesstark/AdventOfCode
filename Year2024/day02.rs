use std::{fs, iter};

fn main() {
    let input = fs::read_to_string("input/day02input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let safe_count = input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            
            if levels.len() < 2 {
                return false;
            }

            let mut diffs = Vec::new();
            for window in levels.windows(2) {
                let diff = window[1] - window[0];
                if diff.abs() < 1 || diff.abs() > 3 {
                    return false;
                }
                diffs.push(diff);
            }

            diffs.iter().all(|&d| d > 0 || diffs.iter().all(|&d| d < 0))
    
        })
        .count();

    println!("Part 1: Number of safe reports: {}", safe_count);
}

fn part2(input: &str) {
    let safe_count = input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            if levels.len() < 2 {
                return false;
            }

            let is_safe = |levels: &[i32]| {
                let mut diffs = Vec::new();
                for window in levels.windows(2) {
                    let diff = window[1] - window[0];
                    if diff.abs() < 1 || diff.abs() > 3 {
                        return false;
                    }
                    diffs.push(diff);
                }

                diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0)
            };

            if is_safe(&levels) {
                return true;
            }

            (0..levels.len())
                .any(|i| {
                    let mut new_levels = levels.clone();
                    new_levels.remove(i);

                    is_safe(&new_levels)
                })
        })
        .count();

    println!("Part 2: Number of safe reports with Problem Dampener: {}", safe_count);
}