use std::fs;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn run_till_done(reg_a_start: i32) {
    let input = load_input("23");

    let comp :Vec<&str> = input.split("\n").collect();

    let mut idx = 0;
    let mut reg_a = reg_a_start as i64;
    let mut reg_b :i64 = 0; 

    while idx < comp.len() {

        let l : Vec<&str> = comp[idx].split(" ").collect();
        let mut amt :i32 = 1;

        if l[0] == "hlf"{
            if l[1] == "a"{
                reg_a = reg_a / 2;
            } else {
                reg_b = reg_b / 2;
            }
        } else if l[0] == "tpl" {
            if l[1] == "a"{
                reg_a *= 3;
            } else {
                reg_b *= 3;
            }
        } else if l[0] == "inc" {
            if l[1] == "a"{
                reg_a += 1;
            } else {
                reg_b += 1;
            }
        } else if l[0] == "jio" {
            let a = if l[1] == "a," {
                true
            } else {
                false
            };
            if a && reg_a == 1 {
                amt = l[2].parse().unwrap();
            } else if !a && reg_b == 1 {
                amt = l[2].parse().unwrap();
            }

        } else if l[0] == "jie" {
            let a = if l[1] == "a," {
                true
            } else {
                false
            };
            if a && reg_a % 2 == 0 {
                amt = l[2].parse().unwrap();
            } else if !a && reg_b % 2 == 0 {
                amt = l[2].parse().unwrap();
            }
        } else if l[0] == "jmp" {
             amt = l[1].parse().unwrap();
            
        }

        idx = (idx as i32 + amt) as usize;

    }
    println!("a {} b {}", reg_a, reg_b);
}

fn main() {
    run_till_done(0);
    run_till_done(1);
}
