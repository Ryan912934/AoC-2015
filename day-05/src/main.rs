use std::fs;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn vowals(word : &str) -> bool {

    let mut c = 0;

    for l in word.chars(){
        match l {
            'a' | 'e' | 'i' | 'o' | 'u' => c += 1,
            _ => (),
        }

        if c > 2 {
            break;
        }
    }

    c > 2

}

fn dubs(word :&str) -> bool {

    let mut res = false;

    for i in 1..word.len()  {
        if word[i..i+1] == word[i-1..i] {
            res = true;
            break
        }
    }

    res

}

fn bad_words(word : &str) -> bool {

    let bad = ["ab", "cd", "pq", "xy"];
    let mut res = false;

    for b in bad.iter()  {
        if word.contains(b) {
            res = true;
            break
        }
    }

    res

}

fn part_1(input : &str) -> i32 {

    let words = input.split("\n");

    let mut ans = 0;

    for word in words {

        if vowals(&word) && dubs(&word) && !bad_words(&word){
            ans += 1;
        }


    }

    ans

}

fn double_pair(word : &str) -> bool {


    let mut res = false;

    for i in 0..word.len()-1{
        
        let pair = &word[i..i+2];
        let word_without_pair = &word[i+2 ..];
        if word_without_pair.contains(pair){
            res = true;
            println!("Word: {} Pair: {} Word without : {}", &word, &pair, & word_without_pair);
            break
        }

    }


    res
}

fn pair_with_gap(word: &str) -> bool {

    let mut res = false;

    for i in 2..word.len(){
        if word[i-2 .. i-1] == word[i .. i+1]{
            //println!("{} has pair at {} and {}", &word, i-2, i);
            res = true;
            break;
        }
    }  

    res

}


fn part_2(input : &str) -> i32 {

    let words = input.split("\n");

    let mut ans = 0;

    for word in words {

        if double_pair(&word) && pair_with_gap(&word) {
            ans += 1;
        }


    }

    ans

}

fn main() {
    let input = load_input("05");

    println!("Part 1 {}", part_1(&input));

    assert!(double_pair("qjhvhtzxzqqjkmpb"));
    assert!(pair_with_gap("qjhvhtzxzqqjkmpb"));
    assert!(double_pair("xxyxx"));
    assert!(pair_with_gap("xxyxx"));
    assert!(double_pair("uurcxstgmygtbstg"));
    assert!(!pair_with_gap("uurcxstgmygtbstg"));
    assert!(!double_pair("ieodomkazucvgmuy"));
    assert!(pair_with_gap("ieodomkazucvgmuy"));

    println!("Part 2 {}", part_2(&input))


}
