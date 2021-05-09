use std::fs;
fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn parse_input() -> [[i32; 100]; 100] {
    let mut res :[[i32; 100]; 100] = [[0; 100]; 100];
    let mut x = 0;
    for line in load_input("18").split("\n"){
        
        let mut y = 0;
        for l in line.split(""){
            if l.len() == 0 {
                continue
            }
     
            res[x][y] = if l == "#" {
                1
            } else {
                0
            };
            y += 1;
        }
        x += 1;

    }

    res
}

fn neighbours(input: &[[i32; 100]; 100] ) -> [[i32; 100]; 100]  {
    let mut res : [[i32; 100]; 100]  = [[0; 100]; 100];

    let d_arr : [i32; 3] = [-1, 0, 1];

    for x in 0..input.len(){
        for y in 0..input[0].len(){
            let mut c = 0;

            for dx in &d_arr{
                for dy in &d_arr{
                    let nx :i32 = x as i32 + dx;
                    let ny :i32 = y as i32 + dy;
                    if (nx == x as i32 && ny == y as i32) || nx < 0 || ny < 0 || nx > 99 || ny > 99 {
                        continue;
                    }

                    c += input[nx as usize][ny as usize];

                }
            }

            res[x][y] = c

        }
    }

    res
}

fn new_lights(start:  &[[i32; 100]; 100], part_2: bool) ->  [[i32; 100]; 100] {
    let neighbours = neighbours(start);
    let mut res :  [[i32; 100]; 100] =  [[0; 100]; 100];

    for x in 0..100{
        for y in 0..100{
            if start[x][y] == 1 {
                if neighbours[x][y] == 2 || neighbours[x][y] == 3 {
                    res[x][y] = 1;
                } else {
                    res[x][y] = 0;
                }
            } else {
                if neighbours[x][y] == 3 {
                    res[x][y] = 1;
                } else {
                    res[x][y] = 0;
                }
            }
        }
    }

    if part_2 {
        res[0][0] = 1;
        res[99][0] = 1;
        res[0][99] = 1;
        res[99][99] = 1;
    }

    res
}

fn sol(part_2:bool) -> i32 {
    let mut start = parse_input();

    if part_2 {
        start[0][0] = 1;
        start[99][0] = 1;
        start[0][99] = 1;
        start[99][99] = 1;
    }

    for _ in 0..100{
        start = new_lights(&start, part_2);
    }

    let mut res =0;

    for i in 0..100{
        for j in 0..100{
            if start[i][j] == 1{
                res += 1;
            }
        }
    }

    res
}

fn main() {
    println!("{:?}", sol(false));
    println!("{:?}", sol(true));
}
