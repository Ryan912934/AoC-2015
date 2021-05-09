extern crate num;

fn play_round(input: &str) -> String {

    let s : Vec<char> = input.to_string().chars().collect();
    let mut i = 0;
    let mut res = String::from("");

    if s.len() == 1 {
        return String::from(format!("1{}", s[0]))
    }

    loop{
        let mut count = 1;
        let same = true;
        while same {
            if s[i] == s[i+count] {
                count += 1;
                if i + count >= s.len(){
                    break
                }
            } else {
                break
            }
        }



        res = format!("{}{}{}", res, count, s[i]);

        i = i + count;

        if i == s.len() - 1 {
            res = format!("{}{}{}", res, 1, s[i]);
            break
        }

        if i >= s.len(){
            break
        }



    }

    res
    

}

fn part_1(input: &str) -> i32 {

    let mut res = String::from(input);
    let mut c = 0;
    for i in 0..50{
        c += 1;
        
        res = play_round(&res);

        let a :Vec<char> = res.chars().collect();
        println!("Loop Num {} - Len {}", c, a.len());
        //println!("{}", res)
    }   

    let r :Vec<char> = res.chars().collect();
    r.len() as i32

}




fn main() {

    //println!("Test : {}", play_round("111221".parse::<BigInt>().unwrap()));

    println!("part 1: {}", part_1("1113222113"));
}
