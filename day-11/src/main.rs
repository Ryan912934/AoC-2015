
fn increase_pass(pass :&str) -> String {

    let mut pass :Vec<char> = pass.chars().collect();

    let mut idx = 0;

    loop{
        let rev_i = pass.len() - 1 - idx;

        if pass[rev_i] == 'z' {
            pass[rev_i] = 'a';
            idx += 1;
        } else {
            let new_char = std::char::from_u32(pass[rev_i] as u32 + 1).unwrap();
            pass[rev_i] = new_char;
            break;
        }
    }

    pass.into_iter().collect()
}

fn valid_pass(pass :&str) -> bool {

    let pass_chars :Vec<char> = pass.chars().collect();

    //increasing abc or def etc
    let mut increasing = false;

    for i in 0..pass_chars.len() - 3{

        let first = std::char::from_u32(pass_chars[i] as u32 ).unwrap();
        let secound = std::char::from_u32(pass_chars[i+1] as u32 -1).unwrap();
        let third = std::char::from_u32(pass_chars[i+2] as u32 -2).unwrap();
        if first == secound && secound == third {
            increasing = true;
            break
        }
    }


    //no i o l
    let mut bad_letters = false;
    if pass.contains("i") || pass.contains("o") || pass.contains("l") {
        bad_letters = true;
    }


    //atleast 2 dubs 
    let mut dub_dub = false;
    let mut m = String::from("-1");
    for i in 0..pass_chars.len()-1{
        if pass_chars[i] == pass_chars[i+1] {
            if m == "-1" {
                m = String::from(pass_chars[i]);
            } else if m != String::from(pass_chars[i+1]){
                dub_dub = true;
                break;
            }
        }
    }

    //println!("For pass {} we get {}-{}-{}", pass, increasing, !bad_letters, dub_dub);

    increasing && !bad_letters && dub_dub
}

fn part_1(start: &str) -> String {

    let mut pass = increase_pass(start);

    while !valid_pass(&pass){
        pass = increase_pass(&pass);
    }

    pass
}

fn main() {

    println!("abcdffaa is {}", valid_pass("abcdffaa"));
    println!("ghjaabcc is {}", valid_pass("ghjaabcc"));

    println!("abc increases to {}", increase_pass("abc"));
    println!("abcz increases to {}", increase_pass("abcz"));
    
    let pass_a = part_1("cqjxjnds");
    println!("Part 1: {}", pass_a);
    println!("Part 2: {}", part_1(&pass_a));
}
