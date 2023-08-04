use std::{thread, time::Duration};

use text_io::read;

use crate::{
    game,
    io_manager::clear_screen,
    item::{Item, Material},
    player::Player,
};

pub mod prices;

pub fn purchase(player: &mut Player, mat: Material) {
    println!("How many would you like to buy?");
    
    print!("> ");

    let amount: u64 = read!();

    if amount == 0 {
        println!("Not purchasing anything");
        purchase(player, mat);
        return;
    }

    let cost = mat.get_value() * amount;

    if player.get_gold() < cost {
        println!(
            "You don't have enough gold to buy {} x {}.",
            amount,
            mat.get_name()
        );
        purchase(player, mat);
        return;
    }

    player.get_inventory_mut().add_item(Item::new(mat, amount));
    println!(
        "You bought {} x {} for {} gold ({} ea.)",
        amount,
        mat.get_name(),
        cost,
        mat.get_value()
    );

    thread::sleep(Duration::from_millis(
        (2000_f32 / game::TICK_RATE as f32 * game::SPEED_SCALE) as u64,
    ));
}

pub fn weapon_menu() {
    let mut input: usize = 0;

    let items = vec![Material::WoodenDagger, Material::NoviceWand];

    while input != items.len() + 1 {
        clear_screen();

        println!("{} - Buy Weapons", game::TITLE);
        println!("Gold: {}\n", game::PLAYER.lock().unwrap().get_gold());

        let mut index = 1;

        for item in items.iter() {
            println!("{}. {} - {} gold", index, item.get_name(), item.get_value());
            index += 1;
        }
        
        print!("> ");

        input = read!();

        if input == index {
            break;
        }

        if input > 0 && input < index {
            let mut player = game::PLAYER.lock().unwrap();
            let item = items[input - 1];
            
            purchase(&mut player, item);
        } else {
            println!("Selection must be between 1 and {}", index);
        }
    }
}

pub fn buy_menu() {
    let mut input: usize = 0;

    while input != 3 {
        clear_screen();

        println!("{} - Buy", game::TITLE);
        println!("Gold: {}\n", game::PLAYER.lock().unwrap().get_gold());

        println!("1. Buy Logs");
        println!("2. Buy Weapons");
        println!("2. Buy Armor");
        println!("3. Exit");

        print!("> ");

        input = read!();

        match input {
            1 => (),
            2 => weapon_menu(),
            3 => (),
            _ => println!("Selection must be between 1 and 3"),
        }
    }
}

pub fn sell_menu() {}

pub fn shop_menu() {
    let mut input: usize = 0;

    while input != 3 {
        clear_screen();

        println!("{} - General Store", game::TITLE);
        println!("Gold: {}\n", game::PLAYER.lock().unwrap().get_gold());

        println!("1. Buy");
        println!("2. Sell");
        println!("3. Exit");

        print!("> ");

        input = read!();

        match input {
            1 => buy_menu(),
            2 => sell_menu(),
            3 => (),
            _ => println!("Selection must be between 1 and 3"),
        }
    }
}
