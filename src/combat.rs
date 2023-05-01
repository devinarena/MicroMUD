use std::{thread, time::Duration};

use rand::random;
use text_io::read;

use crate::{
    combat::monsters::{rat::Rat, tree_spirit::TreeSpirit},
    game::{ACTION, PLAYER},
    io_manager::clear_screen,
    item::Item,
    player::{Action, Player},
};

use self::monsters::{giant_rat::GiantRat, MonsterData};

pub mod monsters;

fn print_status(
    pl: &mut Player,
    monster: &Box<dyn MonsterData>,
    health: i32,
    ehealth: i32,
    emax_health: i32,
    elevel: u64,
) {
    println!(
        "BATTLE BETWEEN {} (LVL {}) AND YOU (LVL {})",
        monster.get_name(),
        elevel as u32,
        pl.get_combat_level() as u32
    );
    println!("========================================\n");

    println!("> {}: {} / {}\n", monster.get_name(), ehealth, emax_health);

    println!(
        "> You: {} / {}",
        health,
        pl.get_level(&"hitpoints".to_string()) * 100
    );

    println!("\n========================================");
}

fn player_attack(
    pl: &mut Player,
    monster: &Box<dyn MonsterData>,
    ehealth: &mut i32,
    pl_crit_chance: f32,
) {
    let max_hit = _max_hit_comp(pl.get_attack_bonus(), monster.get_defense());
    let damage = (max_hit as f32 * random::<f32>()) as i32;
    if random::<f32>() < pl_crit_chance {
        *ehealth -= damage * 2;
        println!(
            "You critically hit the {} for {} melee damage.",
            monster.get_name(),
            damage * 2
        );
    } else {
        *ehealth -= damage;
        println!(
            "You hit the {} for {} melee damage.",
            monster.get_name(),
            damage
        );
    }
}

fn enemy_attack(
    pl: &mut Player,
    monster: &Box<dyn MonsterData>,
    health: &mut i32,
    eattack: u64,
    ecrit_chance: f32,
) {
    let e_max_hit = _max_hit_comp(eattack, pl.get_defense_bonus());
    let edmg = (e_max_hit as f32 * random::<f32>()) as i32;
    if random::<f32>() < ecrit_chance {
        *health -= edmg * 2;
        println!(
            "The {} critically hit you for {} {} damage.",
            monster.get_name(),
            edmg * 2,
            monster.get_attack_style(),
        );
    } else {
        *health -= edmg;
        println!(
            "The {} hit you for {} {} damage.",
            monster.get_name(),
            edmg,
            monster.get_attack_style(),
        );
    }
}

fn eat_menu(
    pl: &mut Player,
    health: &mut i32,
    monster: &Box<dyn MonsterData>,
    ehealth: i32,
    emax_health: i32,
    elevel: u64,
    eattack: u64,
    ecrit_chance: f32,
) {
    if pl.get_inventory().get_items().len() == 0 {
        println!("You have no items to use.");
        thread::sleep(Duration::from_secs(2));
        return;
    }
    let mut input: usize = 1;

    while input != 0 {
        clear_screen();

        print_status(pl, monster, *health, ehealth, emax_health, elevel);

        println!("\nWhat item would you like to use?");
        let mut index = 1;
        let mut food = Vec::<(Item, usize)>::new();
        for (i, item) in pl.get_inventory().get_items().iter().enumerate() {
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
            thread::sleep(Duration::from_secs(2));
            continue;
        }

        let item = &food[input - 1];

        if item.0.get_material().get_food_heal() == 0 {
            println!("You cannot eat that item.");
            thread::sleep(Duration::from_secs(2));
            continue;
        } else {
            let heal = item.0.get_material().get_food_heal() as i32;
            *health += heal;
            pl.get_inventory_mut().remove_quantity(item.1, 1);
            println!(
                "You eat the {} and heal {} health.",
                item.0.get_material().get_name(),
                heal
            );
            thread::sleep(Duration::from_secs(1));

            enemy_attack(pl, monster, health, eattack, ecrit_chance);

            thread::sleep(Duration::from_secs(2));
            break;
        }
    }
}

