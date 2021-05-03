use std::fs;

extern crate enquote;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn part_1(input :&str) -> i32 {
    let mut res = 0;


    let lines = input.split("\n");

    for line in lines {

        let l = line.trim();
        let code_len = l.len();

        let rendered = enquote::unquote(l).unwrap();
        let ren_len = &rendered.chars().count();
        println!("{} ({}): {} ({})", l,code_len, &rendered, ren_len);

        res += code_len - ren_len;

    }

    res as i32
}

fn part_2(input :&str) -> i32 {
    let mut res = 0;


    let lines = input.split("\n");

    for line in lines {

        let l = line.trim();
        let code_len = l.len();

        let new = str::replace(l, "\\", "\\\\");
        let new = str::replace(&new, "\"", "\\\"");
        let new = format!("{}{}{}", "\"", new, "\"");

        let ren_len = &new.chars().count();
        println!("{} ({}): {} ({})", l,code_len, &new, ren_len);

        res += ren_len - code_len ;

    }

    res as i32
}


fn main() {
    let input = load_input("08");

    println!("Part 1: {}", part_1(&input));
    println!();
    println!("Part 2: {}", part_2(&input));
}
