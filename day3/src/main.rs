use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = Vec::new();

    for cap in re.captures_iter(&data){
        let first: u32 = cap[1].parse().unwrap();
        let second: u32 = cap[2].parse().unwrap();
        result.push((first, second));
    }

    let total:u32 = result.iter().map(|(a,b)| a * b).sum();

    println!("part 1: {}", total);

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut result2 = Vec::new();

    for cap in re.captures_iter(&data){
        if &cap[0] == "do()"{
            enabled = true;
        } else if &cap[0] == "don't()"{
            enabled = false;
        } else if enabled{
            let first: u32 = cap[1].parse().unwrap();
            let second: u32 = cap[2].parse().unwrap();
            result2.push((first, second));
        }
    }

    let total2:u32 = result2.iter().map(|(a,b)| a * b).sum();

    println!("part 2: {}", total2);
}
