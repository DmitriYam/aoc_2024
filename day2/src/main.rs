use std::fs;

fn is_safe(report: Vec<i32>) -> bool{
    
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut safe = 0;

    for lines in data.lines(){

        let report: Vec<i32> = lines.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();

        let safe = is_safe(report);
    }
}
