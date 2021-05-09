use std::fs;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

struct Ingredient {
    name: String, 
    cap: i32,
    dur: i32,
    flav: i32,
    tex: i32,
    cal: i32
}


fn parse_int_with_comma(input :&str) -> i32 {
    let mut s = String::from(input);
    s.pop();
    let r = s.parse::<i32>().unwrap();

    r
}

fn get_ingredients(input: &str) -> Vec<Ingredient> {
    let mut res :Vec<Ingredient> = Vec::new();

    for line in input.split("\n"){
        let split :Vec<&str> = line.split(" ").collect();

        let name = String::from(split[0]);
        let cap = parse_int_with_comma(split[2]);
        let dur = parse_int_with_comma(split[4]);
        let flav = parse_int_with_comma(split[6]);
        let tex = parse_int_with_comma(split[8]);
        let cal = split[10].parse::<i32>().unwrap();

        let inged = Ingredient{
            name,
            cap, 
            dur,
            flav, 
            tex,
            cal
        };

        res.push(inged);
    }

    res
}

fn score_part_1(list: &Vec<Ingredient>, i:i32, j:i32, k:i32, l:i32) -> i32 {
    let cap = list[0].cap * i + list[1].cap * j + list[2].cap * k + list[3].cap * l; 
    let dur = list[0].dur * i + list[1].dur * j + list[2].dur * k + list[3].dur * l; 
    let flav = list[0].flav * i + list[1].flav * j + list[2].flav * k + list[3].flav * l; 
    let tex = list[0].tex * i + list[1].tex * j + list[2].tex * k + list[3].tex * l; 

    let cap = if cap > 0{
        cap
    } else {
        0
    };

    let dur = if dur > 0 {
        dur
    } else {
        0
    };

    let flav = if flav > 0 {
        flav
    } else {
        0
    };

    let tex =if tex > 0 {
        tex
    } else {
        0
    };

    cap * dur * flav * tex
}

fn cal_500(list: &Vec<Ingredient>, i:i32, j:i32, k:i32, l:i32) -> bool{
    list[0].cal * i + list[1].cal * j + list[2].cal * k + list[3].cal * l == 500
}

fn part_1(list :&Vec<Ingredient>) -> i32 {

    //lazy approach
    let mut best = 0;

    for i in 1..101 {
        for j in 1..101{
            for k in 1..101{
                for l in 1..101{
                    if i + j + k + l == 100{
                        let score = score_part_1(&list, i, j, k, l);
                        if score > best {
                            best = score;
                        }
                    }
                }
            }
        }
    }

    best
}


fn part_2(list :&Vec<Ingredient>) -> i32 {

    //lazy approach
    let mut best = 0;

    for i in 1..101 {
        for j in 1..101{
            for k in 1..101{
                for l in 1..101{
                    if i + j + k + l == 100 && cal_500(&list, i, j, k, l){
                        let score = score_part_1(&list, i, j, k, l);
                        if score > best {
                            best = score;
                        }
                    }
                }
            }
        }
    }

    best
}

fn main() {
    let input = load_input("15");
    let list = get_ingredients(&input);

    println!("Part 1 {}", part_1(&list));
    println!("Part 2 {}", part_2(&list));

}
