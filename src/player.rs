use std::{collections::HashMap, fmt::Display};

use serde_json::{json, Value};

use crate::{
    combat::ability::Ability,
    inventory::Inventory,
    item::{Item, Material},
};

#[derive(Clone, PartialEq)]
pub enum Action {
    EXITING,
    IDLE,
    CHOPPING,
    FIREMAKING,
    COMBAT,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::EXITING => write!(f, "exiting"),
            Action::IDLE => write!(f, "idle"),
            Action::CHOPPING => write!(f, "chopping"),
            Action::FIREMAKING => write!(f, "firemaking"),
            Action::COMBAT => write!(f, "combat"),
        }
    }
}

pub struct Player {
    name: String,
    xp: HashMap<String, u64>,
    inventory: Inventory,
    gold: u64,
    health: i32,
    location: String,
    abilities: Vec<Ability>,
}

impl Player {
    pub fn new(name: String) -> Player {
        let mut player = Player {
            name,
            xp: HashMap::new(),
            inventory: Inventory::new(),
            gold: 0,
            health: 100,
            location: "Littlewood Town".to_string(),
            abilities: vec![],
        };

        player.xp.insert("hitpoints".to_string(), 0);
        player.xp.insert("melee".to_string(), 0);
        player.xp.insert("defense".to_string(), 0);
        player.xp.insert("ranged".to_string(), 0);
        player.xp.insert("magic".to_string(), 0);
        player.xp.insert("mining".to_string(), 0);
        player.xp.insert("smithing".to_string(), 0);
        player.xp.insert("woodcutting".to_string(), 0);
        player.xp.insert("firemaking".to_string(), 0);
        player.xp.insert("fishing".to_string(), 0);
        player.xp.insert("cooking".to_string(), 0);
        player.xp.insert("farming".to_string(), 0);

        return player;
    }

    pub fn empty() -> Player {
        Player {
            name: "".to_string(),
            xp: HashMap::new(),
            inventory: Inventory::new(),
            gold: 0,
            health: 0,
            location: "".to_string(),
            abilities: vec![],
        }
    }

    pub fn deserialize(json: &Value) -> Player {
        let name = json["name"].as_str().unwrap().to_string();
        let mut player = Player {
            name,
            xp: HashMap::new(),
            inventory: Inventory::new(),
            gold: json["gold"].as_u64().unwrap(),
            health: 100,
            location: "Littlewood Town".to_string(),
            abilities: vec![],
        };

        player.xp.insert(
            "hitpoints".to_string(),
            json["xp"]["hitpoints"].as_u64().unwrap(),
        );
        player
            .xp
            .insert("melee".to_string(), json["xp"]["melee"].as_u64().unwrap());
        player.xp.insert(
            "defense".to_string(),
            json["xp"]["defense"].as_u64().unwrap(),
        );
        player
            .xp
            .insert("ranged".to_string(), json["xp"]["ranged"].as_u64().unwrap());
        player
            .xp
            .insert("magic".to_string(), json["xp"]["magic"].as_u64().unwrap());
        player
            .xp
            .insert("mining".to_string(), json["xp"]["mining"].as_u64().unwrap());
        player.xp.insert(
            "smithing".to_string(),
            json["xp"]["smithing"].as_u64().unwrap(),
        );
        player.xp.insert(
            "woodcutting".to_string(),
            json["xp"]["woodcutting"].as_u64().unwrap(),
        );
        player.xp.insert(
            "firemaking".to_string(),
            json["xp"]["firemaking"].as_u64().unwrap(),
        );
        player.xp.insert(
            "fishing".to_string(),
            json["xp"]["fishing"].as_u64().unwrap(),
        );
        player.xp.insert(
            "cooking".to_string(),
            json["xp"]["cooking"].as_u64().unwrap(),
        );

        player.inventory.deserialize(&json["inventory"]);

        player.set_health((player.get_level(&"hitpoints".to_string()) * 100) as i32);

        player.register_abilities();

        return player;
    }

