use text_io::read;

use crate::{game::PLAYER, io_manager::clear_screen, skills::{woodcutting::woodcutting_menu, firemaking::firemaking_menu}};

pub fn print_skills() {
    let pl = &PLAYER.lock().unwrap();
    let mut skills: Vec<String> = Vec::new();
    for (key, _) in pl.get_skills() {
        skills.push(key.clone());
    }
    println!("Current skills:");
    for skill in skills {
        println!(
            "  {}: {} ({} / {})",
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
    println!("2. Firemaking");
    println!("3. Main Menu");

    print!("> ");

    let mut input: usize = read!();
    while input < 1 || input > 3 {
        println!("Invalid input. Please enter a number between 1 and 3.");
        print!("> ");
        input = read!();
    }

    match input {
        1 => woodcutting_menu(),
        2 => firemaking_menu(),
        3 => {}
        _ => println!("Invalid input. Please enter a number between 1 and 1."),
    }
}
