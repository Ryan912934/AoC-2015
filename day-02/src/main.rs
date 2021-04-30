use std::fs;


fn load_input(day :&str) -> String {



    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn part_1 (lines : &Vec<&str>) -> i32 {
    
    let mut total_area = 0;

    for l in lines {
        let mut dimensions : Vec<i32> = l.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
        dimensions.sort();

        let area = 3 * dimensions[0] * dimensions[1] + 2*dimensions[0] * dimensions[2] + 2 * dimensions[1] * dimensions[2];
        total_area += area

    }

    total_area
}

fn part_2 (lines : &Vec<&str>) -> i32 {

    let mut total = 0;

    for l in lines {
        let mut dimensions : Vec<i32> = l.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
        dimensions.sort();

        let area = 2 * dimensions[0] + 2 * dimensions[1] + dimensions[0] * dimensions[1] * dimensions[2];
        total += area

    }

    total

}


fn main() {
    let input = load_input("02");

    let lines = input.split("\n").collect();

    println!("Area needed : {}", part_1(&lines));

    println!("Ribbon needed : {}", part_2(&lines))
}
