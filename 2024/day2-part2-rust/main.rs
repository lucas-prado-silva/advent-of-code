use std::fs::{self};

fn main() {
    let mut safe_reports = 0;

    let input: Vec<Vec<isize>> = input_generator(&fs::read_to_string("input1.txt").unwrap());

    for report in input {
        if is_report_safe(&report) || is_any_report_variation_safe(report) {
            safe_reports += 1;
        }
    }

    println!("{safe_reports}");
}

fn input_generator(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_any_report_variation_safe(report: Vec<isize>) -> bool {
    for i in 0..report.len() {
        let mut mutated_report = report.clone();
        mutated_report.remove(i);


        if is_report_safe(&mutated_report) {
            return true;
        }
    }

    false
}

fn is_report_safe(report: &Vec<isize>) -> bool {
    if report.len() == 0 {
        return false;
    }

    if report.len() == 1 {
        return true;
    }

    let first_level = report.get(0).unwrap();
    let second_level = report.get(1).unwrap();
    let is_incrementing = first_level < second_level;

    for level_index in 1..=report.len() - 1 {
        let level = report.get(level_index).unwrap();
        let previous_level = report.get(level_index - 1).unwrap();

        let difference = level - previous_level;
        if is_incrementing {
            if difference <= 0 || difference > 3 {
                return false;
            }
        } else {
            if difference >= 0 || difference < -3 {
                return false;
            }
        }
    }

    true
}
