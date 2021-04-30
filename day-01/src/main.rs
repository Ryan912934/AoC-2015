use std::fs;

fn part_1 (input :&str) -> i32 {

    let mut floor :i32 = 0;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1
        }
    }

    floor
}

fn part_2 (input :&str) -> i32 {

    let mut floor :i32 = 0;
    let mut count :i32 = 0;

    for c in input.chars() {
        count += 1;
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1
        }

        if floor == -1 {
            break
        }
    }

    count   
}

fn main() {

    let contents = fs::read_to_string("/home/ryan/Documents/AoC/2015/inputs/day01.txt")
        .expect("Something went wrong reading the file");


    

    println!("On Floor {}", part_1(&contents));
    println!("Hit the bassment at index {}", part_2(&contents));
}
