use text_io::read;

use crate::{io_manager::clear_screen, player::Player};



pub fn game_loop(player: &Player) {
    clear_screen();
    println!("Welcome to MicroMUD, {}!", player.get_name());
    println!("Current location: {}", player.get_location());
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
}