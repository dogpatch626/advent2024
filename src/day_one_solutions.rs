// use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn dayonesolution() {
    let mut input_file = BufReader::new(File::open("src/day1input.txt").unwrap());
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input_file.lines() {
        let processed_line = line.expect("could not read line");

        let words: Vec<&str> = processed_line.split_whitespace().collect();

        left.push(words[0].parse().unwrap());
        right.push(words[1].parse().unwrap());
    }

    let mut occurrence: HashMap<i32, i32> = HashMap::new();

    for (i, &number) in left.iter().enumerate() {
        occurrence.insert(number, 0);
    }
    println!("{:?}", left);

    for number in right.iter() {
        let numb: i32 = *number;
        if occurrence.contains_key(&numb) {
            // let number_from_occurrence = *occurrence.entry(numb).or_insert(0);
            *occurrence.entry(numb).or_insert(0) += 1;
            // occurrence.insert(numb, number_from_occurrence as i32 + 1);
        }
    }
    println!("{:?}", occurrence);
    let mut summation: i32 = 0;
    for number in left.iter() {
        let occurance_value = occurrence.get(number).unwrap();
        let similarity_sub_score = number * occurance_value;
        summation += similarity_sub_score;
    }
    println!("{}", summation);
}
