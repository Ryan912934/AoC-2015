use std::fs;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}



fn part_1(input : &str) -> i32 {

    let mut hasher = Md5::new();

    let mut ans : i32 = 0;

    loop {

        let mut to_hash = String::from(input);
        to_hash.push_str(&ans.to_string());
        hasher.input(to_hash.as_bytes());

        let mut output = [0; 16]; 
        hasher.result(&mut output);

        let mut res = String::from("");

        for i in 0..3 {
            res.push_str(&format!("{:02x}", output[i]));
        }

        let chars = String::from(&res[0..5]);

        if chars == "00000" {
            break
        }

        ans += 1;

        hasher.reset();


    }

    println!("Ans for {} is {}", &input, ans);

    ans

}

fn part_2(input : &str) -> i32 {

    let mut hasher = Md5::new();

    let mut ans : i32 = 0;

    loop {

        let mut to_hash = String::from(input);
        to_hash.push_str(&ans.to_string());
        hasher.input(to_hash.as_bytes());

        let mut output = [0; 16]; 
        hasher.result(&mut output);

        let mut res = String::from("");

        for i in 0..3 {
            res.push_str(&format!("{:02x}", output[i]));
        }

        if res == "000000" {
            break
        }

        ans += 1;

        hasher.reset();


    }

    println!("Ans for {} is {}", &input, ans);

    ans

}


fn main() {

    let input = load_input("04");

    assert_eq!(part_1("abcdef"),609043);
    assert_eq!(part_1("pqrstuv"),1048970);

    println!("Part 1: {}", part_1(&input));

    println!("Part 2: {}", part_2(&input));

}
