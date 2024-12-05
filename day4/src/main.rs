use std::fs;

fn check_valid(grid: &Vec<Vec<char>>, di: i32, dj: i32, i:i32, j:i32, key: &str) -> bool{
    let len = key.len();

    for pos in 0..len{
        let x = i + pos as i32 *di;
        let y = j + pos as i32 *dj;
        if x < 0 || y < 0 || x as usize >= grid.len() || y as usize >= grid[0].len(){
            return false
        }
        if grid[x as usize][y as usize] != key.chars().nth(pos).unwrap(){
            return false
        }
    }
    true
}

fn check_valid_2(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool{

    let combinations = [
        ('M', 'M', 'A', 'S', 'S'),
        ('S', 'M', 'A', 'M', 'S'),
        ('S', 'S', 'A', 'M', 'M'),
        ('M', 'S', 'A', 'S', 'M'),
    ];
    for (tl,tr,m,br,bl) in combinations{
        if grid[i][j] == tl 
            && grid[i][j+2] == tr
            && grid[i+1][j+1] == m
            && grid[i+2][j] == bl
            && grid[i+2][j+2] == br{
                return true;
            }
    }

    false
}


fn main() {
    let data = fs::read_to_string("input.txt").expect("file error");

    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut num = 0;
    let direction = [
        (0, 1),   // Horizontal right
        (1, 0),   // Vertical down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (0, -1),  // Horizontal left
        (-1, 0),  // Vertical up
        (-1, -1), // Diagonal up-left
        (-1, 1),  // Diagonal up-right(0, 1)
    ];

    for (i, row) in grid.iter().enumerate(){
        for (j, _) in row.iter().enumerate() {
            for &(di, dj) in &direction{
                if check_valid(&grid, di, dj, i as i32, j as i32, "XMAS"){
                    num += 1;
                }
            }
        }
    }

    println!("Part 1: {}", num);

    //part 2 check for 4 fixed patterns for every cell
    let mut num_2 = 0;
    for i in 0..grid.len()-2{
        for j in 0..grid[i].len()-2 {
            if check_valid_2(&grid, i, j){
                num_2 += 1;
            }
        }
    }

    println!("Part 2: {}", num_2);
}
