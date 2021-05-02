use std::fs;
use std::collections::HashMap;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}



fn get_value(input :&str, wire:&str, map: &mut HashMap<String, i32> ) -> i32 {

    if map.contains_key(wire){
        return *map.get(wire).unwrap();
    }


    if wire.chars().all(char::is_numeric) {
        return wire.parse::<i32>().unwrap();
    }

    let mut res :i32 = -1;
    let lines = input.split("\n");

    for line in lines {

        let split : Vec<&str> = line.split(" -> ").collect();
        let left = split[0];
        let dest = split[1];

        let left_split : Vec<&str>= left.split(" ").collect();

        if dest != wire {
            continue;
        }

        if left.contains("AND"){
            res = get_value(input, &left_split[0], map) & get_value(input, &left_split[2], map);
        } else if left.contains("OR") {
            res = get_value(input, &left_split[0], map) | get_value(input, &left_split[2], map);
        } else if left.contains("RSHIFT") {
            res = get_value(input, &left_split[0], map) >>  get_value(input, &left_split[2], map);
        } else if left.contains("LSHIFT"){
            res = get_value(input, &left_split[0], map) <<  get_value(input, &left_split[2], map);
        } else if left.contains("NOT"){
            res = !get_value(input, &left_split[1], map);
        } else {
            if left.chars().all(char::is_numeric) {
                res = left.parse::<i32>().unwrap();
            } else {
                res = get_value(input, &left, map)
            }
        }
        break

    }
    
    map.insert(String::from(wire), res);

    res
}

fn part_1(input: &str) -> i32 {

    let mut map :HashMap<String, i32> = HashMap::new();

    get_value(input, "a", &mut map)
}


fn part_2(input: &mut str, overide:i32) -> i32 {

    let mut map :HashMap<String, i32> = HashMap::new();
    let b_line = format!("{} -> b", overide);

    get_value(&input.replace("14146 -> b", &b_line), "a", &mut map)
}


fn main() {
    let mut input = load_input("07");

    let part_1 = part_1(&input);

    println!("Part 1:{}", part_1);
    println!("Part 2:{}", part_2(&mut input, part_1));


}