    pub fn serialize(&self) -> String {
        json!({
            "name": self.name,
            "xp": {
                "hitpoints": self.xp["hitpoints"],
                "melee": self.xp["melee"],
                "defense": self.xp["defense"],
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
        self.get_level(&"hitpoints".to_string()) as i32 * 100
    }

    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }

    pub fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn get_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn get_skills(&self) -> &HashMap<String, u64> {
        &self.xp
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_xp(&self, skill: &String) -> u64 {
        self.xp[skill]
    }

    pub fn get_level(&self, skill: &String) -> u64 {
        let mut xp = self.xp[skill] as f64;

        let mut level: u64 = 1;

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

    pub fn needed_xp(&self, skill: &String) -> u64 {
        let level = self.get_level(skill);
        let mut next_xp = 0;

        for i in 1..level + 1 {
            let needed_xp = self._needed_xp_l(i);
            next_xp += needed_xp;
        }

        return next_xp;
    }

    fn _needed_xp_l(&self, level: u64) -> u64 {
        let needed_xp = (150.0 * 1.75_f64.powf((level - 1) as f64 / 8.0) / 4.7) as u64;
        return needed_xp;
    }

    pub fn add_xp(&mut self, skill: &String, xp: u64) {
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

    pub fn get_attack_bonus(&self) -> u64 {
        let mut attack_bonus: u64 = self.get_level(&"melee".to_string());

        if let Some(item) = self.inventory.get_main_hand() {
            attack_bonus += item.get_material().get_melee_bonus();
        }
        if let Some(item) = self.inventory.get_off_hand() {
            attack_bonus += item.get_material().get_attack_bonus();
        }
        if let Some(item) = self.inventory.get_helmet() {
            attack_bonus += item.get_material().get_attack_bonus();
        }
        if let Some(item) = self.inventory.get_chestplate() {
            attack_bonus += item.get_material().get_attack_bonus();
        }
        if let Some(item) = self.inventory.get_leggings() {
            attack_bonus += item.get_material().get_attack_bonus();
        }
        if let Some(item) = self.inventory.get_boots() {
            attack_bonus += item.get_material().get_attack_bonus();
        }

        return attack_bonus;
    }

    pub fn get_defense_bonus(&self) -> u64 {
        let mut defense_bonus: u64 = self.get_level(&"defense".to_string());

        if let Some(item) = self.inventory.get_main_hand() {
            defense_bonus += item.get_material().get_defense_bonus();
        }
        if let Some(item) = self.inventory.get_off_hand() {
            defense_bonus += item.get_material().get_defense_bonus();
        }
        if let Some(item) = self.inventory.get_helmet() {
            defense_bonus += item.get_material().get_defense_bonus();
        }
        if let Some(item) = self.inventory.get_chestplate() {
            defense_bonus += item.get_material().get_defense_bonus();
        }
        if let Some(item) = self.inventory.get_leggings() {
            defense_bonus += item.get_material().get_defense_bonus();
        }
        if let Some(item) = self.inventory.get_boots() {
            defense_bonus += item.get_material().get_defense_bonus();
        }

        return defense_bonus;
    }

    pub fn get_combat_level(&self) -> u64 {
        let mut combat_level: u64 = 0;
        combat_level += self.get_level(&"melee".to_string());
        combat_level += self.get_level(&"defense".to_string());
        combat_level += self.get_level(&"ranged".to_string());
        combat_level += self.get_level(&"magic".to_string());
        combat_level += self.get_level(&"hitpoints".to_string());
        return combat_level / 5;
    }

    pub fn get_abilities(&self) -> &Vec<Ability> {
        &self.abilities
    }

    pub fn equip(&mut self, index: usize, slot: &str) -> String {
        let item = self.inventory.get_item(index).clone();

        let mat = item.get_material();
        let req = mat.get_required_level_equip();

        if mat.get_slot() == "none" {
            return format!("You cannot equip {}.", mat.get_name());
        } else if mat.get_slot() != slot {
            return format!(
                "{} must be equipped in your {}.",
                mat.get_name(),
                mat.get_slot()
            );
        }

        if self.get_level(&req.0.to_string()) < req.1 {
            return format!("You need level {} {} to equip this item.", req.1, req.0);
        }

        self.inventory.remove_item(index);

        let old = match slot {
            "helmet" => self.inventory.get_helmet().clone(),
            "chestplate" => self.inventory.get_chestplate().clone(),
            "leggings" => self.inventory.get_leggings().clone(),
            "boots" => self.inventory.get_boots().clone(),
            "main_hand" => self.inventory.get_main_hand().clone(),
            "off_hand" => self.inventory.get_off_hand().clone(),
            _ => return format!("{} is not a valid slot.", slot),
        };

        match slot {
            "helmet" => self.inventory.set_helmet(Some(item)),
            "chestplate" => self.inventory.set_chestplate(Some(item)),
            "leggings" => self.inventory.set_leggings(Some(item)),
            "boots" => self.inventory.set_boots(Some(item)),
            "main_hand" => self.inventory.set_main_hand(Some(item)),
            "off_hand" => self.inventory.set_off_hand(Some(item)),
            _ => return format!("{} is not a valid slot.", slot),
        }

        if let Some(i) = old {
            println!("Adding {} to inventory.", i.get_material().get_name());
            self.inventory.add_item(i);
        }

        String::new()
    }

    pub fn register_abilities(&mut self) {
        self.abilities.push(Ability::new(
            "Backhand".to_string(),
            "Forcefully backhand your opponent, dealing a guaranteed 15 melee damage.".to_string(),
            "melee".to_string(),
            1,
            0.1,
            |_, _, _, _, monster, ehealth, _, _, _, _, _| -> () {
                *ehealth -= 15;
                println!(
                    "Your backhand deals 15 damage to {}!",
                    monster.get_name()
                );
            }            
        ))
    }

    pub fn print_stats(&self) {
        println!("Name: {}", self.name);
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
