use std::fs;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

struct Cord {
    x :i32,
    y :i32,
} 

impl Cord {
    fn equal(&self, cord2: &Cord) -> bool {
        self.x == cord2.x && self.y == cord2.y
    }

    fn in_vec(&self, vec : &Vec<Cord>) -> bool{
        let mut res:bool = false;

        for c in vec {
            if c.equal(self){
                res = true;
                break;
            }
        }

        res
    }
}

fn part_01 (input : &str) -> i32 {
    

    let mut x = 0;
    let mut y = 0;
    let mut house_count = 1;
    let mut cords : Vec<Cord> = Vec::new();

    let start_cord = Cord{
        x: 0,
        y:0
    };

    cords.push(start_cord);

    for i in input.chars() {
        if i == '^' {
            y +=1
        } else if i == 'v' {
            y -= 1
        } else if i == '>' {
            x += 1
        } else {
            x -= 1 
        }

        let current_cord = Cord{
            x: x,
            y: y,
            };

        if !current_cord.in_vec(&cords){
            house_count += 1;
            cords.push(current_cord);
        } 
    }


    house_count
}

fn part_02 (input : &str) -> i32 {
    let mut xa = 0;
    let mut ya = 0;
    let mut xb = 0;
    let mut yb = 0;
    let mut house_count = 1;
    let mut cords : Vec<Cord> = Vec::new();

    let start_cord = Cord{
        x: 0,
        y:0
    };


    cords.push(start_cord);

    let mut a =  true;

    for i in input.chars() {

        a = !a;
        let current_cord: Cord;

        if a {
            if i == '^' {
                ya +=1
            } else if i == 'v' {
                ya -= 1
            } else if i == '>' {
                xa += 1
            } else {
                xa -= 1 
            }
            current_cord = Cord{
                x: xa,
                y: ya,
            };
        } else {
            if i == '^' {
                yb +=1
            } else if i == 'v' {
                yb -= 1
            } else if i == '>' {
                xb += 1
            } else {
                xb -= 1 
            }
            current_cord = Cord{
                x: xb,
                y: yb,
            };
        }

        if !current_cord.in_vec(&cords){
            house_count += 1;
            cords.push(current_cord);
        } 
    }


    house_count
}


fn main() {
    let input = load_input("03");

    println!("Part 1: {}", part_01(&input));
    println!("Part 2: {}", part_02(&input));
}
