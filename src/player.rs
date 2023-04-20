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

    pub fn get_inventory(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn get_skills(&self) -> &HashMap<String, i64> {
        &self.xp
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
        let mut xp = self.xp[skill] as f64;
        let mut level: i32 = 1;
        while xp > 0.0 {
            let needed_xp = 100.0 * 1.75_f64.powf((level - 1) as f64 / 8.0) / 4.7;
            if xp >= needed_xp as f64 {
                level += 1;
                xp -= needed_xp;
            } else {
                return level;
            }
        }

        return level;
    }

    pub fn needed_xp(&self, skill: &String) -> i64 {
        let level = self.get_level(skill);
        let mut xp = 0;
        for i in 1..level {
            xp += self._needed_xp_l(i);
        }
        return xp;
    }

    fn _needed_xp_l(&self, level: i32) -> i64 {
        let needed_xp = (100.0 * 1.75_f64.powf((level - 1) as f64 / 8.0) / 4.7) as i64;
        return needed_xp;
    }

    pub fn add_xp(&mut self, skill: &String, xp: i64) {
        let needed_xp = self.needed_xp(skill);

        let old_xp = self.xp[skill];

        self.xp.insert(skill.clone(), self.xp[skill] + xp);

        if old_xp < needed_xp && self.xp[skill] >= needed_xp {
            println!(
                "Congratulations! You have just advanced a level in {}! You are now level {}.",
                skill,
                self.get_level(skill)
            );
        }
    }
}
