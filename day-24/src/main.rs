use std::fs;
use itertools::Itertools;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}


fn vec_diff(vec_all : &Vec<i32>, vec_slice :&Vec<i32>) -> Vec<i32> {
    let mut res : Vec<i32> = Vec::new();

    for i in vec_all.into_iter() {
        if !vec_slice.iter().any(|&j| &j == i ){
            res.push(*i);
        }
    }


    res
}

fn vec_prod(v : &Vec<i32>) -> i64 {
    let mut res :i64 = 1;

    for i in v.iter(){
        res *= *i as i64
    } 

    res
}

fn can_this_split( v :&Vec<i32>, amt :i32, ways :i32) -> bool{
    if ways == 1 {
        let mut sum = 0;
        for i in v.iter() {
            sum += i;
        }
        return sum == amt
    }

    let max_size = *(&v.len());

    for i in 1..max_size{
        let this_pile = v.into_iter().combinations(i);


        for combo in this_pile {
            let mut l_sum = 0;
            let mut this_combo :Vec<i32> = Vec::new();
            for c in combo{
                l_sum += c;
                this_combo.push(*c);
            }

            if l_sum != amt {
                continue
            }

            if can_this_split(&vec_diff(v, &this_combo), amt, ways-1) {
                return true
            }
        }
    }

    false

}

fn sol_2(split_ways:i32){
    let input = load_input("24");

    let mut sizes : Vec<i32> = Vec::new();
    let mut tot = 0;

    for l in input.split("\n"){
        let size :i32 = l.parse().unwrap();
        sizes.push(size);
        tot += size;
    }

    let aim :i32 = tot/split_ways;
    let max_len = sizes.len();
    for i in 1..max_len {
        let new_sizes = sizes.clone();
        for first_combo in new_sizes.into_iter().combinations(i){
            let mut cnt = 0;
            let mut first_pile :Vec<i32> = Vec::new();
            for fc in first_combo {
                cnt += fc;
                first_pile.push(fc);
            }

            if cnt != aim {
                continue
            }

            let remaining = vec_diff(&sizes, &first_pile);

            if can_this_split(&remaining, aim, split_ways - 1) {
                println!("min {} qe {}", first_pile.len(), vec_prod(&first_pile));
                return
            }

        }
    }

}

fn main() {
    sol_2(3);
    sol_2(4);
}
