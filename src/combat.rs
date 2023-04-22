use std::{thread, time::Duration};

use rand::random;
use text_io::read;

use crate::{
    combat::monsters::{rat::Rat, tree_spirit::TreeSpirit},
    game::{ACTION, PLAYER},
    io_manager::clear_screen,
    item::Item,
    player::Action,
};

use self::monsters::MonsterData;

pub mod monsters;

fn fight(monster: &Box<dyn MonsterData>) {
    let mut pl = PLAYER.lock().unwrap();

    let mut health: i32 = pl.get_health();
    let mut ehealth: i32 = (monster.get_hitpoints() * 100) as i32;
    let emax_health = health;

    let pl_level = (pl.get_level(&"melee".to_string())
        + pl.get_level(&"ranged".to_string())
        + pl.get_level(&"magic".to_string())
        + pl.get_level(&"defense".to_string())) as f32
        / 4.0;

    let elevel =
        ((monster.get_melee() + monster.get_magic() + monster.get_ranged() + monster.get_defence())
            as f32
            / 4.0) as i32;
    let eattack = match monster.get_attack_style().as_str() {
        "melee" => monster.get_melee(),
        "ranged" => monster.get_ranged(),
        "magic" => monster.get_magic(),
        _ => 0,
    };

    *ACTION.lock().unwrap() = Action::COMBAT;

    while health > 0 && ehealth > 0 {
        clear_screen();

        println!(
            "BATTLE BETWEEN {} (LVL {}) AND YOU (LVL {})",
            monster.get_name(),
            elevel as u32,
            pl_level as u32
        );
        println!("========================================\n");

        let pl_attack = pl.get_level(&"melee".to_string());
        let pl_defense = pl.get_level(&"defense".to_string());

        println!("> {}: {} / {}\n", monster.get_name(), ehealth, emax_health);

        println!(
            "> You: {} / {}",
            health,
            pl.get_level(&"hitpoints".to_string()) * 100
        );

        println!("\n========================================");

        println!("\nWhat would you like to do?");
        println!("1. Attack");
        println!("2. Item");
        println!("3. Run");

        print!("> ");

        let mut input: usize = read!();

        while input < 1 || input > 3 {
            println!("Invalid input. Please enter a number between 1 and 3.");
            print!("> ");
            input = read!();
        }

        match input {
            1 => {
                let max_hit = (10.0 * (pl_attack as f64 + 1.0).log2()
                    / (monster.get_defence() as f64 + 1.0).log(4.0))
                    as i32;
                let damage = (max_hit as f32 * random::<f32>()) as i32;
                ehealth -= damage;
                println!(
                    "You hit the {} for {} melee damage.",
                    monster.get_name(),
                    damage
                );

                thread::sleep(Duration::from_secs(1));

                let e_max_hit = (10.0 * (eattack as f64 + 1.0).log2()
                    / (pl_defense as f64 + 1.0).log(4.0)) as i32;
                let edmg = (e_max_hit as f32 * random::<f32>()) as i32;
                health -= edmg;
                println!(
                    "The {} hits you for {} {} damage.",
                    monster.get_name(),
                    edmg,
                    monster.get_attack_style(),
                );

                thread::sleep(Duration::from_secs(2));
            }
            2 => {
                println!("You don't have any items!");
            }
            3 => {
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

        let xp = (elevel * 75) as u64;
        let hp_xp = (monster.get_hitpoints() * 50) as u64;
        let defense_xp = (pl.get_level(&"hitpoints".to_string()) * 10 - health as u32) * 10;
        pl.add_xp(&"melee".to_string(), xp);
        pl.add_xp(&"hitpoints".to_string(), hp_xp);
        pl.add_xp(&"defense".to_string(), defense_xp as u64);

        println!(
            "You gained {} melee xp, {} xp, and {} hitpoints xp.",
            xp, defense_xp, hp_xp
        );

        thread::sleep(Duration::from_secs(1));

        let gold = monster.get_gold();

        println!("{} dropped {}g!", monster.get_name(), gold);
        pl.add_gold(gold);

        for (material, min, max, chance) in monster.get_drops() {
            if random::<f32>() < chance {
                let quantity = random::<u32>() % (max - min) + min;
                println!("Rat dropped {} x {}!", quantity, material);
                pl.get_inventory_mut()
                    .add_item(Item::new(material, quantity as i32));
            }
        }

        thread::sleep(Duration::from_secs(2));
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
    println!("============");
    println!("Melee: {} ({} / {})", pl.get_level(melee), pl.get_xp(melee), pl.needed_xp(melee));
    println!("Ranged: {} ({} / {})", pl.get_level(ranged), pl.get_xp(ranged), pl.needed_xp(ranged));
    println!("Magic: {} ({} / {})", pl.get_level(magic), pl.get_xp(magic), pl.needed_xp(magic));
    println!("Defense: {} ({} / {})", pl.get_level(defense), pl.get_xp(defense), pl.needed_xp(defense));
    println!("Hitpoints: {} ({} / {})", pl.get_level(hitpoints), pl.get_xp(hitpoints), pl.needed_xp(hitpoints));
    println!("============\n");
}

pub fn combat_menu() {
    clear_screen();

    print_combat_stats();

    println!("Combat Menu");
    println!("What would you like to fight?");

    let mut monsters: Vec<Box<dyn MonsterData>> = Vec::new();
    monsters.push(Box::new(Rat::new()));
    monsters.push(Box::new(TreeSpirit::new()));

    let mut i = 1;
    for monster in &monsters {
        println!(
            "  {}. {} (level: {})",
            i,
            monster.get_name(),
            ((monster.get_melee() + monster.get_ranged() + monster.get_magic()) / 3) as u32
        );
        i += 1;
    }

    println!("  {}. Main Menu", i);

    print!("> ");

    let mut input: u32 = read!();

    while input < 1 || input > i {
        println!("Invalid input. Please enter a number between 1 and {}.", i);
        print!("> ");
        input = read!();
    }

    if input == i {
        return;
    }

    let monster: &Box<dyn MonsterData> = &monsters[(input - 1) as usize];

    fight(monster);
}
