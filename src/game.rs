use std::{
    io::{stdout, Write},
    sync::Mutex,
    thread,
    time::Duration,
};

use lazy_static::lazy_static;
use text_io::read;

use crate::{
    inventory::view_inventory,
    io_manager::{clear_screen, write_player_save},
    player::{Action, Player},
    skilling::skilling_menu, combat::combat_menu, item::{Material, Item},
};

pub static TICK_RATE: u32 = 1;
pub static SPEED_SCALE: f32 = 0.5;
pub static ACTION: Mutex<Action> = Mutex::new(Action::IDLE);
pub static LOADED: Mutex<bool> = Mutex::new(false);
lazy_static! {
    pub static ref PLAYER: Mutex<Player> = Mutex::new(Player::empty());
}

pub fn game_loop() {
    let mut input: usize = 0;
    while input != 8 {
        clear_screen();

        let pl = PLAYER.lock().unwrap();

        println!("Welcome to MicroMUD, {}!", pl.get_name());
        println!("Current location: {}", pl.get_location());
        println!("What would you like to do?");
        println!("  1. View Stats");
        println!("  2. View Inventory");
        println!("  3. Skill");
        println!("  4. Fight");
        println!("  5. Shop");
        println!("  6. Move");
        println!("  7. Save");
        println!("  8. Exit");

        print!("> ");

        drop(pl);

        input = read!();
        while input < 1 || input > 8 {
            println!("Invalid input. Please enter a number between 1 and 7.");
            print!("> ");
            input = read!();
        }

        match input {
            1 => {
                stdout().flush().unwrap();
                PLAYER.lock().unwrap().print_stats();
                println!("Press enter to continue...");
                let _: String = read!();
            }
            2 => {
                stdout().flush().unwrap();
                view_inventory();
            }
            3 => {
                skilling_menu();
            }
            4 => {
                combat_menu();
            }
            5 => {
                
            }
            7 => {
                stdout().flush().unwrap();
                write_player_save();
                println!("Game saved!");
                thread::sleep(Duration::from_millis(
                    (100_f32 / TICK_RATE as f32 * SPEED_SCALE) as u64,
                ));
            }
            _ => {}
        }
    }
}

pub fn give_starter_items() {
    let mut pl = PLAYER.lock().unwrap();
    pl.get_inventory_mut().add_item(Item::new(Material::WoodenDagger, 1));
    pl.get_inventory_mut().add_item(Item::new(Material::WoodenShield, 1));
}