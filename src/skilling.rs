use text_io::read;

use crate::{
    game::PLAYER, io_manager::clear_screen, player::Player, skills::woodcutting::woodcutting_menu,
};

pub fn print_skills() {
    let pl = &PLAYER.lock().unwrap();
    let mut skills: Vec<String> = Vec::new();
    for (key, _) in pl.get_skills() {
        skills.push(key.clone());
    }
    println!("Current skills:");
    for skill in skills {
        println!(
            "{}: {} ({} / {})",
            skill,
            pl.get_level(&skill),
            pl.get_xp(&skill),
            pl.needed_xp(&skill)
        );
    }
}

pub fn skilling_menu() {
    clear_screen();

    print_skills();
    println!("\nWhich skill would you like to train?");
    println!("1. Woodcutting");
    println!("2. Main Menu");

    print!("> ");

    let mut input: usize = read!();
    while input < 1 || input > 2 {
        println!("Invalid input. Please enter a number between 1 and 1.");
        print!("> ");
        input = read!();
    }

    match input {
        1 => woodcutting_menu(),
        2 => {}
        _ => println!("Invalid input. Please enter a number between 1 and 1."),
    }
}
