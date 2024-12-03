use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day_3.txt").expect("Couldn't read file");

    // part1(&contents);
    part2(&contents);
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: i32 = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [fac1, fac2]) = caps.extract();
            fac1.parse::<i32>().unwrap() * fac2.parse::<i32>().unwrap()
        })
        .sum();

    println!("Result for Part 1: {:?}", sum);
}

fn part2(input: &str) {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don?'?t?\(\)").unwrap();
    let instructions: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut enabled = true;
    let mut valid_instructions = Vec::new();

    for &i in instructions.iter() {
        if i == "do()" {
            enabled = true;
        } else if i == "don't()" {
            enabled = false;
        } else {
            if enabled {
                valid_instructions.push(i);
            }
        }
    }

    part1(&valid_instructions.join(""));

    // println!("Result for Part 2: {:?}", instructions);
}
