use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day_1.txt").expect("Couldn't read file");

    part1(&contents);
    part2(&contents);
}

fn part1(input: &str) {
    // Create vectors to hold the individual columns from the input
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines.iter() {
        // Split every line in two strings and then convert them to integers
        let parts = line
            .split("   ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Add the integers to the prepared column vectors
        left_column.push(parts[0]);
        right_column.push(parts[1]);
    }

    // Sort the columns
    left_column.sort();
    right_column.sort();

    let mut overall_sum = 0;

    for (i, _left_column_item) in left_column.iter().enumerate() {
        let line_sum = left_column[i] - right_column[i];

        overall_sum = overall_sum + line_sum.abs();
    }

    println!("Result for Part 1: {:?}", overall_sum);
}

fn part2(input: &str) {
    let mut left_column: Vec<i32> = Vec::new();
    // Create a hash map to store the amount of digits in the right column
    let mut number_frequency: HashMap<i32, i32> = HashMap::new();

    let mut similarity_score = 0;

    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines.iter() {
        // Split every line in two strings and then convert them to integers
        let parts = line
            .split("   ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Add the digit count to the map
        *number_frequency.entry(parts[1]).or_insert(0) += 1;

        // Also create a proper vector for the left column needed in the next step
        left_column.push(parts[0]);
    }

    for &n in left_column.iter() {
        let score = n * number_frequency.get(&n).copied().unwrap_or(0);
        similarity_score = similarity_score + score;
    }
    println!("Result for Part 2: {:?}", similarity_score);
}
