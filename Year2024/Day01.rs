use std::env;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("input/day01input.txt").expect("Something went wrong reading the file");

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();

        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    let differences: Vec<u64> = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .collect();

    let total_difference: u64 = differences.iter().sum();
    println!("Total difference: {}", total_difference);

    part2();
}

fn part2() {
    let input =
        fs::read_to_string("input/day01input.txt").expect("Something went wrong reading the file");

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();

        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    let mut total_similarity = 0;

    for &num in &left {
        let count = right.iter().filter(|&&x| x == num).count();
        total_similarity += num * count as u64;
    }

    println!("Total similarity: {}", total_similarity);
}