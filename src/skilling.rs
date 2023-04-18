use text_io::read;

use crate::{io_manager::clear_screen, player::Player};

pub fn print_skills(player: &Player) {
    println!("Current skills:");
    println!("Woodcutting: {}", player.get_level(&"woodcutting".to_string()));
}

pub fn skilling_menu(player: &Player) {
    clear_screen();
    print_skills(player);
    println!("Which skill would you like to train?");
    println!("1. Woodcutting");

    print!("> ");

    let mut input: usize = read!();
    while input < 1 || input > 1 {
        println!("Invalid input. Please enter a number between 1 and 1.");
        print!("> ");
        input = read!();
    }
}

pub fn woodcutting_menu(player: &Player) {
    clear_screen();
    println!("Which tree would you like to cut?");
    println!("1. Normal Tree");
    println!("2. Oak Tree");
    println!("3. Willow Tree");
    println!("4. Maple Tree");
    println!("5. Yew Tree");
    println!("6. Magic Tree");
}