fn fight(monster: &Box<dyn MonsterData>) {
    clear_screen();

    let mut pl = PLAYER.lock().unwrap();
    let can_fight = monster.can_fight(&pl);
    if can_fight != "" {
        println!("{}", can_fight);
        thread::sleep(Duration::new(3, 0));
        return;
    }

    let mut health: i32 = pl.get_health();
    let mut ehealth: i32 = (monster.get_hitpoints() * 100) as i32;
    let emax_health = ehealth;

    let pl_crit_chance = 0.15;

    let elevel = ((monster.get_melee()
        + monster.get_magic()
        + monster.get_ranged()
        + monster.get_defense()
        + monster.get_hitpoints()) as f32
        / 5.0) as u64;
    let eattack = match monster.get_attack_style().as_str() {
        "melee" => monster.get_melee(),
        "ranged" => monster.get_ranged(),
        "magic" => monster.get_magic(),
        _ => 0,
    };
    let ecrit_chance = 0.1;

    *ACTION.lock().unwrap() = Action::COMBAT;

    while health > 0 && ehealth > 0 {
        clear_screen();

        print_status(&mut pl, monster, health, ehealth, emax_health, elevel);

        println!("\nWhat would you like to do?");
        println!("1. Attack");
        println!("2. Item");
        println!("3. Magic");
        println!("4. Run");

        print!("> ");

        let mut input: usize = read!();

        while input < 1 || input > 4 {
            println!("Invalid input. Please enter a number between 1 and 4.");
            print!("> ");
            input = read!();
        }

        match input {
            1 => {
                player_attack(&mut pl, monster, &mut ehealth, pl_crit_chance);

                thread::sleep(Duration::from_secs(1));

                enemy_attack(&mut pl, monster, &mut health, eattack, ecrit_chance);

                thread::sleep(Duration::from_secs(2));
            }
            2 => {
                eat_menu(
                    &mut pl,
                    &mut health,
                    monster,
                    ehealth,
                    emax_health,
                    elevel,
                    eattack,
                    ecrit_chance,
                );
            }
            3 => {
                println!("You don't know any magic spells.");
                thread::sleep(Duration::from_secs(2));
            }
            4 => {
                println!("You run away humiliated but alive.");
                thread::sleep(Duration::from_secs(3));
                *ACTION.lock().unwrap() = Action::IDLE;
                return;
            }
            _ => {
                println!("Invalid input. Please enter a number between 1 and 3.");
            }
        }
    }

    if health <= 0 {
        println!("You died at the hands of {}!", monster.get_name());
    } else if ehealth <= 0 {
        println!("You killed the {}!", monster.get_name());

        thread::sleep(Duration::from_secs(1));

        let xp = (elevel as u64) * 15;
        let hp_xp = monster.get_hitpoints() as u64 * 20;
        let defense_xp = (pl.get_level(&"hitpoints".to_string()) as u64 * 100 - health as u64) / 3;
        pl.add_xp(&"melee".to_string(), xp);
        pl.add_xp(&"hitpoints".to_string(), hp_xp);
        pl.add_xp(&"defense".to_string(), defense_xp);

        println!(
            "You gained {} melee xp, {} defense xp, and {} hitpoints xp.",
            xp, defense_xp, hp_xp
        );

        thread::sleep(Duration::from_secs(2));

        let gold = monster.get_gold();

        println!("{} dropped {}g!", monster.get_name(), gold);
        pl.add_gold(gold);

        for (material, min, max, chance) in monster.get_drops() {
            if random::<f32>() < chance {
                let quantity = match max - min {
                    0 => max,
                    _ => random::<u32>() % (max - min) + min,
                };
                println!(
                    "{} dropped {} x {}!",
                    monster.get_name(),
                    quantity,
                    material
                );
                pl.get_inventory_mut()
                    .add_item(Item::new(material, quantity as i32));
            }
        }

        thread::sleep(Duration::from_secs(3));
    } else {
        println!("You ran away from the {}!", monster.get_name());
    }

    *ACTION.lock().unwrap() = Action::IDLE;

    thread::sleep(std::time::Duration::from_secs(3));
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
                println!(", req. {})", monster.get_reqs());
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

fn _max_hit_comp(attack: u64, defense: u64) -> u32 {
    let max_hit = (15.0 * (attack as f64 + 1.0) / (defense as f64 + 1.0).sqrt()).ceil() as u32;
    max_hit
}
