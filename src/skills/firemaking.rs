use std::{thread, time::Duration};

use text_io::read;

use crate::{
    game::{ACTION, PLAYER},
    io_manager::clear_screen,
    item::{Material, MaterialType},
    player::Action,
    skills::firemaking::logdata::LogData,
};

pub mod logdata;

pub fn firemake(material: Material) {
    clear_screen();

    let pl = &mut PLAYER.lock().unwrap();

    let skill = &"firemaking".to_string();

    if pl.get_level(skill) < material.get_firemaking_level() {
        println!(
            "You need a firemaking level of {} to burn {}.",
            material.get_firemaking_level(),
            material.get_name()
        );
        thread::sleep(Duration::new(3, 0));
        return;
    }

    println!("You begin firemaking.");

    let mut act = ACTION.lock().unwrap();
    *act = Action::FIREMAKING;
    drop(act);

    let index = pl.get_inventory().find_item_index(material);

    if index.is_none() {
        println!("You don't have any {} to burn!", material.get_name());
        thread::sleep(Duration::new(3, 0));
        return;
    }

    let idx = index.unwrap();

    let mh = pl.get_inventory().get_main_hand();
    if mh.is_none() || mh.as_ref().unwrap().get_material().get_type() != MaterialType::Gloves {
        println!("You would burn yourself without a pair of gloves.");
        thread::sleep(Duration::new(3, 0));
        return;
    }

    while *ACTION.lock().unwrap() == Action::FIREMAKING {
        let item = pl.get_inventory_mut().get_item_mut(idx);

        let quantity = item.get_quantity();

        println!(
            "You light the {} on fire. (+{} firemaking xp) ({})",
            material.get_name(),
            material.get_firemaking_xp(),
            quantity
        );

        pl.get_inventory_mut().remove_quantity(idx, 1);
        pl.add_xp(skill, material.get_firemaking_xp());

        if quantity == 1 {
            println!("You have run out of {} to burn!", material.get_name());
            thread::sleep(Duration::new(3, 0));
            *ACTION.lock().unwrap() = Action::IDLE;
        }

        thread::sleep(Duration::from_millis(material.get_firemaking_time() as u64));
    }

    println!("You stop firemaking.");
    thread::sleep(Duration::new(3, 0));
}

pub fn firemaking_menu() {
    let mut input: u32 = 0;

    let burnable = vec![
        Material::Log,
        Material::OakLog,
        Material::BirchLog,
        Material::TreeSpiritRemains,
    ];

    while input as usize != burnable.len() + 1 {
        clear_screen();

        println!("Firemaking Menu");
        println!("Please select what you'd like to burn:");

        let mut i = 1;
        for material in &burnable {
            println!(
                "  {}. {} ({} xp) (req: {} firemaking, 1x{})",
                i,
                material.get_name(),
                material.get_firemaking_xp(),
                material.get_firemaking_level(),
                material.get_name()
            );
            i += 1;
        }

        println!("  {}. Main Menu", i);

        print!("> ");

        input = read!();

        while input < 1 || input > i {
            println!("Invalid input. Please enter a number between 1 and {}.", i);
            print!("> ");
            input = read!();
        }

        if input == i {
            return;
        }

        let material = burnable[(input - 1) as usize];

        firemake(material);
    }
}
