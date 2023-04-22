pub mod tree;

use std::{thread, time::Duration};

use rand::random;
use text_io::read;

use crate::{
    game::{self, ACTION, PLAYER},
    io_manager::clear_screen,
    item::{Item, Material, MaterialType},
    player::Action,
    skills::woodcutting::tree::NormalTree,
};

use self::tree::{BirchTree, OakTree, TreeData};

pub fn woodcut(tree: &dyn TreeData) {
    clear_screen();
    println!("You walk over to the nearest {}.", tree.get_name());

    let mut pl = PLAYER.lock().unwrap();
    let mh = pl.get_inventory().get_main_hand().clone();

    if mh.is_none()
        || mh
            .as_ref()
            .expect("Unexpected item")
            .get_material()
            .get_type()
            != MaterialType::Axe
    {
        println!("You don't have an axe equipped!");
        thread::sleep(Duration::new(1, 0));
        return;
    }

    let skill = &"woodcutting".to_string();

    if pl.get_level(skill) < tree.get_required_level() {
        println!(
            "You need a woodcutting level of {} to cut this tree.",
            tree.get_required_level()
        );
        thread::sleep(Duration::new(3, 0));
        return;
    }

    println!("You begin to chop down the tree.");

    let mut act = ACTION.lock().unwrap();
    *act = Action::CHOPPING;
    drop(act);

    while *ACTION.lock().unwrap() == Action::CHOPPING {
        // max decrease to roll is currently -22
        let bonus = (pl.get_level(skill) as f32 * 20.0 / 150.0).min(20.0) as u32;
        let roll =
            random::<u32>() % (100 - bonus - mh.as_ref().unwrap().get_material().get_axe_bonus());

        if roll < tree.get_success_rate() {
            let mut quantity = 1;
            if let Some(item) = pl
                .get_inventory()
                .find_item(tree.get_result().get_material())
            {
                quantity = item.get_quantity() + 1;
            }
            println!(
                "You chop the tree and get some {}. (+{} woodcutting xp) ({})",
                tree.get_result().get_material().get_name(),
                tree.get_xp(),
                quantity
            );
            pl.add_xp(skill, tree.get_xp());
            pl.get_inventory_mut().add_item(tree.get_result());
        }

        if roll < tree.get_apple_chance() {
            let mut quantity = 1;
            if let Some(item) = pl.get_inventory().find_item(Material::Apple) {
                quantity = item.get_quantity() + 1;
            }
            println!(
                "You find an apple while chopping the tree! (+65 woodcutting xp) ({})",
                quantity
            );
            pl.add_xp(skill, 65);
            pl.get_inventory_mut()
                .add_item(Item::new(Material::Apple, 1));
        }

        thread::sleep(Duration::new(
            0,
            (1000000000_f32 / game::TICK_RATE as f32 * game::TICK_RATE as f32 * game::SPEED_SCALE)
                as u32,
        ));
    }

    println!("You stop chopping the tree.");
    thread::sleep(Duration::new(3, 0));
}

pub fn woodcutting_menu() {
    let mut input = 0;

    // list of all trees
    let trees: Vec<Box<dyn TreeData>> = vec![
        Box::new(NormalTree::new()),
        Box::new(OakTree::new()),
        Box::new(BirchTree::new()),
    ];

    while input != trees.len() + 1 {
        clear_screen();
        println!("Which tree would you like to cut?");
        let mut i = 1;
        for tree in trees.iter() {
            println!(
                "{}. {} (req lv: {})",
                i,
                tree.get_name(),
                tree.get_required_level()
            );
            i += 1;
        }
        println!("{}. Main Menu", i);

        print!("> ");

        input = read!();
        while input < 1 || input > i {
            println!("Invalid input. Please enter a number between 1 and 2.");
            print!("> ");
            input = read!();
        }

        if input == i {
            return;
        }

        woodcut(trees[input - 1].as_ref());
    }
}
