use startup::main_menu;

pub mod game;
pub mod inventory;
pub mod io_manager;
pub mod item;
///
/// MicroMUD by Devin Arena
/// A small text-based RPG game written in Rust
///
pub mod player;
pub mod skilling;
pub mod skills;
pub mod startup;
pub mod combat;
pub mod shop;

fn main() {
    main_menu();
}
