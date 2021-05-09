use std::fs;

fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Sue {
    children: i32, 
    cats: i32, 
    samoyeds: i32,
    pomeranians: i32,
    akitas: i32,
    vizslas: i32,
    goldfish: i32,
    trees :i32,
    cars :i32,
    perfumes: i32,
    num : i32
}

impl Sue {
    fn sue(&self) -> bool {
        (self.children == 3 || self.children == -1) &&
        (self.cats == 7 || self.cats == -1 )&&
        (self.samoyeds == 2 || self.samoyeds == -1 )&&
        (self.pomeranians == 3 || self.pomeranians == -1 )&&
        (self.akitas == 0 || self.akitas == -1 )&&
        (self.vizslas == 0 || self.vizslas == -1 )&&
        (self.goldfish == 5 || self.goldfish == -1) &&
        (self.trees == 3 || self.trees == -1 )&&
        (self.cars == 2 || self.cars == -1 )&&
        (self.perfumes == 1 || self.perfumes == -1)
    }

    fn sue_2(&self) -> bool {
        (self.children == 3 || self.children == -1) &&
        (self.cats > 7 || self.cats == -1 )&&
        (self.samoyeds == 2 || self.samoyeds == -1 )&&
        (self.pomeranians < 3 || self.pomeranians == -1 )&&
        (self.akitas == 0 || self.akitas == -1 )&&
        (self.vizslas == 0 || self.vizslas == -1 )&&
        (self.goldfish < 5 || self.goldfish == -1) &&
        (self.trees > 3 || self.trees == -1 )&&
        (self.cars == 2 || self.cars == -1 )&&
        (self.perfumes == 1 || self.perfumes == -1)
    }
}

fn part_1(input: &str)  {

    for line in input.split("\n"){


        let split :Vec<&str> = line.split(" ").collect();
        let mut name = String::from(split[1]);
        name.pop();

        let num = name.parse::<i32>().unwrap();

        let mut children: i32 = -1; 
        let mut cats: i32 = -1;  
        let mut samoyeds: i32 = -1; 
        let mut pomeranians: i32 = -1; 
        let mut akitas: i32= -1; 
        let mut vizslas: i32 = -1; 
        let mut goldfish: i32 = -1; 
        let mut trees :i32 = -1; 
        let mut cars :i32 = -1; 
        let mut perfumes: i32 = -1; 

        for i in 0..3{
            let j = 2 + i*2;
            let c = if i == 2 {
                split[j+1].parse::<i32>().unwrap()
            } else {
                let mut temp = String::from(split[j+1]);
                temp.pop();
                temp.parse::<i32>().unwrap()
            };


            match split[j] {
                "children:" => children = c,
                "cats:" =>  cats = c, 
                "samoyeds:" =>  samoyeds = c,
                "pomeranians:" =>  pomeranians= c,
                "akitas:" => akitas = c,
                "vizslas:" => vizslas = c,
                "goldfish:" => goldfish = c,
                "trees:" => trees = c,
                "cars:" => cars = c,
                "perfumes:" => perfumes = c,
                &_ => println!("Bad match"),
            };


        }
        let sue = Sue{
            children, 
            cats, 
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
            num 
        };

        if sue.sue() {
            println!("{} is good sue", sue.num);
            println!("{:?}", sue);
        }
        
        if sue.sue_2() {
            println!("{} is good sue 2", sue.num);
            println!("{:?}", sue);
        }


    }
    
}

fn main() {
    let input = load_input("16");
    part_1(&input);
}
