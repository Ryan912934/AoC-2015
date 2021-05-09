use std::fs;
use regex::Regex;
use serde_json::Value;


fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn part_1(input :&str) -> i32 {
    let mut res = 0;
    let re = Regex::new(r"-?\d*").unwrap();

    let matches = re.find_iter(input);

    let m :Vec<&str>=matches.map(|x| x.as_str()).collect();

    for i in m {
        if i.len() > 0 && i != "-" {
            //println!("{}", i);
            res += i.parse::<i32>().unwrap();
        }
    }

    res


}

//had to look up https://github.com/alokmenghrajani/adventofcode/blob/master/src/year_2015/day12.rs :()
fn sum(v: Value) -> i64 {
    return match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(v) => v.into_iter().map(|e| sum(e)).sum(),
        Value::Object(ref v) => {
            let mut max = 0;
            for v in v.values() {
                if v == "red" {
                    return 0;
                }
                max += sum(v.clone());
            }
            return max;
        }
    };
}

fn part_2(input:&str) -> i64{

    let json: serde_json::Value = serde_json::from_str(input).unwrap();
        
    sum(json)
    
}
fn main() {
    let input = load_input("12");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
