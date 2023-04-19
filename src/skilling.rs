use text_io::read;

use crate::{io_manager::clear_screen, player::Player, skills::woodcutting::woodcutting_menu};

pub fn print_skills(player: &Player) {
    println!("Current skills:");
    println!("Woodcutting: {}", player.get_level(&"woodcutting".to_string()));
}

pub fn skilling_menu(player: &Player) {
    clear_screen();
    print_skills(player);
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
        1 => woodcutting_menu(player),
        _ => println!("Invalid input. Please enter a number between 1 and 1."),
    }
}