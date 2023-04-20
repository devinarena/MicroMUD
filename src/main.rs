use startup::main_menu;


pub mod io_manager;
///
/// MicroMUD by Devin Arena
/// A small text-based RPG game written in Rust
///
pub mod player;
pub mod startup;
pub mod game;
pub mod item;
pub mod skilling;
pub mod inventory;
pub mod skills;


fn main() {
    // main_menu();
    let mut xp: f64 = 32.0;
    let mut level: i32 = 1;
    let mut ntotal = 0.0;
    while xp > 0.0 {
        let needed_xp = 100.0 * 1.75_f64.powf((level - 1) as f64 / 8.0) / 4.7;
        if xp >= needed_xp as f64 {
            level += 1;
            xp -= needed_xp;
            ntotal += needed_xp;
        } else {
            println!("Level: {}", level);
            println!("Next level: {}", ntotal + needed_xp);
            println!("Needed XP: {}", xp);
            break;
        }
    }
}
