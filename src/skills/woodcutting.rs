
pub mod tree;

use std::{thread, time::Duration};

use rand::random;
use text_io::read;

use crate::{io_manager::clear_screen, player::{Player, Action}, game::{self, action}};

use self::tree::TreeData;

pub fn woodcut(player: &mut Player, tree: &dyn TreeData) {
    clear_screen();
    println!("You walk over to the nearest tree.");
    println!("You begin to chop down the tree.");

    let mut act = action.lock().unwrap();
    *act = Action::CHOPPING;

    while *act == Action::CHOPPING {
        let roll = random::<u32>() % 100;

        if roll < tree.get_success_rate() {
            println!("You chop the tree and get some logs.");
        }

        thread::sleep(Duration::new(0, 1000000000 / game::TICK_RATE));
    }

    println!("You stop chopping the tree.");
    woodcutting_menu(player);
}

pub fn woodcutting_menu(player: &Player) {
    clear_screen();
    println!("Which tree would you like to cut?");
    println!("1. Normal Tree");
    println!("2. Main Menu");

    print!("> ");

    let mut input: usize = read!();
    while input < 1 || input > 2 {
        println!("Invalid input. Please enter a number between 1 and 2.");
        print!("> ");
        input = read!();
    }
}