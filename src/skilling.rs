use text_io::read;

use crate::{io_manager::clear_screen, player::Player, skills::woodcutting::woodcutting_menu, game::player};

pub fn print_skills(pl: &Player) {
    println!("Current skills:");
    println!("Woodcutting: {}", pl.get_level(&"woodcutting".to_string()));
}

pub fn skilling_menu() {
    clear_screen();

    let mut pl= &player.lock().unwrap();

    print_skills(pl);
    println!("\nWhich skill would you like to train?");
    println!("1. Woodcutting");
    println!("2. Main Menu");

    print!("> ");

    let mut input: usize = read!();
    while input < 1 || input > 1 {
        println!("Invalid input. Please enter a number between 1 and 1.");
        print!("> ");
        input = read!();
    }

    match input {
        1 => woodcutting_menu(),
        _ => println!("Invalid input. Please enter a number between 1 and 1."),
    }
}