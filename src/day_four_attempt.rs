use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn matrix_forge() -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];

    let input_data = BufReader::new(File::open("./src/day2input.txt").unwrap());

    for line in input_data.lines() {
        let extracted: String = line.unwrap();
        let splut_extraction: Vec<char> = extracted.chars().collect();
        matrix.push(splut_extraction);
    }
    return matrix;
}

fn dfs_helper(
    row: i32,
    col: i32,
    row_l: i32,
    col_l: i32,
    matrix: Vec<Vec<char>>,
    mut deq: Vec<char>,
) -> bool {
    if row < 0 || row > row_l || col < 0 || col > col_l {
        return false;
    }
    let mut joined: String = "".to_string();
    deq.push(matrix[row as usize][col as usize]);
    if deq.len() == "XMAS".len() {
        joined = deq.iter().copied().collect();
    }

    if joined == "XMAS" {
        // println!("{}, {}", row, col);
        // println!("{}", joined);
        return true;
    } else if deq.len() >= "XMAS".len() && joined != "XMAS" {
        return false;
    }
    return (dfs_helper(row + 1, col, row_l, col_l, matrix.clone(), deq.clone())
        || dfs_helper(row, col + 1, row_l, col_l, matrix.clone(), deq.clone())
        || dfs_helper(row - 1, col, row_l, col_l, matrix.clone(), deq.clone())
        || dfs_helper(row, col + 1, row_l, col_l, matrix.clone(), deq.clone())
        || dfs_helper(row + 1, col + 1, row_l, col_l, matrix.clone(), deq.clone())
        || dfs_helper(row - 1, col - 1, row_l, col_l, matrix.clone(), deq.clone())
        || dfs_helper(row - 1, col + 1, row_l, col_l, matrix.clone(), deq.clone())
        || dfs_helper(row + 1, col - 1, row_l, col_l, matrix.clone(), deq.clone()));
}

fn main() {
    let mut matrix: Vec<Vec<char>> = matrix_forge();
    let mut deq: Vec<char> = vec![];

    let mut row_length = matrix.len() as i32;
    let mut col_length = matrix[0].len() as i32;
    let mut count = 0;
    row_length -= 1;
    col_length -= 1;
    for row in 0..row_length {
        for col in 0..col_length {
            if matrix[row as usize][col as usize] == 'X' {
                let found = dfs_helper(
                    row,
                    col,
                    row_length,
                    col_length,
                    matrix.clone(),
                    deq.clone(),
                );

                if found {
                    count += 1
                }
            }
        }
    }

    println!("{}", count);
}
