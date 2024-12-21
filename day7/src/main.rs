use std::{fs};

fn main() {
    //parsing
    let data = fs::read_to_string("input.txt").expect("file error");

    let lines_parts: Vec<&str>  = data.lines().collect();
    /*
    let mut sum1 = 0;
    for l in lines_parts{
        let parsed: Vec<&str> = l.split(":").collect();
        let tgt = parsed[0].parse::<i64>().unwrap();
        //print!("{}", l);
        let values:Vec<i64> = parsed[1].split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let len = values.len() - 1;
        let mut stack:Vec<(i64,usize)> = Vec::new();
        stack.push((values[0],1));
        loop{
            if stack.is_empty(){
               // print!("no buddy");
                break;
            }
            let pair :(i64,usize) = stack.pop().unwrap();

            let accum = pair.0;
             
            if pair.1 > len {
                if accum == tgt{
                    sum1 = sum1 + tgt;
                    break;
                }
            }
            else{
                let add_pair = (accum + values[pair.1], pair.1 + 1);
                let mul_pair = (accum * values[pair.1], pair.1 + 1);
                
                stack.push(add_pair);
                stack.push(mul_pair);
            }
        }
    }

    println!("part 1: {}", sum1);
    */
    let mut sum2 = 0;
    for l in lines_parts{
        let parsed: Vec<&str> = l.split(":").collect();
        let tgt = parsed[0].parse::<i64>().unwrap();
        //print!("{}", l);
        let values:Vec<i64> = parsed[1].split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let len = values.len() - 1;
        let mut stack:Vec<(i64,usize)> = Vec::new();
        stack.push((values[0],1));
        loop{
            if stack.is_empty(){
               // print!("no buddy");
                break;
            }
            let pair :(i64,usize) = stack.pop().unwrap();

            let accum = pair.0;
             
            if pair.1 > len {
                if accum == tgt{
                    sum2 = sum2 + tgt;
                    break;
                }
            }
            else{
                let add_pair = (accum + values[pair.1], pair.1 + 1);
                let mul_pair = (accum * values[pair.1], pair.1 + 1);
                let str_accum = accum.to_string();
                let str_next = values[pair.1].to_string();
                let concat = str_accum + &str_next;
                //print!("{} ", concat);
                let concat_pair = (concat.parse::<i64>().unwrap(), pair.1 + 1);

                stack.push(add_pair);
                stack.push(mul_pair);
                stack.push(concat_pair);
            }
        }
    }

    println!("part 2: {}", sum2);

}