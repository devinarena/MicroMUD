use player::Player;
use text_io::read;

use crate::io_manager::clear_screen;

pub mod io_manager;
///
/// MicroMUD by Devin Arena
/// A small text-based RPG game written in Rust
///
pub mod player;

fn game_loop(player: Player) {
    clear_screen();
    println!("Welcome to MicroMUD, {}!", player.get_name());
    println!("Current location: {}", player.get_location());
    println!("What would you like to do?");
    println!("1. View Stats");
    println!("2. View Inventory");
    println!("3. Skill");
    println!("4. Fight");
    println!("5. Move")
    println!("6. Save");
    println!("7. Exit");

}

fn create_character_menu() {
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

    let player = player::Player::new(name, "Warrior".to_string());
    io_manager::write_player_save(&player);
    game_loop(player);
}

fn load_character_menu(saves: &Vec<String>) {
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
    let player = io_manager::read_player_save(&save);
    game_loop(player);
}

fn main_menu() {
    clear_screen();
    println!("Welcome to MicroMUD!");

    let saves = io_manager::get_all_saves();
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

fn main() {
    main_menu();
}
