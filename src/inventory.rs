use serde_json::{json, Value};

use crate::item::Item;

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
