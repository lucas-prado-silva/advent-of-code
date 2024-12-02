use core::panic;
use std::fs::File;
use std::i64;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

fn main() {
    let mut vec1: Vec<i64> = Vec::new();
    let mut vec2: Vec<i64> = Vec::new();

    // File input1.txt must exist in the current path
    if let Ok(lines) = read_lines("./input1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("line {:?}", line);
            let mut split_res = line.split_whitespace();
            let first_value = split_res.next();
            match first_value {
                Some(x) => {
                    let number = x.parse(); 
                    match number {
                        Ok(x) => {
                            println!("left {x}");
                            vec1.push(x)
                        }
                        Err(_) => {
                            println!("bad string {}", first_value.unwrap().to_string())
                        }
                    }
                }
                None => panic!("did not find first value"),
            }

            let second_value = split_res.next();
            match second_value {
                Some(x) => {
                    let number = x.parse(); 
                    match number {
                        Ok(x) => {
                            println!("right {x}");
                            vec2.push(x)
                        }
                        Err(_) => {
                            println!("bad string{}", second_value.unwrap().to_string())
                        }
                    }
                }
                None => panic!("did not find second value"),
            }
        }
    }

    vec1.sort();
    vec2.sort();

    println!("left sorted {:?}", vec1);
    println!("right sorted {:?}", vec2);

    assert!(vec1.len() == vec2.len(), "vecs are not the same len");

    let mut sum: i64 = 0;
    for n in 0..=vec1.len() - 1 {
        let first_list_value = vec1.index(n);
        let second_list_value = vec2.index(n);
        let difference = (first_list_value - second_list_value).abs();
        println!("{difference}");
        sum += difference;
    }

    println!("{}", sum)
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}