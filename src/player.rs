use std::{collections::HashMap, fmt::Display};

use serde_json::{json, Value};

use crate::{
    inventory::Inventory,
    item::{Item, Material},
};

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
    gold: u64,
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
            gold: 0,
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
        player.xp.insert("firemaking".to_string(), 0);
        player.xp.insert("fishing".to_string(), 0);
        player.xp.insert("cooking".to_string(), 0);
        player.xp.insert("farming".to_string(), 0);

        player.inventory.add_item(Item::new(Material::WoodenAxe, 1));

        return player;
    }

    pub fn empty() -> Player {
        Player {
            name: "".to_string(),
            class: "".to_string(),
            xp: HashMap::new(),
            inventory: Inventory::new(),
            gold: 0,
            health: 0,
            location: "".to_string(),
        }
    }

    pub fn equip(&mut self, index: usize) -> String {
        let item = self.inventory.get_item(index).clone();

        let mat = item.get_material();
        let req = mat.get_required_level_equip();
        if self.get_level(&req.0.to_string()) < req.1 {
            return format!(
                "You need level {} {} to equip this item.",
                req.1, req.0
            );
        }

        self.inventory.remove_item(index);

        let old = self.inventory.main_hand.take();

        self.inventory.set_main_hand(item);

        if let Some(i) = old {
            self.inventory.add_item(i);
        }

        String::new()
    }

    pub fn deserialize(json: &Value) -> Player {
        let name = json["name"].as_str().unwrap().to_string();
        let class = json["class"].as_str().unwrap().to_string();
        let mut player = Player {
            name,
            class,
            xp: HashMap::new(),
            inventory: Inventory::new(),
            gold: json["gold"].as_u64().unwrap(),
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
            "firemaking".to_string(),
            json["xp"]["firemaking"].as_i64().unwrap(),
        );
        player.xp.insert(
            "fishing".to_string(),
            json["xp"]["fishing"].as_i64().unwrap(),
        );
        player.xp.insert(
            "cooking".to_string(),
            json["xp"]["cooking"].as_i64().unwrap(),
        );

        player.inventory.deserialize(&json["inventory"]);

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
                "firemaking": self.xp["firemaking"],
                "fishing": self.xp["fishing"],
                "cooking": self.xp["cooking"],
            },
            "inventory": self.inventory.serialize(),
            "gold": self.gold,
        })
        .to_string()
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn get_inventory_mut(&mut self) -> &mut Inventory {
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

    pub fn get_level(&self, skill: &String) -> u32 {
        let mut xp = self.xp[skill] as f64;

        let mut level: u32 = 1;

        while xp >= 0.0 {
            let needed_xp = self._needed_xp_l(level);
            xp -= needed_xp as f64;
            level += 1;
        }

        return level - 1;
    }

    pub fn get_gold(&self) -> u64 {
        self.gold
    }

    pub fn needed_xp(&self, skill: &String) -> i64 {
        let level = self.get_level(skill);
        let mut next_xp = 0;

        for i in 1..level + 1 {
            let needed_xp = self._needed_xp_l(i);
            next_xp += needed_xp;
        }

        return next_xp;
    }

    fn _needed_xp_l(&self, level: u32) -> i64 {
        let needed_xp = (150.0 * 1.75_f64.powf((level - 1) as f64 / 8.0) / 4.7).floor() as i64;
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

    pub fn add_gold(&mut self, gold: u64) {
        self.gold += gold;
    }

    pub fn print_stats(&self) {
        println!("Name: {}", self.name);
        println!("  Class: {}", self.class);
        println!("  Location: {}", self.location);
        println!("  Health: {}", self.health);
        println!("  Gold: {}g", self.gold);
        println!("  Skills:");
        for (skill, xp) in &self.xp {
            println!(
                "    {}: {} ({} / {})",
                skill,
                self.get_level(skill),
                xp,
                self.needed_xp(skill)
            );
        }
    }
}
