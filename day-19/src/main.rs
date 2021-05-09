use std::fs;
use std::collections::HashMap;

use rand::thread_rng;
use rand::seq::SliceRandom;


fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

struct Changes {
    from: String,
    to: String
}

fn part_1() {

    let mut changes : Vec<Changes> = Vec::new();
    let mut orig :String = String::new();

    let mut empty = false;
    
    for l in load_input("19").split("\n"){
        if l.len() == 0 {
            empty = true;
        } else if !empty {
            let split :Vec<&str> = l.split(" => ").collect();
            
            let change = Changes {
                from: String::from(split[0]),
                to: String::from(split[1]),
            };

            changes.push(change);
        } else {
            orig = String::from(l);
        }
    }

    let mut record :HashMap<String, i32> = HashMap::new();
    let mut part_1 = 0;

    for c in changes {

        for idx in 0..orig.len(){
            let change_len = c.from.len();

            if idx + change_len > orig.len(){
                continue
            }

            let mut match_seg = true;
            for i in 0..change_len{
                
                if  orig.chars().nth(idx + i).unwrap() != c.from.chars().nth(i).unwrap(){
                    match_seg = false;
                }
            }

            if match_seg {

                let new_str = format!("{}{}{}", &orig[0..idx], c.to, &orig[idx+change_len..orig.len()]);
                //println!("Using {}-{}, at idx {} on {} gives {}", c.from, c.to, idx, orig, new_str);

                if !record.contains_key(&new_str){
                    record.insert(new_str, 1);
                    part_1 += 1;
                } else {
                    *record.get_mut(&new_str).unwrap() += 1;
                }

                
            }
        }

    }

    println!("part 1 {}", part_1);
    //println!("{:?}", record);


}



fn part_2 () {
    let mut mols = vec![String::from("e")];
    let mut record :HashMap<String, i32> = HashMap::new();

    let mut changes : Vec<Changes> = Vec::new();
    let mut current = String::new();
    let mut start = String::new();

    let mut empty = false;
    
    for l in load_input("19").split("\n"){
        if l.len() == 0 {
            empty = true;
        } else if !empty {
            let split :Vec<&str> = l.split(" => ").collect();
            
            let change = Changes {
                from: String::from(split[0]),
                to: String::from(split[1]),
            };

            changes.push(change);
        } else {
            start = String::from(l);
        }
    }

    let mut steps = 0;
    let mut finished = false;

    while !finished {
        changes.shuffle(&mut thread_rng());

        current = String::from(&start);
        let mut steps = 0;

        while current != "e" {
            let mut change_made = false;
            for c in &changes{
                for idx in 0..current.len() {
                    let mut m = true;
                    for i in 0..c.to.len(){
                        if current.chars().nth(idx + i) != c.to.chars().nth(i) {
                            m = false;
                        }
                    }
                    if m {
                        steps += 1;
                        let new_str = String::from(format!("{}{}{}", &current[0..idx], c.from, &current[idx+c.to.len() .. current.len()]));
                        current = new_str;
                        change_made = true;
                        break
                    }
                }
            }
            if !change_made {
                println!("bad set");
                break
            } else {
                println!("current : {}", current.len());
            }
        }

        if current == "e" {
            finished = true;
            println!("part 2 {}", steps);
        }
    }

}

fn main() {
    part_1();
    part_2();
}
