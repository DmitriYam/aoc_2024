use std::fs;
use std::collections::HashMap;

pub fn elementwise_sub_and_sum(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v1.into_iter()
        .zip(v2)
        .map(|(e1, e2)| (e1 - e2).abs())
        .sum()
}

fn main() {

    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for lines in data.lines(){
        let l = lines.split_whitespace();
        let mut collection = l.collect::<Vec<&str>>();
        
        let first = collection.pop().unwrap();
        right_list.push(first.parse::<i32>().unwrap());
        let second = collection.pop().unwrap();
        left_list.push(second.parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let diff = elementwise_sub_and_sum(&left_list, &right_list);

    println!("part 1: {}", diff);

    //part 2

    let mut map: HashMap<i32, i32> = HashMap::new();

    for e2 in right_list.iter(){
        *map.entry(*e2).or_insert(0) += 1;
    }

    let mut c = 0;
    for e1 in left_list.iter(){
        let occurrences = match map.get(e1){
            Some(val) => val,
            None => &0
        };
        c += occurrences * e1;
    }

    println!{"part 2: {}", c};
}