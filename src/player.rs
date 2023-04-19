use std::{collections::HashMap, fmt::Display};

use serde_json::{json, Value};

use crate::{inventory::Inventory, item::Item};

#[derive(Clone, PartialEq)]
pub enum Action {
    EXITING,
    IDLE,
    CHOPPING,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::EXITING => write!(f, "exiting"),
            Action::IDLE => write!(f, "idle"),
            Action::CHOPPING => write!(f, "chopping"),
        }
    }
}

pub struct Player {
    name: String,
    class: String,
    xp: HashMap<String, i64>,
    inventory: Inventory,
    health: i32,
    location: String,
}

impl Player {
    pub fn new(name: String, class: String) -> Player {
        let mut player = Player {
            name,
            class,
            xp: HashMap::new(),
            inventory: Inventory::new(),
            health: 100,
            location: "Littlewood Town".to_string(),
        };

        player.xp.insert("hitpoints".to_string(), 0);
        player.xp.insert("melee".to_string(), 0);
        player.xp.insert("ranged".to_string(), 0);
        player.xp.insert("magic".to_string(), 0);
        player.xp.insert("mining".to_string(), 0);
        player.xp.insert("smithing".to_string(), 0);
        player.xp.insert("woodcutting".to_string(), 0);
        player.xp.insert("fishing".to_string(), 0);
        player.xp.insert("cooking".to_string(), 0);
        player.xp.insert("farming".to_string(), 0);

        return player;
    }

    pub fn empty() -> Player {
        Player {
            name: "".to_string(),
            class: "".to_string(),
            xp: HashMap::new(),
            inventory: Inventory::new(),
            health: 0,
            location: "".to_string(),
        }
    }

    pub fn deserialize(json: &Value) -> Player {
        let name = json["name"].as_str().unwrap().to_string();
        let class = json["class"].as_str().unwrap().to_string();
        let mut player = Player {
            name,
            class,
            xp: HashMap::new(),
            inventory: Inventory::new(),
            health: 100,
            location: "Littlewood Town".to_string(),
        };

        player.xp.insert(
            "hitpoints".to_string(),
            json["xp"]["hitpoints"].as_i64().unwrap(),
        );
        player
            .xp
            .insert("melee".to_string(), json["xp"]["melee"].as_i64().unwrap());
        player
            .xp
            .insert("ranged".to_string(), json["xp"]["ranged"].as_i64().unwrap());
        player
            .xp
            .insert("magic".to_string(), json["xp"]["magic"].as_i64().unwrap());
        player
            .xp
            .insert("mining".to_string(), json["xp"]["mining"].as_i64().unwrap());
        player.xp.insert(
            "smithing".to_string(),
            json["xp"]["smithing"].as_i64().unwrap(),
        );
        player.xp.insert(
            "woodcutting".to_string(),
            json["xp"]["woodcutting"].as_i64().unwrap(),
        );
        player.xp.insert(
            "fishing".to_string(),
            json["xp"]["fishing"].as_i64().unwrap(),
        );
        player.xp.insert(
            "cooking".to_string(),
            json["xp"]["cooking"].as_i64().unwrap(),
        );
        player.xp.insert(
            "farming".to_string(),
            json["xp"]["farming"].as_i64().unwrap(),
        );

        for item in json["inventory"]["items"].as_array().unwrap() {
            player.inventory.add_item(Item::deserialize(item));
        }

        return player;
    }

    pub fn serialize(&self) -> String {
        json!({
            "name": self.name,
            "class": self.class,
            "xp": {
                "hitpoints": self.xp["hitpoints"],
                "melee": self.xp["melee"],
                "ranged": self.xp["ranged"],
                "magic": self.xp["magic"],
                "mining": self.xp["mining"],
                "smithing": self.xp["smithing"],
                "woodcutting": self.xp["woodcutting"],
                "fishing": self.xp["fishing"],
                "cooking": self.xp["cooking"],
                "farming": self.xp["farming"],
            },
            "inventory": self.inventory.serialize(),
        })
        .to_string()
    }

    pub fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_class(&self) -> String {
        self.class.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_xp(&self, skill: &String) -> i64 {
        self.xp[skill]
    }

    pub fn get_level(&self, skill: &String) -> i32 {
        let xp = self.xp[skill];
        let level = (1.0 + 6.0 * ((xp as f64) * 4.7 / 150.0).log2()).floor();
        return level as i32;
    }

    pub fn needed_xp(&self, skill: &String) -> i64 {
        let next_level = self.get_level(skill) + 1;
        let needed_xp = ((150.0 * 2_f32.powf((next_level - 1) as f32) / 6.0) / 4.7) as i64;
        return needed_xp;
    }
}
