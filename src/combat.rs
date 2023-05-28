use std::{thread, time::Duration};

use rand::random;
use text_io::read;

use crate::{
    combat::monsters::{rat::Rat, tree_spirit::TreeSpirit},
    game::{self, ACTION, PLAYER},
    io_manager::clear_screen,
    item::{Item, Material},
    player::{Action, Player},
};

use self::monsters::{giant_rat::GiantRat, slime::Slime, MonsterData};

pub mod ability;
pub mod monsters;

// TODO: Better state but this violates good OOP
pub struct FightState<'a> {
    pub player: &'a mut Player,
    pub health: i32,
    pub max_health: i32,
    pub adrenaline: f32,
    pub combat_style: &'a str,
    pub pl_crit_chance: f32,
    pub monster: &'a Box<dyn MonsterData>,
    pub monster_level: u64,
    pub monster_attack: u64,
    pub monster_health: i32,
    pub monster_max_health: i32,
    pub monster_adrenaline: f32,
    pub monster_crit_chance: f32,
}

fn print_status(state: &mut FightState) {
    println!(
        "BATTLE BETWEEN {} (LVL {}) AND YOU (LVL {})",
        state.monster.get_name(),
        state.monster_level as u32,
        state.player.get_combat_level() as u32
    );
    println!("========================================\n");

    println!(
        "> {}: {} / {}",
        state.monster.get_name(),
        state.monster_health,
        state.monster_max_health
    );
    println!("  - Adrenaline: {:.2}%\n", state.monster_adrenaline * 100.0);

    println!("> You: {} / {}", state.health, state.max_health);
    println!("  - Adrenaline: {:.2}%", state.adrenaline * 100.0);

    println!("\n========================================");
}

fn player_attack(state: &mut FightState) {
    // roll for damage
    let max_hit = max_hit_comp(
        state.player.get_bonus(&state.combat_style.to_string()),
        state.monster.get_defense(),
    );
    let damage = (max_hit as f32 * random::<f32>()) as i32;
    if random::<f32>() < state.pl_crit_chance {
        state.monster_health -= damage * 2;
        println!(
            "You critically hit the {} for {} {} damage.",
            state.monster.get_name(),
            damage * 2,
            state.combat_style
        );
    } else {
        state.monster_health -= damage;
        println!(
            "You hit the {} for {} {} damage.",
            state.monster.get_name(),
            damage,
            state.combat_style
        );
    }

    // add adrenaline
    let mut adr = (damage as f32 / max_hit as f32) / 10.0;
    adr *= 2.0f32.powf(1.0 - state.health as f32 / state.max_health as f32);
    state.adrenaline += adr;
    state.adrenaline = state.adrenaline.min(1.0);
}

fn enemy_attack(state: &mut FightState) {
    if state.monster.choose_ability(state) {
        return;
    }

    // roll for damage
    let max_hit = max_hit_comp(
        state.monster_attack,
        state.player.get_bonus(&"defense".to_string()),
    );
    let dmg = (max_hit as f32 * random::<f32>()) as i32;
    if random::<f32>() < state.monster_crit_chance {
        state.health -= dmg * 2;
        println!(
            "The {} critically hit you for {} {} damage.",
            state.monster.get_name(),
            dmg * 2,
            state.monster.get_attack_style(),
        );
    } else {
        state.health -= dmg;
        println!(
            "The {} hit you for {} {} damage.",
            state.monster.get_name(),
            dmg,
            state.monster.get_attack_style(),
        );
    }

    // add adrenaline
    let mut adr = (dmg as f32 / max_hit as f32) / 10.0;
    adr *= 2.0f32.powf(1.0 - state.monster_health as f32 / state.monster_max_health as f32);
    state.monster_adrenaline += adr;
    state.monster_adrenaline = state.monster_adrenaline.min(1.0);
}

