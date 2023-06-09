use std::{
    io::{self, Write},
    thread::{self},
    time::Duration,
};

use text_io::read;

use crate::{
    game::{game_loop, give_starter_items, ACTION, LOADED, PLAYER, self},
    io_manager::{clear_screen, get_all_saves, read_player_save, write_player_save},
    player::{Action, Player},
};

fn setup_ctrl_c_handler() {
    ctrlc::set_handler(move || {
        let mut act = ACTION.lock().unwrap();
        if *act == Action::IDLE {
            if !*LOADED.lock().unwrap() {
                println!("Exiting without saving...");
                thread::sleep(std::time::Duration::from_millis(500));
                std::process::exit(0);
            }
            write_player_save();
            println!("Saving and exiting...");
            thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0);
        } else {
            if *act != Action::COMBAT {
                *act = Action::IDLE;
            }
        }
    })
    .expect("Error setting Ctrl-C handler");
    println!("Press Ctrl-C to stop.");
}

pub fn create_character_menu() {
    println!("Enter your name:");
    print!("> ");
    let mut name: String = read!();
    while !is_valid_name(&name) {
        println!("Invalid name. Please enter a name between 3 and 40 characters long, containing only letters.");

        print!("> ");
        name = read!();
    }

    *PLAYER.lock().unwrap() = Player::new(name);
    *LOADED.lock().unwrap() = true;
    write_player_save();

    give_starter_items();
}

pub fn load_character_menu(saves: &Vec<String>) {
    println!("Saves:");
    for i in 0..saves.len() {
        println!("{}. {}", i + 1, saves[i]);
    }
    println!("Enter the number of the save you want to load:");
    print!("> ");
    let mut input: usize = read!();
    while input < 1 || input > saves.len() {
        println!(
            "Invalid input. Please enter a number between 1 and {}.",
            saves.len()
        );
        print!("> ");
        input = read!();
    }
    let save = saves[input - 1].clone();

    *PLAYER.lock().unwrap() = read_player_save(&save);
    *LOADED.lock().unwrap() = true;
}

pub fn main_menu() {
    clear_screen();

    setup_ctrl_c_handler();

    let mut input = 0;

    while input != 3 {
        *LOADED.lock().unwrap() = false;
        println!("Welcome to MicroMUD!");

        let saves = get_all_saves();
        if saves.len() == 0 {
            println!("No saves were found.");

            println!("Welcome to the game! The first step is to create a character!");

            thread::sleep(Duration::from_millis(
                (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
            ));
            create_character_menu();

            println!();
            println!("Now the adventure begins! Travel around new areas to train skills, complete quests, and fight monsters.");

            for i in 0..5 {
                thread::sleep(Duration::from_secs(1));
                print!("{}... ", 5 - i);
                io::stdout().flush().unwrap();
            }

            thread::sleep(Duration::from_secs(1));

            game_loop();
        }

        println!();

        println!("What would you like to do?");

        println!("1. New Game");
        println!("2. Load Game");
        println!("3. Exit");

        print!("> ");
        input = read!();

        match input {
            1 => {
                create_character_menu();

                game_loop();
            }
            2 => {
                load_character_menu(&saves);

                game_loop();
            }
            3 => {}
            _ => {
                println!("Invalid input.");
            }
        }
    }
}

fn is_valid_name(name: &String) -> bool {
    if name.len() < 3 || name.len() > 40 {
        return false;
    }
    for c in name.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    true
}
