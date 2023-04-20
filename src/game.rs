use std::sync::Mutex;

use lazy_static::lazy_static;
use text_io::read;

use crate::{
    io_manager::clear_screen,
    player::{Action, Player},
    skilling::skilling_menu, inventory::view_inventory,
};

pub static TICK_RATE: u32 = 20;
pub static action: Mutex<Action> = Mutex::new(Action::IDLE);
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
        println!("1. View Stats");
        println!("2. View Inventory");
        println!("3. Skill");
        println!("4. Fight");
        println!("5. Move");
        println!("6. Save");
        println!("7. Exit");

        print!("> ");

        drop(pl);

        input = read!();
        while input < 1 || input > 7 {
            println!("Invalid input. Please enter a number between 1 and 7.");
            print!("> ");
            input = read!();
        }

        match input {
            2 => {
                view_inventory();
            }
            3 => {
                skilling_menu();
            }
            _ => {}
        }
    }
}
