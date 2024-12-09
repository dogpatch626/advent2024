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

fn main() {
    let mut matrix: Vec<Vec<char>> = matrix_forge();
    let mut deq: Vec<i32> = vec![];

    let mut row_length = matrix.len() as i32;
    let mut col_length = matrix[0].len() as i32;
    let mut count = 0;
    row_length -= 1;
    col_length -= 1;
    for row in 0..row_length {
        for col in 0..col_length {
            if matrix[row as usize][col as usize] == '^' {
                deq.push(row);
                deq.push(col);
            }
        }
    }

    let mut row = deq[0];
    let mut col = deq[1];
    let mut count = 0;
    let directions = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    // let mut direction = "up";
    let mut direct = 0;
    while row >= 0 && col >= 0 && row <= row_length && col <= col_length {
        println!("{},{}", row, col);

        let mut checkr = row + directions[direct][0];
        let mut checkc = col + directions[direct][1];
        if !(checkr >= 0 && checkc >= 0 && checkr <= row_length && checkc <= col_length) {
            println!("{}", count);
            break;
        }

        if matrix[checkr as usize][checkc as usize] == '#' {
            if direct != 3 {
                direct += 1
            } else {
                direct = 0
            }
        }
        if matrix[row as usize][col as usize] == 'X' {
            matrix[row as usize][col as usize] = '+'
        } else {
            matrix[row as usize][col as usize] = 'X';
            count += 1;
        }
        row += directions[direct][0];
        col += directions[direct][1];
    }

    println!("{:?}", count + 1);
}
