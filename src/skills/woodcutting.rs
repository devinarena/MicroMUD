pub mod tree;

use std::{thread, time::Duration};

use rand::random;
use text_io::read;

use crate::{
    game::{self, action, PLAYER},
    io_manager::clear_screen,
    player::{Action, Player},
    skills::woodcutting::tree::NormalTree,
};

use self::tree::TreeData;

pub fn woodcut(tree: &dyn TreeData) {
    clear_screen();
    println!("You walk over to the nearest tree.");
    println!("You begin to chop down the tree.");

    let mut pl = PLAYER.lock().unwrap();
    let mut act = action.lock().unwrap();
    *act = Action::CHOPPING;

    drop(act);

    while *action.lock().unwrap() == Action::CHOPPING {
        let roll = random::<u32>() % (100 - (pl.get_level(&"woodcutting".to_string()) / 2).min(20) as u32);

        if roll < tree.get_success_rate() {
            println!("You chop the tree and get some logs. (+{} woodcutting xp)", tree.get_xp());
            pl.add_xp(&"woodcutting".to_string(), tree.get_xp());
            pl.get_inventory().add_item(tree.get_result());
        }

        thread::sleep(Duration::new(
            0,
            1000000000 / game::TICK_RATE * game::TICK_RATE,
        ));
    }

    println!("You stop chopping the tree.");
    thread::sleep(Duration::new(1, 0));
}

pub fn woodcutting_menu() {
    clear_screen();

    let mut input = 0;

    while input != 2 {
        println!("Which tree would you like to cut?");
        println!("1. Normal Tree");
        println!("2. Main Menu");

        print!("> ");

        input = read!();
        while input < 1 || input > 2 {
            println!("Invalid input. Please enter a number between 1 and 2.");
            print!("> ");
            input = read!();
        }

        match input {
            1 => {
                let normal_tree = NormalTree::new();
                woodcut(&normal_tree);
            }
            _ => {}
        }
    }
}
