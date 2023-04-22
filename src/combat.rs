use std::{thread, time::Duration};

use rand::random;
use text_io::read;

use crate::{
    combat::{monster::{MonsterData}, rat::Rat},
    game::{PLAYER, ACTION},
    io_manager::clear_screen, player::Action,
};

pub mod monster;
pub mod rat;

fn fight(monster: &Box<dyn MonsterData>) {
    let mut pl = PLAYER.lock().unwrap();

    let mut health: i32 = pl.get_health();
    let mut ehealth: i32 = monster.get_max_hp() as i32;

    let elevel = ((monster.get_melee() + monster.get_magic() + monster.get_ranged()) as f32 / 3.0) as i32;

    *ACTION.lock().unwrap() = Action::COMBAT;

    while health > 0 && ehealth > 0  {
        clear_screen();

        println!(
            "{}: {} / {}\n",
            monster.get_name(),
            ehealth,
            monster.get_max_hp()
        );

        println!(
            "You: {} / {}",
            health,
            pl.get_level(&"hitpoints".to_string()) * 100
        );

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
                let damage: i32 =
                    (pl.get_level(&"melee".to_string()) as f32 * 20.0 * random::<f32>()) as i32;
                ehealth -= damage;
                println!(
                    "You hit the {} for {} melee damage.",
                    monster.get_name(),
                    damage
                );

                thread::sleep(Duration::from_secs(1));

                let edmg = (monster.get_melee() as f32 * 20.0 * random::<f32>()) as i32;
                health -= edmg;
                println!(
                    "The {} hits you for {} melee damage.",
                    monster.get_name(),
                    edmg
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

        let xp = (elevel * 8) as u64;
        let hpxp = xp * 3 / 5;;
        pl.add_xp(&"melee".to_string(), xp);
        pl.add_xp(&"hitpoints".to_string(), hpxp);

        println!("You gained {} melee xp and {} hitpoints xp.", xp, hpxp);

        thread::sleep(Duration::from_secs(1));

        for (item, chance) in monster.get_drops() {
            if random::<f32>() < chance {
                println!("Rat dropped {} x {}!", item.get_quantity(), item.get_material().get_name());
                pl.get_inventory_mut().add_item(item);
            }
        }

        thread::sleep(Duration::from_secs(2));
    } else {
        println!("You ran away from the {}!", monster.get_name());
    }

    *ACTION.lock().unwrap() = Action::IDLE;

    thread::sleep(std::time::Duration::from_secs(3));
}

pub fn combat_menu() {
    clear_screen();

    println!("Combat Menu");
    println!("What would you like to fight?");

    let monsters: Vec<Box<dyn MonsterData>> = vec![Box::new(Rat::new())];

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

    let monster = &monsters[(input - 1) as usize];

    fight(monster);
}
