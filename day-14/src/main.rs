use std::fs;
use std::str::FromStr;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    run_time: i32,
    rest_time:i32,
    count: i32, 
    running: bool,
    dist: i32, 
    points: i32
}

impl Reindeer {

    fn progress(&mut self){
        if self.running {
            self.dist += self.speed;
            self.count += 1;

            if self.count == self.run_time {
                self.running = false;
                self.count = 0;
            }
        } else {
            self.count += 1;
            if self.count == self.rest_time {
                self.running = true;
                self.count = 0;
            }
        }
    }

    fn award(&mut self, best_dist: i32){
        if self.dist == best_dist {
            self.points += 1;
        }
    }

}

fn make_list(input :&str) -> Vec<Reindeer> {
    let mut res : Vec<Reindeer> = Vec::new();

    for line in input.split("\n"){
        let split :Vec<&str> = line.split(" ").collect();

  
        let reindeer = Reindeer{
            name: String::from(split[0]),
            speed: FromStr::from_str(split[3]).unwrap(),
            run_time: FromStr::from_str(split[6]).unwrap(),
            rest_time: FromStr::from_str(split[13]).unwrap(),
            count: 0,
            running: true,
            dist: 0,
            points: 0
        };
        res.push(reindeer);
    }

    res
}

fn run(input: &str, time:i32) -> i32 {


    let mut list = make_list(&input);
    println!("{:?}", list);

    for _ in 0..time{
        for r in &mut list{
            r.progress();
        }
    }

    let mut best = 0;

    for r in list{
        let name = r.name;
        let dist = r.dist;
        println!("{} ran {}", name, dist);
        if dist > best {
            best = dist;
        }

    }

    best

}


fn points(input: &str, time:i32) -> i32 {


    let mut list = make_list(&input);
    println!("{:?}", list);

    for _ in 0..time{
        let mut best = 0;
        for r in &mut list{
            r.progress();
            if r.dist > best{
                best = r.dist
            }
        }
        for r in &mut list{
            r.award(best);
        }
    }

    let mut best = 0;

    for r in list{
        let name = r.name;
        let points = r.points;
        println!("{} scored {}", name, points);
        if points > best {
            best = points;
        }

    }

    best

}

fn main() {

    let input = load_input("14");
    

    println!("part 1 {}", run(&input, 2503));
    println!("part 2 {}", points(&input, 2503));
}
