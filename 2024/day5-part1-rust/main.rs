use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug)]
struct OrderingRule(usize, usize);

struct Input {
    ordering_rules: Vec<OrderingRule>,
    updates: Vec<Vec<usize>>,
}

fn main() {
    let input = read_input("input1.txt");

    let mut sum = 0;

    for update in input.updates {
        let applicable_ordering_rules: Vec<&OrderingRule> = input
            .ordering_rules
            .iter()
            .filter(|ordering_rule| {
                update.contains(&ordering_rule.0) && update.contains(&ordering_rule.1)
            })
            .collect();

        let is_update_in_right_order = applicable_ordering_rules.iter().all(|ordering_rule| {
            let update_iterator = update.iter();
            let before_position = update_iterator
                .clone()
                .position(|&e| e == ordering_rule.0)
                .unwrap();
            let after_position = update_iterator
                .clone()
                .position(|&e| e == ordering_rule.1)
                .unwrap();
            before_position < after_position
        });

        if !is_update_in_right_order {
            continue;
        }

        sum += update[update.len() / 2];
    }

    println!("{sum}")
}

fn read_input(file_name: &str) -> Input {
    let br = BufReader::new(File::open(file_name).unwrap());
    let mut input = Input {
        ordering_rules: Vec::new(),
        updates: Vec::new(),
    };

    let mut is_reading_ordering_rules = true;
    for line in br.lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    is_reading_ordering_rules = false;
                    continue;
                }
                if is_reading_ordering_rules {
                    let vec: Vec<usize> = line.split("|").map(|c| c.parse().unwrap()).collect();
                    let ordering_rule = OrderingRule(vec[0], vec[1]);
                    input.ordering_rules.push(ordering_rule);
                }
                if !is_reading_ordering_rules {
                    let vec: Vec<usize> = line.split(",").map(|c| c.parse().unwrap()).collect();
                    input.updates.push(vec);
                }
            }
            Err(_) => (),
        }
    }

    input
}
