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
    pub off_hand: Option<Item>,
    pub helmet: Option<Item>,
    pub chestplate: Option<Item>,
    pub leggings: Option<Item>,
    pub boots: Option<Item>,
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            main_hand: None,
            off_hand: None,
            helmet: None,
            chestplate: None,
            leggings: None,
            boots: None,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        if let Some(equipped) = &mut self.main_hand {
            if equipped.get_material() == item.get_material() {
                equipped.add_quantity(item.get_quantity());
                return;
            }
        }
        if let Some(equipped) = &mut self.off_hand {
            if equipped.get_material() == item.get_material() {
                equipped.add_quantity(item.get_quantity());
                return;
            }
        }
        if let Some(equipped) = &mut self.helmet {
            if equipped.get_material() == item.get_material() {
                equipped.add_quantity(item.get_quantity());
                return;
            }
        }
        if let Some(equipped) = &mut self.chestplate {
            if equipped.get_material() == item.get_material() {
                equipped.add_quantity(item.get_quantity());
                return;
            }
        }
        if let Some(equipped) = &mut self.leggings {
            if equipped.get_material() == item.get_material() {
                equipped.add_quantity(item.get_quantity());
                return;
            }
        }
        if let Some(equipped) = &mut self.boots {
            if equipped.get_material() == item.get_material() {
                equipped.add_quantity(item.get_quantity());
                return;
            }
        }
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
        let mut oh = json!(null);
        if let Some(item) = &self.off_hand {
            oh = item.serialize();
        }
        let mut helm = json!(null);
        if let Some(item) = &self.helmet {
            helm = item.serialize();
        }
        let mut chest = json!(null);
        if let Some(item) = &self.chestplate {
            chest = item.serialize();
        }
        let mut legs = json!(null);
        if let Some(item) = &self.leggings {
            legs = item.serialize();
        }
        let mut boots = json!(null);
        if let Some(item) = &self.boots {
            boots = item.serialize();
        }
        json!({ "main_hand": mh, 
                "off_hand": oh,
                "helmet": helm,
                "chestplate": chest,
                "leggings": legs,
                "boots": boots,
                "items": output  })
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
        if data["off_hand"].is_null() {
            self.off_hand = None;
        } else {
            self.off_hand = Some(Item::deserialize(&data["off_hand"]));
        }
        if data["helmet"].is_null() {
            self.helmet = None;
        } else {
            self.helmet = Some(Item::deserialize(&data["helmet"]));
        }
        if data["chestplate"].is_null() {
            self.chestplate = None;
        } else {
            self.chestplate = Some(Item::deserialize(&data["chestplate"]));
        }
        if data["leggings"].is_null() {
            self.leggings = None;
        } else {
            self.leggings = Some(Item::deserialize(&data["leggings"]));
        }
        if data["boots"].is_null() {
            self.boots = None;
        } else {
            self.boots = Some(Item::deserialize(&data["boots"]));
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

    pub fn set_main_hand(&mut self, item: Option<Item>) {
        self.main_hand = item;
    }

    pub fn get_off_hand(&self) -> &Option<Item> {
        &self.off_hand
    }

    pub fn get_off_hand_mut(&mut self) -> &mut Option<Item> {
        &mut self.off_hand
    }

    pub fn set_off_hand(&mut self, item: Option<Item>) {
        self.off_hand = item;
    }

    pub fn get_helmet(&self) -> &Option<Item> {
        &self.helmet
    }

    pub fn get_helmet_mut(&mut self) -> &mut Option<Item> {
        &mut self.helmet
    }

    pub fn set_helmet(&mut self, item: Option<Item>) {
        self.helmet = item;
    }

    pub fn get_chestplate(&self) -> &Option<Item> {
        &self.chestplate
    }

    pub fn get_chestplate_mut(&mut self) -> &mut Option<Item> {
        &mut self.chestplate
    }

    pub fn set_chestplate(&mut self, item: Option<Item>) {
        self.chestplate = item;
    }

    pub fn get_leggings(&self) -> &Option<Item> {
        &self.leggings
    }

    pub fn get_leggings_mut(&mut self) -> &mut Option<Item> {
        &mut self.leggings
    }

    pub fn set_leggings(&mut self, item: Option<Item>) {
        self.leggings = item;
    }

    pub fn get_boots(&self) -> &Option<Item> {
        &self.boots
    }

    pub fn get_boots_mut(&mut self) -> &mut Option<Item> {
        &mut self.boots
    }

    pub fn set_boots(&mut self, item: Option<Item>) {
        self.boots = item;
    }
}

