use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day_2_example.txt").expect("Couldn't read file");

    // part1(&contents);
    part2(&contents);
}

fn part1(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut safe_count = 0;

    for report in lines.iter() {
        // println!("--- REPORT ITEM --- {}", report);
        let report_items = report
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let report_increasing = report_items[0] < report_items[1];

        for (i, &report_item) in report_items.iter().enumerate() {
            if i > 0 {
                let previous_item = report_items[i - 1];

                // Check if still increasing/decreasing
                let currently_increasing = report_item > previous_item;

                let distance = (report_item - previous_item).abs();

                let distance_valid = (distance > 0) && (distance < 4);

                // println!("Checking {:?}: (Previous Item is {:?} – Keep Sorting {} – Distance {} - Distance valid {})", report_item, previous_item, report_increasing == currently_increasing, distance, distance_valid);

                let item_is_safe = (report_increasing == currently_increasing) && distance_valid;

                if item_is_safe {
                    // println!("Potentially safe: {:?} : {}", report, i);
                    if i == (report_items.len() - 1) {
                        safe_count = safe_count + 1;
                        println!("Safe report: {:?}", report);
                    } else {
                        continue;
                    }
                } else {
                    break;
                }
            }
        }
        // println!("--- END REPORT ITEM --- {}", report);
    }

    println!("Result for Part 1: {:?}", safe_count);
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut modified_reports: Vec<Vec<i32>> = Vec::new();
    let mut safe_count = 0;

    for report in lines.iter() {
        println!("--- REPORT ITEM --- {}", report);
        let report_items = report
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let report_increasing = report_items[0] < report_items[1];
        let mut unsafe_count = 0;

        for (i, &report_item) in report_items.iter().enumerate() {
            if i > 0 {
                let previous_item = report_items[i - 1];

                let mut currently_increasing = report_item > previous_item;

                if report_item == previous_item {
                    currently_increasing = !report_increasing
                }

                let distance = (report_item - previous_item).abs();

                let distance_valid = (distance > 0) && (distance < 4);

                println!("Checking {:?}: (Previous Item is {:?} – Monotony? {} – Distance ({}) valid? {})", report_item, previous_item, report_increasing == currently_increasing, distance, distance_valid);

                let item_is_safe = (report_increasing == currently_increasing) && distance_valid;

                if item_is_safe {
                    // println!("Potentially safe: {:?} : {}", report, i);
                    if i == (report_items.len() - 1) && unsafe_count == 0 {
                        safe_count = safe_count + 1;
                        println!("Report is safe: {:?} Unsafe: {}", report, unsafe_count);
                    } else {
                        continue;
                    }
                } else {
                    unsafe_count = unsafe_count + 1;
                    let mut permutations :Vec<i32> = Vec::new();
                    
                    for i in 0..report.len() + 1 {

                    }
                }
            }
        }
        if unsafe_count == 1 {
            println!("Report is safe with one level removed: {:?}", report);
            safe_count = safe_count + 1;
        }
        println!("--- END REPORT ITEM --- {}", report);
    }

    println!("Result for Part 2: {:?}", safe_count);
}
