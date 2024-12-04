use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Matrix {
    matrix: Vec<Vec<char>>,
    horizontal_len: usize,
    vertical_len: usize,
}

fn main() {
    let matrix = read_matrix("input1.txt");

    let matrix = Matrix {
        // cloning to avoid borrowing and dealing with lifetimes
        matrix: matrix.clone(),
        horizontal_len: matrix[0].len(),
        vertical_len: matrix.len(),
    };

    let mut count = 0;

    for i in 1..matrix.vertical_len - 1 {
        for j in 1..matrix.horizontal_len - 1 {
            if matrix.matrix[i][j] == 'A' {
                if has_xmas(&matrix, i, j) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

fn has_xmas(matrix: &Matrix, i: usize, j: usize) -> bool {
    let left_up = matrix.matrix[i - 1][j - 1];
    let left_down = matrix.matrix[i + 1][j - 1];
    let right_up = matrix.matrix[i - 1][j + 1];
    let right_down = matrix.matrix[i + 1][j + 1];

    // if has one diagonal, check the other diagonal
    if left_up == 'M' && right_down == 'S' || left_up == 'S' && right_down == 'M' {
        return left_down == 'M' && right_up == 'S' || left_down == 'S' && right_up == 'M';
    }

    false
}

fn read_matrix(file_name: &str) -> Vec<Vec<char>> {
    let br = BufReader::new(File::open(file_name).unwrap());
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in br.lines() {
        match line {
            Ok(line) => {
                matrix.push(line.chars().collect());
            }
            Err(_) => (),
        }
    }

    matrix
}
