use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn load_map(input :&str) -> HashMap<String, i32> {

    let lines = input.split("\n");
    let mut res : HashMap<String, i32> = HashMap::new();

    for line in lines{
        let split:Vec<&str> = line.split(" ").collect();
        let name_1 = split[0];
        let mut name_2 = String::from(split[split.len() - 1]);
        name_2.pop();

        let name = format!("{}:{}", name_1,name_2);
        let gain = if split[2] == "gain" {
            true
        } else {
            false
        };
        let amount = split[3].parse::<i32>().unwrap();

        let amount = if gain {
            amount
        } else {
            -amount
        };

        res.insert(name, amount);
    }

    res

}

fn load_names(input: &str) -> Vec<String> {
    let mut res :Vec<String> = Vec::new();
    for line in input.split("\n"){
        let split :Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
        if !res.contains(&split[0]){
            res.push(String::from(&split[0]));
        }
    }
    res
}

fn part_1(names: &Vec<String>, map:&HashMap<String, i32>) -> i32 {

    let mut best = -99999;
    let name_len = names.len();
    let combo = names.into_iter().permutations(name_len);

    for c in combo{
        let mut score = 0;

        for idx in 0..c.len() {

            //below
            let key_1 = if idx == 0 {
                format!("{}:{}", c[idx], c[c.len()-1])
            } else {
                format!("{}:{}", c[idx], c[idx - 1])
            };

            //above
            let key_2 = if idx == c.len() - 1 {
                format!("{}:{}", c[idx], c[0])
            } else {
                format!("{}:{}", c[idx], c[idx + 1])
            };

            //println!("1: {:?} 2: {:?}", map.get(&key_1),  map.get(&key_2));
            score += map.get(&key_1).unwrap() + map.get(&key_2).unwrap();

        }

        if score > best {
            best = score;
        }
    }


    best

}

fn add_me_to_map(map: &HashMap<String, i32>, names:&Vec<String>) -> HashMap<String, i32> {
    let mut res :HashMap<String, i32> = map.clone();

    for name in names{
        let key_1 = format!("Me:{}", name);
        let key_2 = format!("{}:Me", name);
        
        res.insert(key_1, 0);
        res.insert(key_2, 0);
    }

    res
}

fn main() {
    let input = load_input("13");

    let map = load_map(&input);
    let names = load_names(&input);

    let map_2 = add_me_to_map(&map, &names);
    let mut names_2 = names.clone();
    names_2.push(String::from("Me"));

    //println!("Map {:?}", map);
    println!("");

    println!("Part 1: {}", part_1(&names, &map));
    println!("Part 2: {}", part_1(&names_2, &map_2));

    

}
