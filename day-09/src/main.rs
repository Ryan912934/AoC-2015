use std::fs;
use std::collections::HashMap;
use itertools::Itertools;


fn load_input(day :&str) -> String {
    fs::read_to_string(format!("/home/ryan/Documents/AoC/2015/inputs/day{}.txt", day))
        .expect("Something went wrong reading the file")
}


fn load_routes(input :&str) -> HashMap<String, i32> {

    let mut res = HashMap::new();

    let lines = input.split("\n");

    for line in lines{

        let split :Vec<&str> = line.split(" = ").collect();
        let dist = split[1].parse::<i32>().unwrap();

        let city : Vec<&str>= split[0].split(" to ").collect();

        let key = format!("{}:{}", city[0], city[1]);

        res.insert(key, dist);
    }

    res


}

fn get_cities(input:&str) -> Vec<&str> {

    let mut res :  Vec<&str> = Vec::new();

    let lines = input.split("\n");

    for line in lines{

        let split :Vec<&str> = line.split(" = ").collect();

        let city : Vec<&str>= split[0].split(" to ").collect();

        for c in city{
            if !res.iter().any(|x| x == &c){
                res.push(c);
            }
        }

    }

    res

}

fn get_dist(from :&str, to:&str, map:&HashMap<String, i32>) -> i32{

    let key_a = format!("{}:{}", from, to);
    let key_b = format!("{}:{}", to, from);

    if map.contains_key(&key_a){
        return *map.get(&key_a).unwrap()
    } else if map.contains_key(&key_b){
        return *map.get(&key_b).unwrap()
    } else {
        return 99999
    }
}

fn get_dist_2(from :&str, to:&str, map:&HashMap<String, i32>) -> i32{

    let key_a = format!("{}:{}", from, to);
    let key_b = format!("{}:{}", to, from);

    if map.contains_key(&key_a){
        return *map.get(&key_a).unwrap()
    } else if map.contains_key(&key_b){
        return *map.get(&key_b).unwrap()
    } else {
        return 0
    }
}


fn part_1(cities: &Vec<&str>, map: &HashMap<String, i32> ) -> i32{

    let routes = cities.into_iter().permutations(cities.len());

    let mut best = 999999999;
    let mut best_route : Vec<&&str> = Vec::new();

    for route in routes{

        let mut cost = 0;

        for i in 0..route.len()-1 {
            cost += get_dist(route[i], route[i+1], map);
        }

        if cost < best {
            best = cost;
            best_route = route;
        }



    }

    println!("For part 1 the best dist is {} with {:?}", best, best_route);

    best
}

fn part_2(cities: &Vec<&str>, map: &HashMap<String, i32> ) -> i32{

    let routes = cities.into_iter().permutations(cities.len());

    let mut worst = 0;
    let mut worst_route : Vec<&&str> = Vec::new();

    for route in routes{

        let mut cost = 0;

        for i in 0..route.len()-1 {
            cost += get_dist_2(route[i], route[i+1], map);
        }

        if cost > worst {
            worst = cost;
            worst_route = route;
        }



    }

    println!("For part 2 the best dist is {} with {:?}", worst, worst_route);

    worst
}

fn main() {
    let input = load_input("09");
    let cities = get_cities(&input);

    let routes = load_routes(&input);

    println!("{:?}", routes);
    println!("{:?}", cities);

    println!("Part 1: {}", part_1(&cities, &routes));
    println!("Part 2: {}", part_2(&cities, &routes));
}
