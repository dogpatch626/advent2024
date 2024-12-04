use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn daythree() {
    let input_file = BufReader::new(File::open("src/day3input.txt").unwrap());
    let mut count = 0;
    let matcher = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let mut matches: Vec<String> = Vec::new();
    for line in input_file.lines() {
        let idk: String = line.unwrap();
        for found in matcher.captures_iter(&idk) {
            let match_str = found.get(0).unwrap().as_str().to_string();
            matches.push(match_str);
        }
    }
    let mut sum = 0;
    for entry in matches.iter() {
        // lets not talk about it ..
        println!("{},", entry);
        let remove_mul: Vec<&str> = entry.split("mul").collect();
        let join_after_mul = remove_mul.join("");
        let removeParen: Vec<&str> = join_after_mul.split("(").collect();
        let joined = removeParen.join("");
        let formatted: Vec<&str> = joined.split(")").collect();
        let joinedAgain = formatted.join("");
        let formatted_again: Vec<&str> = joinedAgain.split(",").collect();
        let left: i32 = formatted_again[0].to_string().parse().unwrap();
        let right: i32 = formatted_again[1].to_string().parse().unwrap();
        let multiple = left * right;
        sum += multiple;
    }
    println!("{}", sum)
}
