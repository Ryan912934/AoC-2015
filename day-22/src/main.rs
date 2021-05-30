#[derive( Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct GameState{
    player_health :i32,
    player_armour :i32,
    boss_health: i32,
    posion_turns: i32,
    shield_turns: i32,
    mana_regen_turns: i32,
    boss_attack: i32,
    mana_spend: i32,
    mana: i32,
    move_count: i32,
    last_move: String,
}

impl GameState {

    fn effect(&mut self){
        if self.posion_turns > 0 {
            self.boss_health -= 3;
            self.posion_turns -= 1;
        }
        if self.shield_turns > 0 {
            self.player_armour = 7;
            self.shield_turns -= 1;
        } else {
            self.player_armour = 0;
        }
        if self.mana_regen_turns > 0 {
            self.mana += 101;
            self.mana_regen_turns -= 1;
        }
    }

    fn boss_turn(&mut self){
        let damage = self.boss_attack - self.player_armour;
        let damage = if damage > 0 {
            damage
        } else {
            1
        };
        self.player_health -= damage
    }
}

fn all_play_turns(gs :&GameState, p2:bool) -> Vec<GameState> {

    let mut new_states :Vec<GameState> = Vec::new();

    let health_loss = if p2 {
        1
    } else 
    {
        0
    };

    if gs.mana > 52 {
        new_states.push(GameState{
            boss_health: gs.boss_health -4,
            mana: gs.mana - 53,
            mana_spend: gs.mana_spend + 53,
            move_count: gs.move_count + 1,
            last_move: String::from("mm"),
            player_health: gs.player_health - health_loss,
            ..*gs
        });
    }
 
    if gs.mana > 72 {
        new_states.push(GameState{
            boss_health: gs.boss_health -2,
            mana: gs.mana - 73,
            mana_spend: gs.mana_spend + 73,
            player_health: gs.player_health + 2 - health_loss,
            move_count: gs.move_count + 1,
            last_move: String::from("drain"),
            ..*gs
        });
    }

    if gs.mana > 112 && gs.shield_turns == 0 {
        new_states.push(GameState{
            mana: gs.mana - 113,
            mana_spend: gs.mana_spend + 113,
            shield_turns: 6,
            move_count: gs.move_count + 1,
            last_move: String::from("sheild"),
            player_health: gs.player_health - health_loss,
            ..*gs
        });
    }
    
    if gs.mana > 172 && gs.posion_turns == 0{
        new_states.push(GameState{
            mana: gs.mana - 173,
            mana_spend: gs.mana_spend + 173,
            posion_turns: 6,
            move_count: gs.move_count + 1,
            last_move: String::from("posion"),
            player_health: gs.player_health - health_loss,
            ..*gs
        });
    }

    if gs.mana > 228 && gs.mana_regen_turns == 0 {
        new_states.push(GameState{
            mana: gs.mana - 229,
            mana_spend: gs.mana_spend + 229,
            mana_regen_turns: 5,
            move_count: gs.move_count + 1,
            last_move: String::from(""),
            player_health: gs.player_health - health_loss,
            ..*gs
        });
    }

    new_states

}

fn sol(part_2:bool){
    let inital  = GameState{
        player_health :50,
        player_armour :0,
        boss_health: 58,
        posion_turns: 0,
        shield_turns: 0,
        mana_regen_turns: 0,
        boss_attack: 9,
        mana_spend: 0,
        mana: 500,
        move_count: 0,
        last_move: String::from("n/a"),
    };

    let mut all_states :Vec<GameState> = Vec::new();

    for mut i in all_play_turns(&inital, part_2){
        i.effect();
        i.boss_turn();
        all_states.push(i);
    }


    while all_states.len() > 0  {
        
        //all_states.sort_by(|a, b| a.mana_spend.cmp(&b.mana_spend));

        let low_spend = &mut all_states[0];

        low_spend.effect();

        let new_states = all_play_turns(&low_spend, part_2);
     
        for mut i in new_states {
            i.effect();

            if i.boss_health < 1 {
                println!("sol {}", i.mana_spend);
                return
            }

            i.boss_turn();
            if i.player_health > 0 {
                

                if i.boss_health < 1 {
                    println!("sol {}", i.mana_spend);
                    return
                }
                all_states.push(i);
            }

        }

 

        all_states.remove(0);
}


}

fn main() {
    sol(false);
    sol(true);
}
