
fn run_till_goal(goal: i32) -> i32{

    let mut houses = vec![0; (goal/10) as usize];

    for i in 1..goal/10 {
        for j in (i..goal/10).step_by(i as usize){
            houses[j as usize] += i * 10;
        }
    }

    
    for i in 0..houses.len(){
        if houses[i] >= goal {
            return i as i32
        }
    }

    -1
}

fn part_2(goal: i32) -> i32{

    let mut houses = vec![0; (goal/10) as usize];

    for i in 1..goal/10 {
        //println!("");
        //println!("{}", i);
        for j in 0..50{
            let idx = i + i*j;
            if idx >= houses.len() as i32{
                //println!("out of bounds");
                break
            }
            //println!("goes into {}", idx);
            houses[idx as usize] += i*11;
        }
    }

    //println!("{:?}", houses);
    
    for i in 0..houses.len(){
        if houses[i] >= goal {
            return i as i32
        }
    }

    
    -1
}

fn main() {
    //println!("part 1 {}", run_till_goal(36000000));
    println!("part 2 {}", part_2(36000000));
}
