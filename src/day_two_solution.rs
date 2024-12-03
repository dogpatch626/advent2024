use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn daytwo() {
    let input_file = BufReader::new(File::open("src/day2input.txt").unwrap());
    let mut count = 0;
    for lines in input_file.lines() {
        let processed_line = lines.expect("could not process line");
        let numbers: Vec<&str> = processed_line.split_whitespace().collect();
        let mut increasing = false;
        let decide_movement_zeroth: i32 = numbers[0].parse().unwrap();
        let decide_movement_first: i32 = numbers[1].parse().unwrap();

        if decide_movement_zeroth < decide_movement_first {
            increasing = true;
        }

        for (i, numb) in numbers.iter().enumerate() {
            let curr: i32 = numbers[i].parse().unwrap();
            let next: i32 = numbers[i + 1].parse().unwrap();
            let delta: i32 = (curr - next).abs();
            let inner_increasing = curr < next;
            if delta < 1 || delta > 3 || increasing != inner_increasing {
                break;
            }
            if i == numbers.len() - 2 {
                count += 1;
                break;
            }
        }
    }
    println!("{}", count)
}
