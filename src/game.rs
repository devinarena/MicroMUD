use std::{
    io::{stdout, Read, Write},
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
    skilling::skilling_menu,
};

pub static TICK_RATE: u32 = 20;
pub static SPEED_SCALE: f32 = 1.0;
pub static ACTION: Mutex<Action> = Mutex::new(Action::IDLE);
pub static LOADED: Mutex<bool> = Mutex::new(false);
lazy_static! {
    pub static ref PLAYER: Mutex<Player> = Mutex::new(Player::empty());
}

pub fn game_loop() {
    let mut input: usize = 0;
    while input != 7 {
        clear_screen();

        let pl = PLAYER.lock().unwrap();

        println!("Welcome to MicroMUD, {}!", pl.get_name());
        println!("Current location: {}", pl.get_location());
        println!("What would you like to do?");
        println!("  1. View Stats");
        println!("  2. View Inventory");
        println!("  3. Skill");
        println!("  4. Fight");
        println!("  5. Move");
        println!("  6. Save");
        println!("  7. Exit");

        print!("> ");

        drop(pl);

        input = read!();
        while input < 1 || input > 7 {
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
            6 => {
                stdout().flush().unwrap();
                write_player_save();
                println!("Game saved!");
                thread::sleep(Duration::from_secs(1));
            }
            _ => {}
        }
    }
}
