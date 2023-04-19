use std::sync::Mutex;

use text_io::read;

use crate::{io_manager::clear_screen, player::{Player, Action}, skilling::skilling_menu};

pub static TICK_RATE: u32 = 20;
pub static action: Mutex<Action> = Mutex::new(Action::IDLE);
pub static player: Mutex<Player>;

pub fn game_loop() {
    clear_screen();

    let mut pl = player.lock().unwrap();

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

    let mut input: usize = read!();
    while input < 1 || input > 7 {
        println!("Invalid input. Please enter a number between 1 and 7.");
        print!("> ");
        input = read!();
    }

    match input {
        3 => {
            skilling_menu();
        }
        _ => {}
    }
}