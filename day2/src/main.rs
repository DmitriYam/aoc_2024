use std::fs;

fn is_safe_1(report: &Vec<i32>) -> bool{
    let mut f = report.first().unwrap();

    for data in report.iter().skip(1){
        let temp = data;
        let diff = (data-f).abs();
        print!("{}",diff);
        if diff < 1 || diff > 3{
            return false
        }
        f = temp;
    }

    if report.is_sorted() || report.is_sorted_by(|a,b| a > b){
        print!("sorted");
        return true;
    }
    return false;
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut safe_1 = 0;
    for lines in data.lines(){

        let report: Vec<i32> = lines.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();

        if is_safe_1(&report){
            println!(" {} is safe!", lines);
            safe_1 += 1;
        }
        else{
            println!(" {} is not safe!", lines);
        }
    }
    println!("Total number of safe data: {}", safe_1);
    //part 2
    println!("-----------------");
    let mut safe_2 = 0;
    for lines in data.lines(){

        let report: Vec<i32> = lines.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();


        if is_safe_1(&report){
            println!(" {} is safe!", lines);
            safe_2 += 1;
        }
        else{
            let len = report.len();
            let mut i = 0;
            while i < len {
                let mut v = report.clone();
                v.remove(i);
                if is_safe_1(&v){
                    safe_2 += 1;
                    break;
                }
                i +=1;
            }
            println!(" {} is not safe!", lines);
        }
    }
    println!("Total number of safe data: {}", safe_2);

}