fn eat_menu(state: &mut FightState) {
    if state.player.get_inventory().get_items().len() == 0 {
        println!("You have no items to use.");
        thread::sleep(Duration::from_millis(
            (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
        ));
        return;
    }
    let mut input: usize = 1;

    while input != 0 {
        clear_screen();

        print_status(state);

        println!("\nWhat item would you like to use?");
        let mut index = 1;
        let mut food = Vec::<(Item, usize)>::new();
        for (i, item) in state.player.get_inventory().get_items().iter().enumerate() {
            if item.get_material().get_food_heal() > 0 {
                println!(
                    "{}. {} (+{} hitpoints) (left: {})",
                    index,
                    item.get_material().get_name(),
                    item.get_material().get_food_heal(),
                    item.get_quantity()
                );
                food.push((item.clone(), i));
                index += 1;
            }
        }
        println!("{}. Cancel", index);

        print!("> ");

        input = read!();

        if input == index {
            break;
        }

        if input < 1 || input > index {
            println!(
                "Invalid input. Please enter a number between 1 and {}.",
                index
            );
            thread::sleep(Duration::from_millis(
                (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
            ));
            continue;
        }

        let item = &food[input - 1];

        if item.0.get_material().get_food_heal() == 0 {
            println!("You cannot eat that item.");
            thread::sleep(Duration::from_millis(
                (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
            ));
            continue;
        } else {
            let heal = item.0.get_material().get_food_heal() as i32;
            state.health += heal;
            state.player.get_inventory_mut().remove_quantity(item.1, 1);
            println!(
                "You eat the {} and heal {} health.",
                item.0.get_material().get_name(),
                heal
            );
            thread::sleep(Duration::from_millis(
                (1000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
            ));

            enemy_attack(state);

            thread::sleep(Duration::from_millis(
                (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
            ));
            break;
        }
    }
}

fn ability_menu(state: &mut FightState) {
    let mut input: usize = 1;

    while input != 0 {
        clear_screen();

        print_status(state);

        println!("\nWhat ability would you like to use?");
        let mut index = 1;
        let mut abilities = Vec::new();

        // print ability list
        for ability in state.player.get_abilities().iter() {
            if state.combat_style != ability.get_combat_style() {
                continue;
            }
            if state.player.get_level(ability.get_combat_style()) < ability.get_level() {
                println!(
                    "{}. {} ({:.2}% adrenaline) - {} - (requires lv. {} {})",
                    index,
                    ability.get_name(),
                    ability.get_cost() * 100.0,
                    ability.get_description(),
                    ability.get_level(),
                    ability.get_combat_style(),
                );
                index += 1;
            } else {
                println!(
                    "{}. {} ({:.2}% adrenaline) - {}",
                    index,
                    ability.get_name(),
                    ability.get_cost() * 100.0,
                    ability.get_description()
                );
            }
            index += 1;
            abilities.push(ability)
        }
        println!("{}. Cancel", index);

        print!("> ");

        input = read!();

        if input < 1 || input > index {
            println!(
                "Invalid input. Please enter a number between 1 and {}.",
                index
            );
            thread::sleep(Duration::from_millis(
                (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
            ));
            continue;
        }

        if input == index {
            break;
        }

        let ability = abilities[input - 1];

        if state.adrenaline < ability.get_cost() {
            println!("You do not have enough adrenaline to use that ability.");
            thread::sleep(Duration::from_millis(
                (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
            ));
            continue;
        }

        state.adrenaline -= ability.get_cost();

        (ability.activate)(state);

        thread::sleep(Duration::from_millis(
            (1000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
        ));

        enemy_attack(state);

        thread::sleep(Duration::from_millis(
            (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
        ));

        break;
    }
}

fn fight(monster: &Box<dyn MonsterData>) {
    clear_screen();

    let mut player = PLAYER.lock().unwrap();
    let can_fight: String = monster.can_fight(&player);
    if can_fight != "" {
        println!("{}", can_fight);
        thread::sleep(Duration::from_millis(
            (3000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
        ));
        return;
    }

    let health: i32 = player.get_health();
    let max_health: i32 = (player.get_level(&"hitpoints".to_string()) * 100) as i32;
    let pl_crit_chance: f32 = 0.15;
    let mut combat_style: &str = "melee";
    if let Some(item) = player.get_inventory().get_main_hand() {
        let mat: Material = item.get_material();
        combat_style = mat.get_combat_style();
    }
    let adrenaline = 0.0;

    let monster_health: i32 = (monster.get_hitpoints() * 100) as i32;
    let monster_max_health: i32 = monster_health;
    let monster_adrenaline: f32 = 0.0;

    let monster_level: u64 = ((monster.get_melee()
        + monster.get_magic()
        + monster.get_ranged()
        + monster.get_defense()
        + monster.get_hitpoints()) as f32
        / 5.0) as u64;
    let monster_attack = match monster.get_attack_style().as_str() {
        "melee" => monster.get_melee(),
        "ranged" => monster.get_ranged(),
        "magic" => monster.get_magic(),
        _ => 0,
    };
    let monster_crit_chance = 0.1;

    let mut state = &mut FightState {
        player: &mut (*player),
        health: health,
        max_health: max_health,
        adrenaline: adrenaline,
        combat_style: combat_style,
        pl_crit_chance: pl_crit_chance,
        monster: monster,
        monster_health: monster_health,
        monster_max_health: monster_max_health,
        monster_adrenaline: monster_adrenaline,
        monster_level: monster_level,
        monster_attack: monster_attack,
        monster_crit_chance: monster_crit_chance,
    };

    *ACTION.lock().unwrap() = Action::COMBAT;

    while state.health > 0 && state.monster_health > 0 {
        clear_screen();

        print_status(&mut state);

        println!("\nWhat would you like to do?");
        println!("1. Attack");
        println!("2. Ability");
        println!("3. Item");
        println!("4. Run");

        print!("> ");

        let mut input: usize = read!();

        while input < 1 || input > 5 {
            println!("Invalid input. Please enter a number between 1 and 4.");
            print!("> ");
            input = read!();
        }

        match input {
            1 => {
                player_attack(state);

                thread::sleep(Duration::from_millis(
                    (1000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
                ));

                enemy_attack(state);

                thread::sleep(Duration::from_millis(
                    (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
                ));
            }
            2 => {
                ability_menu(state);
            }
            3 => {
                eat_menu(state);
            }
            4 => {
                println!("You run away humiliated but alive.");
                thread::sleep(Duration::from_millis(
                    (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
                ));
                *ACTION.lock().unwrap() = Action::IDLE;
                return;
            }
            _ => {
                println!("Invalid input. Please enter a number between 1 and 4.");
            }
        }
    }

    if state.health <= 0 {
        println!("You died at the hands of {}!", monster.get_name());
    } else if state.monster_health <= 0 {
        println!("You killed the {}!", monster.get_name());

        thread::sleep(Duration::from_millis(
            (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
        ));

        let xp = (state.monster_level as u64) * 15;
        let hp_xp = state.monster.get_hitpoints() as u64 * 20;
        let defense_xp = (state.max_health - state.health) as u64 / 3;
        state.player.add_xp(&combat_style.to_string(), xp);
        state.player.add_xp(&"hitpoints".to_string(), hp_xp);
        state.player.add_xp(&"defense".to_string(), defense_xp);

        println!(
            "You gained {} {} xp, {} defense xp, and {} hitpoints xp.",
            xp,
            combat_style.to_string(),
            defense_xp,
            hp_xp
        );

        thread::sleep(Duration::from_millis(
            (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
        ));

        let gold = state.monster.get_gold();

        println!("{} dropped {}g!", state.monster.get_name(), gold);
        state.player.add_gold(gold);

        for (material, min, max, chance) in monster.get_drops() {
            if random::<f32>() < chance {
                let quantity = match max - min {
                    0 => max,
                    _ => random::<u32>() % (max - min) + min,
                };
                println!(
                    "{} dropped {} x {}!",
                    state.monster.get_name(),
                    quantity,
                    material
                );
                state
                    .player
                    .get_inventory_mut()
                    .add_item(Item::new(material, quantity as i32));
            }
        }

        thread::sleep(Duration::from_millis(
            (3000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
        ));
    } else {
        println!("You ran away from the {}!", state.monster.get_name());
    }

    *ACTION.lock().unwrap() = Action::IDLE;

    thread::sleep(Duration::from_millis(
        (3000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
    ));
}

fn print_combat_stats() {
    let pl = PLAYER.lock().unwrap();
    let melee = &"melee".to_string();
    let ranged = &"ranged".to_string();
    let magic = &"magic".to_string();
    let defense = &"defense".to_string();
    let hitpoints = &"hitpoints".to_string();

    let total_lvl =
        (pl.get_level(melee) + pl.get_level(ranged) + pl.get_level(magic) + pl.get_level(defense))
            / 4;

    println!("Combat Stats (Total LVL: {})", total_lvl);
    println!("========================");
    println!(
        "Melee: {} ({} / {})",
        pl.get_level(melee),
        pl.get_xp(melee),
        pl.needed_xp(melee)
    );
    println!(
        "Ranged: {} ({} / {})",
        pl.get_level(ranged),
        pl.get_xp(ranged),
        pl.needed_xp(ranged)
    );
    println!(
        "Magic: {} ({} / {})",
        pl.get_level(magic),
        pl.get_xp(magic),
        pl.needed_xp(magic)
    );
    println!(
        "Defense: {} ({} / {})",
        pl.get_level(defense),
        pl.get_xp(defense),
        pl.needed_xp(defense)
    );
    println!(
        "Hitpoints: {} ({} / {})",
        pl.get_level(hitpoints),
        pl.get_xp(hitpoints),
        pl.needed_xp(hitpoints)
    );
    println!("========================\n");
}

pub fn combat_menu() {
    let mut input = 0;

    let mut monsters: Vec<Box<dyn MonsterData>> = Vec::new();
    monsters.push(Box::new(Rat::new()));
    monsters.push(Box::new(Slime::new()));
    monsters.push(Box::new(GiantRat::new()));
    monsters.push(Box::new(TreeSpirit::new()));

    while input as usize != monsters.len() + 1 {
        clear_screen();

        print_combat_stats();

        println!("Combat Menu");
        println!("What would you like to fight?");

        let mut i = 1;
        for monster in &monsters {
            print!(
                "  {}. {} (level: {}",
                i,
                monster.get_name(),
                ((monster.get_melee() + monster.get_ranged() + monster.get_magic()) / 3) as u32
            );
            if monster.get_reqs().len() > 0 {
                let player = PLAYER.lock().unwrap();
                if monster.can_fight(&player) != "" {
                    println!(", req. {})", monster.get_reqs());
                }
            } else {
                println!(")");
            }
            i += 1;
        }

        println!("  {}. Main Menu", i);

        print!("> ");

        input = read!();

        while input < 1 || input > i {
            println!("Invalid input. Please enter a number between 1 and {}.", i);
            print!("> ");
            input = read!();
        }

        if input == i {
            continue;
        }

        let monster: &Box<dyn MonsterData> = &monsters[(input - 1) as usize];

        fight(monster);
    }
}

pub fn max_hit_comp(attack: u64, defense: u64) -> u32 {
    let max_hit =
        (20.0 * (1.075f64.powf(attack as f64 - 1.0) / 1.025f64.powf(defense as f64 - 1.0))).ceil();
    max_hit as u32
}