pub fn print_inventory() {
    let player = PLAYER.lock().unwrap();
    println!("Gold: {}g", player.get_gold());
    println!("\nEquipment:");
    if let Some(item) = &player.get_inventory().get_main_hand() {
        println!(
            "  Main Hand: {} x {}",
            item.get_material(),
            item.get_quantity()
        );
    } else {
        println!("  Main Hand: None");
    }
    if let Some(item) = &player.get_inventory().get_off_hand() {
        println!(
            "  Off Hand: {} x {}",
            item.get_material(),
            item.get_quantity()
        );
    } else {
        println!("  Off Hand: None");
    }
    if let Some(item) = &player.get_inventory().get_helmet() {
        println!("  Helmet: {} x {}", item.get_material(), item.get_quantity());
    } else {
        println!("  Helmet: None");
    }
    if let Some(item) = &player.get_inventory().get_chestplate() {
        println!(
            "  Chestplate: {} x {}",
            item.get_material(),
            item.get_quantity()
        );
    } else {
        println!("  Chestplate: None");
    }
    if let Some(item) = &player.get_inventory().get_leggings() {
        println!(
            "  Leggings: {} x {}",
            item.get_material(),
            item.get_quantity()
        );
    } else {
        println!("  Leggings: None");
    }
    if let Some(item) = &player.get_inventory().get_boots() {
        println!("  Boots: {} x {}", item.get_material(), item.get_quantity());
    } else {
        println!("  Boots: None");
    }

    println!("\nAttack Bonus: {}", player.get_attack_bonus() - player.get_level(&"melee".to_string()));
    println!("Defense Bonus: {}", player.get_defense_bonus() - player.get_level(&"defense".to_string()));

    println!("\nInventory:");
    let mut index = 0;
    for item in player.get_inventory().get_items() {
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
                println!(
                    "off_hand(oh) <index> - equips an item to your off hand, type no index to unequip"
                );
                println!(
                    "helmet(hm) <index> - equips an item to your helmet, type no index to unequip"
                );
                println!(
                    "chestplate(cp) <index> - equips an item to your chestplate, type no index to unequip"
                );
                println!(
                    "leggings(lg) <index> - equips an item to your leggings, type no index to unequip"
                );
                println!(
                    "boots(bt) <index> - equips an item to your boots, type no index to unequip"
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

                let res = player.equip(index - 1, "main_hand");

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
            "off_hand" | "oh" => {
                if tokens.len() == 1 {
                    let item = PLAYER
                        .lock()
                        .unwrap()
                        .get_inventory_mut()
                        .get_off_hand_mut()
                        .take();
                    let mat = item.as_ref().unwrap().get_material();
                    if let Some(item) = item {
                        PLAYER.lock().unwrap().get_inventory_mut().add_item(item);
                    }
                    println!("Unequipped {} from off hand.", mat);
                    thread::sleep(Duration::from_secs(1));
                    clear_screen();
                    print_inventory();
                    continue;
                }

                let index = tokens[1].parse::<usize>().unwrap();

                let mut player = PLAYER.lock().unwrap();

                let res = player.equip(index - 1, "off_hand");

                if res == "" {
                    println!(
                        "Equipped {} in off hand",
                        player
                            .get_inventory()
                            .get_off_hand()
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
            "helmet" | "hm" => {
                if tokens.len() == 1 {
                    let item = PLAYER
                        .lock()
                        .unwrap()
                        .get_inventory_mut()
                        .get_helmet_mut()
                        .take();
                    let mat = item.as_ref().unwrap().get_material();
                    if let Some(item) = item {
                        PLAYER.lock().unwrap().get_inventory_mut().add_item(item);
                    }
                    println!("Unequipped {} from helm.", mat);
                    thread::sleep(Duration::from_secs(1));
                    clear_screen();
                    print_inventory();
                    continue;
                }

                let index = tokens[1].parse::<usize>().unwrap();

                let mut player = PLAYER.lock().unwrap();

                let res = player.equip(index - 1, "helmet");

                if res == "" {
                    println!(
                        "Equipped {} in helm",
                        player
                            .get_inventory()
                            .get_helmet()
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
            "chestplate" | "cp" => {
                if tokens.len() == 1 {
                    let item = PLAYER
                        .lock()
                        .unwrap()
                        .get_inventory_mut()
                        .get_chestplate_mut()
                        .take();
                    let mat = item.as_ref().unwrap().get_material();
                    if let Some(item) = item {
                        PLAYER.lock().unwrap().get_inventory_mut().add_item(item);
                    }
                    println!("Unequipped {} from chestplate.", mat);
                    thread::sleep(Duration::from_secs(1));
                    clear_screen();
                    print_inventory();
                    continue;
                }

                let index = tokens[1].parse::<usize>().unwrap();

                let mut player = PLAYER.lock().unwrap();

                let res = player.equip(index - 1, "chestplate");

                if res == "" {
                    println!(
                        "Equipped {} in chestplate",
                        player
                            .get_inventory()
                            .get_chestplate()
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
            "leggings" | "lg" => {
                if tokens.len() == 1 {
                    let item = PLAYER
                        .lock()
                        .unwrap()
                        .get_inventory_mut()
                        .get_leggings_mut()
                        .take();
                    let mat = item.as_ref().unwrap().get_material();
                    if let Some(item) = item {
                        PLAYER.lock().unwrap().get_inventory_mut().add_item(item);
                    }
                    println!("Unequipped {} from leggings.", mat);
                    thread::sleep(Duration::from_secs(1));
                    clear_screen();
                    print_inventory();
                    continue;
                }

                let index = tokens[1].parse::<usize>().unwrap();

                let mut player = PLAYER.lock().unwrap();

                let res = player.equip(index - 1, "leggings");

                if res == "" {
                    println!(
                        "Equipped {} in leggings",
                        player
                            .get_inventory()
                            .get_leggings()
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
            "boots" | "bt" => {
                if tokens.len() == 1 {
                    let item = PLAYER
                        .lock()
                        .unwrap()
                        .get_inventory_mut()
                        .get_boots_mut()
                        .take();
                    let mat = item.as_ref().unwrap().get_material();
                    if let Some(item) = item {
                        PLAYER.lock().unwrap().get_inventory_mut().add_item(item);
                    }
                    println!("Unequipped {} from boots.", mat);
                    thread::sleep(Duration::from_secs(1));
                    clear_screen();
                    print_inventory();
                    continue;
                }

                let index = tokens[1].parse::<usize>().unwrap();

                let mut player = PLAYER.lock().unwrap();

                let res = player.equip(index - 1, "boots");

                if res == "" {
                    println!(
                        "Equipped {} in boots",
                        player
                            .get_inventory()
                            .get_boots()
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
