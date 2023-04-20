use serde_json::{json, Value};
use text_io::read;

use crate::{item::Item, game::{PLAYER, game_loop}, startup::main_menu};

pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Item) {
        for i in 0..self.items.len() {
            if self.items[i].get_material() == item.get_material() {
                self.items[i].add_quantity(item.get_quantity());
                return;
            }
        }
        self.items.push(item);
    }

    pub fn serialize(&self) -> Value {
        let mut output = Vec::new();
        for item in &self.items {
            output.push(item.serialize());
        }
        json!({ "items": output })
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }
}

pub fn view_inventory() {
    println!("Inventory:");
    for item in PLAYER.lock().unwrap().get_inventory().get_items() {
        println!("{} x{}", item.get_material(), item.get_quantity());
    }

    println!("Type anything or press enter to continue: ");
    let input: String = read!();

    game_loop();
}