use std::fs;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn part_1(input :&str) ->i32 {

    let mut arr = [[0 ; 1000]; 1000];

    let lines = input.split("\n");

    for line in lines{

        let turn_on  = if &line[6..7]=="n" {
            1
        } else {
            0
        };

        let toggle = if &line[1..2] == "o" {
            true
        } else {
            false
        };

        let pre_words = if toggle {
            1
        } else {
            2
        };

        let splits : Vec<&str> = line.split(" ").collect();
        let start_cord : Vec<u32> = splits[pre_words].split(",")
                            .map(|x| { x.parse::<u32>().unwrap()}).collect();
        let end_cord : Vec<u32> = splits[pre_words + 2].split(",")
                             .map(|x| x.parse::<u32>().unwrap()).collect();

        for i in start_cord[0]..end_cord[0]+1 {
            for j in start_cord[1]..end_cord[1]+1{
                if toggle {
                    arr[i as usize][j as usize] = if arr[i as usize][j as usize] == 1 {
                        0
                    } else {
                        1
                    };
                } else {
                    arr[i as usize][j as usize] = turn_on;
                }
            }
        }
    }

    let mut ans = 0;

    for i in &arr {
        for j in i {
            ans += j;
        }
    }  ans

}

fn part_2(input :&str) ->i32 {

    let mut arr = [[0 ; 1000]; 1000];

    let lines = input.split("\n");

    for line in lines{

        let turn_on  = if &line[6..7]=="n" {
            1
        } else {
            0
        };

        let toggle = if &line[1..2] == "o" {
            true
        } else {
            false
        };

        let pre_words = if toggle {
            1
        } else {
            2
        };

        let splits : Vec<&str> = line.split(" ").collect();
        let start_cord : Vec<u32> = splits[pre_words].split(",")
                            .map(|x| { x.parse::<u32>().unwrap()}).collect();
        let end_cord : Vec<u32> = splits[pre_words + 2].split(",")
                             .map(|x| x.parse::<u32>().unwrap()).collect();

        for i in start_cord[0]..end_cord[0]+1 {
            for j in start_cord[1]..end_cord[1]+1{
                if toggle {
                    arr[i as usize][j as usize] += 2
                } else if turn_on == 1{
                    arr[i as usize][j as usize] += 1;
                } else if arr[i as usize][j as usize] > 0 {
                    arr[i as usize][j as usize] -= 1;
                }
            }
        }
    }

    let mut ans = 0;

    for i in &arr {
        for j in i {
            ans += j;
        }
    }  ans

}

fn main() {
    let input = load_input("06");

    println!("Part 1 : {}", part_1(&input));
    println!("Part 2 : {}", part_2(&input));

}
