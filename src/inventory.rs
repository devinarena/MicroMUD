use std::{
    io::{stdin, stdout, BufRead, Write},
    thread,
    time::Duration,
};

use serde_json::{json, Value};

use crate::{
    game::PLAYER,
    io_manager::clear_screen,
    item::{Item, Material},
};

pub struct Inventory {
    pub main_hand: Option<Item>,
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            main_hand: None,
            items: Vec::new(),
        }
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
        let mut mh = json!(null);
        if let Some(item) = &self.main_hand {
            mh = item.serialize();
        }
        json!({ "main_hand": mh, "items": output })
    }

    pub fn deserialize(&mut self, data: &Value) {
        for item in data["items"].as_array().unwrap() {
            self.items.push(Item::deserialize(item));
        }
        if data["main_hand"].is_null() {
            self.main_hand = None;
        } else {
            self.main_hand = Some(Item::deserialize(&data["main_hand"]));
        }
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn get_item(&self, index: usize) -> &Item {
        &self.items[index]
    }

    pub fn get_item_mut(&mut self, index: usize) -> &mut Item {
        &mut self.items[index]
    }

    pub fn remove_item(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn remove_quantity(&mut self, index: usize, quantity: i32) {
        let prev = self.items[index].get_quantity();

        if (prev - quantity) <= 0 {
            self.remove_item(index);
            return;
        }

        self.items[index].set_quantity(prev - quantity);
    }

    pub fn find_item_index(&self, material: Material) -> Option<usize> {
        for i in 0..self.items.len() {
            if self.items[i].get_material() == material {
                return Some(i);
            }
        }
        None
    }

    pub fn find_item(&self, material: Material) -> Option<&Item> {
        match self.find_item_index(material) {
            Some(index) => Some(&self.items[index]),
            None => None,
        }
    }

    pub fn get_main_hand(&self) -> &Option<Item> {
        &self.main_hand
    }

    pub fn get_main_hand_mut(&mut self) -> &mut Option<Item> {
        &mut self.main_hand
    }

    pub fn set_main_hand(&mut self, item: Item) {
        self.main_hand = Some(item);
    }
}

pub fn print_inventory() {
    println!("Gold: {}g", PLAYER.lock().unwrap().get_gold());
    println!("\nEquipment:");
    if let Some(item) = &PLAYER.lock().unwrap().get_inventory().get_main_hand() {
        println!(
            "Main Hand: {} x {}",
            item.get_material(),
            item.get_quantity()
        );
    } else {
        println!("Main Hand: None");
    }
    println!("\nInventory:");
    let mut index = 0;
    for item in PLAYER.lock().unwrap().get_inventory().get_items() {
        println!(
            "{}. {} x {}",
            index + 1,
            item.get_material(),
            item.get_quantity()
        );
        index += 1;
    }
}

pub fn view_inventory() {
    clear_screen();

    print_inventory();

    println!("(type 'help' for inventory commands)");

    stdout().flush().unwrap();

    let mut stdin = stdin().lock().lines();
    stdin.next().unwrap().unwrap();
    let mut input = String::new();

    while input != "back" && input != "b" {
        print!("> ");
        stdout().flush().unwrap();
        input = stdin.next().unwrap().unwrap().trim().to_string();
        let tokens = input.split_whitespace().collect::<Vec<&str>>();

        match tokens[0] {
            "help" | "h" => {
                clear_screen();
                print_inventory();
                println!("Available commands:");
                println!("help(h) - displays this message");
                println!("back(b) - returns to the main menu");
                println!("drop(d) [index] <amount> - drops an item from your inventory");
                println!(
                    "main_hand(mh) <index> - equips an item to your main hand, type no index to unequip"
                );
                println!("info(i) [index] - displays information about an item");
                println!("value(v) <index> - displays the value of an item, omit <item> to see total inventory value");
                println!("sell(s) [index] [amount|all] - sells an item from your inventory")
            }
            "drop" | "d" => {
                if tokens.len() < 2 {
                    println!("Invalid syntax. Type 'help' for a list of commands.");
                    continue;
                }
                let index = tokens[1].parse::<usize>().unwrap();
                if index > PLAYER.lock().unwrap().get_inventory().get_items().len() {
                    println!("Invalid syntax. Type 'help' for a list of commands.");
                    continue;
                }
                let mut quantity = 1;
                if tokens.len() == 3 {
                    quantity = tokens[2].parse::<i32>().unwrap();
                }
                let material = PLAYER
                    .lock()
                    .unwrap()
                    .get_inventory()
                    .get_item(index - 1)
                    .get_material();
                PLAYER
                    .lock()
                    .unwrap()
                    .get_inventory_mut()
                    .remove_quantity(index - 1, quantity);
                println!("Dropped {} x {}", quantity, material);
                thread::sleep(Duration::from_secs(1));
                clear_screen();
                print_inventory();
            }
            "main_hand" | "mh" => {
                if tokens.len() == 1 {
                    let item = PLAYER
                        .lock()
                        .unwrap()
                        .get_inventory_mut()
                        .get_main_hand_mut()
                        .take();
                    let mat = item.as_ref().unwrap().get_material();
                    if let Some(item) = item {
                        PLAYER.lock().unwrap().get_inventory_mut().add_item(item);
                    }
                    println!("Unequipped {} from main hand.", mat);
                    thread::sleep(Duration::from_secs(1));
                    clear_screen();
                    print_inventory();
                    continue;
                }

                let index = tokens[1].parse::<usize>().unwrap();

                let mut player = PLAYER.lock().unwrap();

                let res = player.equip(index - 1);

                if res == "" {
                    println!(
                        "Equipped {} in main hand",
                        player
                            .get_inventory()
                            .get_main_hand()
                            .as_ref()
                            .unwrap()
                            .get_material()
                    );
                    drop(player);
                    thread::sleep(Duration::from_secs(1));
                    clear_screen();
                    print_inventory();
                } else {
                    println!("{}", res);
                }
            }
            "info" | "i" => {
                if tokens.len() != 2 {
                    println!("Invalid syntax. Type 'help' for a list of commands.");
                    continue;
                }
                let index = tokens[1].parse::<usize>().unwrap();
                let player = PLAYER.lock().unwrap();
                if index > player.get_inventory().get_items().len() {
                    println!("Invalid syntax. Type 'help' for a list of commands.");
                    continue;
                }
                let item = player.get_inventory().get_item(index - 1);
                item.get_material().print_info();
            }
            "value" | "v" => {
                let mut total: u64 = 0;
                if tokens.len() == 1 {
                    for item in PLAYER.lock().unwrap().get_inventory().get_items() {
                        total += item.get_value();
                    }
                    println!("Total inventory value: {}g", total);
                } else {
                    let index = tokens[1].parse::<usize>().unwrap();
                    if index > PLAYER.lock().unwrap().get_inventory().get_items().len() {
                        println!("Invalid syntax. Type 'help' for a list of commands.");
                        continue;
                    }
                    let player = PLAYER.lock().unwrap();
                    let item = player.get_inventory().get_item(index - 1);
                    println!(
                        "{}: {}g x {} = {}g",
                        item.get_material().get_name(),
                        item.get_material().get_value(),
                        item.get_quantity(),
                        item.get_value()
                    );
                }
            }
            "sell" | "s" => {
                if tokens.len() < 3 {
                    println!("Invalid syntax. Type 'help' for a list of commands.");
                    continue;
                }
                let index = tokens[1].parse::<usize>().unwrap();
                if index > PLAYER.lock().unwrap().get_inventory().get_items().len() {
                    println!("Invalid syntax. Type 'help' for a list of commands.");
                    continue;
                }
                let quantity = match tokens[2] {
                    "all" => PLAYER
                        .lock()
                        .unwrap()
                        .get_inventory()
                        .get_item(index - 1)
                        .get_quantity(),
                    _ => tokens[2].parse::<i32>().unwrap(),
                };
                let item = PLAYER
                    .lock()
                    .unwrap()
                    .get_inventory_mut()
                    .get_item_mut(index - 1)
                    .clone();
                PLAYER
                    .lock()
                    .unwrap()
                    .get_inventory_mut()
                    .remove_quantity(index - 1, quantity);
                let value = item.get_material().get_value() * quantity as u64;
                PLAYER.lock().unwrap().add_gold(value);
                println!("Sold {} x {} for {}g", quantity, item.get_material(), value);
                thread::sleep(Duration::from_secs(1));
                clear_screen();
                print_inventory();
            }
            _ => {
                println!("Invalid command. Type 'help' for a list of commands.");
            }
        }
    }
}
