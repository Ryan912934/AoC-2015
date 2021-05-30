#[derive(Copy, Clone)]
struct Gear {
    cost :i32,
    damage: i32,
    armour: i32,
}

struct Player {
    health :i32,
    gear: Vec<Gear>,
    damage: i32,
    armour: i32,
}

impl Player {
    fn attack_damage(&self) -> i32{
        let mut res = self.damage;
        for g in &self.gear {
            res += g.damage;
        }
        res
    }
    fn total_armour(&self) -> i32{
        let mut r = self.armour;
        for g in &self.gear{
            r += g.armour;
        }
        r
    }

    fn print(self) {
        println!("---Player----");
        println!("health {}", self.health);
        println!("damage {}", self.damage);
        println!("armour {}", self.armour);
        for g in self.gear {
            println!("{} - {} - {}", g.cost, g.damage, g.armour);
        }
        println!("----");
    }
}

fn attack(from :&mut Player, to:&mut Player){

    let damage = from.attack_damage() - to.total_armour();
    let damage = if damage > 0 {
        damage
    } else {
        1
    };

    to.health -= damage;

}

fn win_fight(player: &mut Player, boss: &mut Player) -> bool {
    let mut player_turn:bool = true;


    while player.health > 0 && boss.health > 0 {
        if player_turn{
            attack(player, boss);
        } else {
            attack(boss, player);
        }
        player_turn = !player_turn;
    }

    player.health > 0

}

fn part_1(){

    let mut weapons :Vec<Gear> = Vec::new();
    let mut armours :Vec<Gear> = Vec::new();
    let mut rings :Vec<Gear> = Vec::new();

    weapons.push(Gear{
        cost: 8,
        damage: 4,
        armour: 0,
    });

    weapons.push(Gear{
        cost: 10,
        damage: 5,
        armour: 0,
    });

    weapons.push(Gear{
        cost: 25,
        damage: 6,
        armour: 0,
    });

    weapons.push(Gear{
        cost: 40,
        damage: 7,
        armour: 0,
    });

    weapons.push(Gear{
        cost: 74,
        damage: 8,
        armour: 0,
    });

    armours.push(Gear{
        cost: 13,
        damage: 0,
        armour: 1,
    });

    armours.push(Gear{
        cost: 31,
        damage: 0,
        armour: 2,
    });

    armours.push(Gear{
        cost: 53,
        damage: 0,
        armour: 3,
    });

    armours.push(Gear{
        cost: 75,
        damage: 0,
        armour: 4,
    });


    armours.push(Gear{
        cost: 102,
        damage: 0,
        armour: 5,
    });

    rings.push(Gear{
        cost: 25,
        damage: 1,
        armour: 0,
    });

    rings.push(Gear{
        cost: 50,
        damage: 2,
        armour: 0,
    });

    rings.push(Gear{
        cost: 100,
        damage: 3,
        armour: 0,
    });

    rings.push(Gear{
        cost: 20,
        damage: 0,
        armour: 1,
    });


    rings.push(Gear{
        cost: 40,
        damage: 0,
        armour: 2,
    });

    rings.push(Gear{
        cost: 80,
        damage: 0,
        armour: 3,
    });

    let  mut best_cost = 99999;
    let mut worst_cost = 0;

    for weapon_idx in 0..weapons.len()  {
        for armour_idx in 0..armours.len() + 1{
            for ring_1_idx in 0..rings.len() + 1 {
                for ring_2_idx in 0..rings.len() + 1{
                    if ring_1_idx == ring_2_idx {
                        continue
                    }
                    let mut boss = Player{
                        health: 103,
                        damage: 9,
                        armour: 2,
                        gear: vec![]
                    };
                    let weapon =  weapons[weapon_idx];
                    let armour = if armour_idx == armours.len() {
                        Gear{damage: 0, armour: 0, cost: 0}
                    } else {
                        armours[armour_idx]
                    };
                    let ring_1 = if ring_1_idx == rings.len() {
                        Gear{damage: 0, armour: 0, cost: 0}
                    } else {
                        rings[ring_1_idx]
                    };
                    let ring_2 = if ring_2_idx == rings.len() {
                        Gear{damage: 0, armour: 0, cost: 0}
                    } else {
                        rings[ring_2_idx]
                    };
                    
                    let mut player = Player{
                        health:100,
                        gear : vec![weapon, armour, ring_1, ring_2],
                        damage : 0,
                        armour : 0,
                    };

                    if win_fight(&mut player, &mut boss){
                        let mut cost = 0;
                        for g in &player.gear {
                            cost += g.cost;
                        }
                        if cost < best_cost {
                            best_cost = cost;
                        }
                    } else {
                        let mut cost = 0;
                        for g in &player.gear {
                            cost += g.cost;
                        }
                        if cost > worst_cost {
                            worst_cost = cost;
                        }
                    }
                }
            }


        }
    }
    println!("part 1 - {}", best_cost);
    println!("part 2 - {}", worst_cost);
}

fn main() {
    part_1();
}
