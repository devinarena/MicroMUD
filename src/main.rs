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
    main_menu();
}
