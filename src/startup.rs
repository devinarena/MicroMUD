use text_io::read;

use crate::{
    game::game_loop,
    io_manager::{clear_screen, get_all_saves, read_player_save, write_player_save},
    player::Player,
};

pub fn create_character_menu() {
    println!("Enter your name:");
    print!("> ");
    let mut name: String = read!();
    while !is_valid_name(&name) {
        println!("Invalid name. Please enter a name between 3 and 40 characters long, containing only letters.");

        print!("> ");
        name = read!();
    }

    println!("Classes:");
    println!("Enter your class:");

    let player = Player::new(name, "Warrior".to_string());
    write_player_save(&player);
    game_loop(&player);
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
    let player = read_player_save(&save);
    game_loop(&player);
}

pub fn main_menu() {
    clear_screen();
    println!("Welcome to MicroMUD!");

    let saves = get_all_saves();
    if saves.len() == 0 {
        println!("No saves were found.");
    }

    println!();

    println!("What would you like to do?");

    println!("1. New Game");
    println!("2. Load Game");
    println!("3. Exit");

    print!("> ");
    let input: i32 = read!();

    match input {
        1 => {
            create_character_menu();
        }
        2 => {
            load_character_menu(&saves);
        }
        _ => {
            println!("Invalid input.");
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
