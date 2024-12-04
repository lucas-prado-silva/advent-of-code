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

    for i in 0..matrix.vertical_len {
        for j in 0..matrix.horizontal_len {
            // search horizontal right
            if j + 3 < matrix.horizontal_len && search_horizontal_to_right(&matrix, i, j) {
                println!("found horizontal right match");
                count += 1;
            }
            // search horizontal left
            if j >= 3 && search_horizontal_to_left(&matrix, i, j) {
                println!("found horizontal left match");
                count += 1;
            }
            // search vertical down
            if i + 3 < matrix.vertical_len && search_vertical_down(&matrix, i, j) {
                println!("found vertical down");
                count += 1;
            }
            // search vertical up
            if i >= 3 && search_vertical_up(&matrix, i, j) {
                println!("found vertical up");
                count += 1;
            }
            // search diagonal right down
            if j + 3 < matrix.horizontal_len && i + 3 < matrix.vertical_len && search_diagonal_right_down(&matrix, i, j) {
                println!("found diagonal right down");
                count += 1;
            }
            // search diagonal right up
            if j + 3 < matrix.horizontal_len && i >= 3 && search_diagonal_right_up(&matrix, i, j) {
                println!("found diagonal right up");
                count += 1;
            }
            // search diagonal left down
            if j >= 3 && i + 3 < matrix.vertical_len && search_diagonal_left_down(&matrix, i, j) {
                println!("found vertical diagonal left down");
                count += 1;
            }
            // search diagonal left up
            if j >= 3 && i >= 3 && search_diagonal_left_up(&matrix, i, j) {
                println!("found vertical diagonal left up");
                count += 1;
            }
        }
    }

    println!("{count}");
}

fn search_horizontal_to_right(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i][j + 1] == 'M'
        && matrix.matrix[i][j + 2] == 'A'
        && matrix.matrix[i][j + 3] == 'S'
}

fn search_horizontal_to_left(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i][j - 1] == 'M'
        && matrix.matrix[i][j - 2] == 'A'
        && matrix.matrix[i][j - 3] == 'S'
}

fn search_vertical_down(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i + 1][j] == 'M'
        && matrix.matrix[i + 2][j] == 'A'
        && matrix.matrix[i + 3][j] == 'S'
}

fn search_vertical_up(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i - 1][j] == 'M'
        && matrix.matrix[i - 2][j] == 'A'
        && matrix.matrix[i - 3][j] == 'S'
}

fn search_diagonal_right_down(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i + 1][j + 1] == 'M'
        && matrix.matrix[i + 2][j + 2] == 'A'
        && matrix.matrix[i + 3][j + 3] == 'S'
}

fn search_diagonal_left_down(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i + 1][j - 1] == 'M'
        && matrix.matrix[i + 2][j - 2] == 'A'
        && matrix.matrix[i + 3][j - 3] == 'S'
}

fn search_diagonal_right_up(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i - 1][j + 1] == 'M'
        && matrix.matrix[i - 2][j + 2] == 'A'
        && matrix.matrix[i - 3][j + 3] == 'S'
}

fn search_diagonal_left_up(matrix: &Matrix, i: usize, j: usize) -> bool {
    matrix.matrix[i][j] == 'X'
        && matrix.matrix[i - 1][j - 1] == 'M'
        && matrix.matrix[i - 2][j - 2] == 'A'
        && matrix.matrix[i - 3][j - 3] == 'S'
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
