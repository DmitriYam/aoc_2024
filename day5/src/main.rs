use std::fs;
use std::collections::HashMap;

fn part1(input: &Vec<i32>, rules: &HashMap<i32,Vec<i32>>) -> bool{
    for i in 0..input.len()-1{
        for j in i+1..input.len(){
            let key = input[i];
            let val = input[j];
            //print!("{},{} ", i,j);
            
            match rules.get(&key){
                Some(v) => {
                    if !v.contains(&val){
                        return false;
                    }
                },
                None => return false
            }
            
        }
    }
    //println!("{:?}", input);
    true
}

fn part2(input: &Vec<i32>, rules: &HashMap<i32,Vec<i32>>) -> bool{
    for i in 0..input.len()-1{
        for j in i+1..input.len(){
            let key = input[i];
            let val = input[j];
            //print!("{},{} ", i,j);
            
            match rules.get(&key){
                Some(v) => {
                    if !v.contains(&val){
                        return false;
                    }
                },
                None => return false
            }
            
        }
    }
    //println!("{:?}", input);
    true
}

fn main() {
    //parsing
    let data = fs::read_to_string("1.txt").expect("file error");

    let parts: Vec<&str> = data.split("\n\n").collect();
    let lines_parts: Vec<&str>  = parts[0].lines().collect();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for l in lines_parts{
        let v: Vec<&str> = l.split("|").collect();
        let key  = v[0].parse::<i32>().unwrap(); 
        let value = v[1].parse::<i32>().unwrap(); 
        rules.entry(key).or_insert(Vec::new()).push(value);
    }
    let mut sum =0;
    for lines in parts[1].lines(){
        let sequence: Vec<i32> = lines.split(",").map(|x|->i32{x.parse().unwrap()}).collect();
        
        if part1(&sequence, &rules){
            sum += sequence[sequence.len() / 2];
        }
    }

    println!("part1: {}", sum);

    for lines in parts[1].lines(){
        let sequence: Vec<i32> = lines.split(",").map(|x|->i32{x.parse().unwrap()}).collect();
        
        if !part1(&sequence, &rules){
            
        }
    }
}
