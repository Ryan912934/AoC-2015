use std::fs;
use itertools::Itertools;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}


fn parse_input() -> Vec<i32> {
    let mut res :Vec<i32> = Vec::new();

    for l in load_input("17").split("\n") {
        res.push(l.parse::<i32>().unwrap());
    }

    res
}

fn sol() {


    let buckets = &parse_input();
    let target = 150;
    let mut c = 0;
    let mut lowest = 0;

    for i in 0..buckets.len(){
        println!("On len {}", i+1);
        let combos = buckets.into_iter().combinations(i+1);

        
        for combo in combos{
            let mut current_cap = 0;
            for c in combo{
                current_cap += c;
                if current_cap > target {
                    break
                }
            }
            if current_cap == target{
                if lowest == 0 {
                    lowest = i+1;
                }
                c += 1;
            }
        }      
    }

    let combos = buckets.into_iter().combinations(lowest);
    let mut p2 = 0;
    for combo in combos {
        let mut cur = 0;
        for c in combo {
            cur += c;
        } 
        if cur == target{
            p2 +=1;
        }
    }

    println!("part 1 {}", c);
    println!("part 2 {}", p2);

    

}
fn main() {
    sol();
}
