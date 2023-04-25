use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub mod armor;
pub mod weapon;

#[derive(PartialEq)]
pub enum MaterialType {
    Log,
    Food,
    Axe,
    Weapon,
    Gloves,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Material {
    Log,
    OakLog,
    BirchLog,
    Apple,
    WoodenAxe,
    WoodenDagger,
    WoodenSword,
    WoodenShield,
    BronzeAxe,
    LeatherGloves,
    // IronAxe,
    // SteelAxe,
    // GoldenAxe,
    // DiamondAxe,
    // DragonAxe,
}

impl Material {
    pub fn get_name(&self) -> String {
        match self {
            Material::Log => "Logs".to_string(),
            Material::OakLog => "Oak Logs".to_string(),
            Material::BirchLog => "Birch Logs".to_string(),
            Material::Apple => "Apple".to_string(),
            Material::WoodenAxe => "Wooden Axe".to_string(),
            Material::WoodenDagger => "Wooden Dagger".to_string(),
            Material::WoodenSword => "Wooden Sword".to_string(),
            Material::WoodenShield => "Wooden Shield".to_string(),
            Material::BronzeAxe => "Bronze Axe".to_string(),
            Material::LeatherGloves => "Leather Gloves".to_string(),
        }
    }

    pub fn get_type(&self) -> MaterialType {
        match self {
            Material::Log => MaterialType::Log,
            Material::OakLog => MaterialType::Log,
            Material::BirchLog => MaterialType::Log,
            Material::Apple => MaterialType::Food,
            Material::WoodenAxe => MaterialType::Axe,
            Material::WoodenDagger => MaterialType::Weapon,
            Material::WoodenSword => MaterialType::Weapon,
            Material::WoodenShield => MaterialType::Weapon,
            Material::BronzeAxe => MaterialType::Axe,
            Material::LeatherGloves => MaterialType::Gloves,
        }
    }

    pub fn get_value(&self) -> u64 {
        match self {
            Material::Log => 5,
            Material::OakLog => 20,
            Material::BirchLog => 22,
            Material::Apple => 25,
            Material::WoodenAxe => 50,
            Material::WoodenDagger => 50,
            Material::WoodenSword => 75,
            Material::WoodenShield => 75,
            Material::BronzeAxe => 250,
            Material::LeatherGloves => 100,
        }
    }

    pub fn get_required_level_equip(&self) -> (&str, u64) {
        match self {
            Material::BronzeAxe => ("woodcutting", 100),
            _ => ("melee", 1),
        }
    }

    pub fn get_food_heal(&self) -> u32 {
        match self {
            Material::Apple => 10,
            _ => 0,
        }
    }

    pub fn print_info(&self) {
        match self {
            Material::Log => {
                println!("Logs are gathered from trees and can be used in firemaking.")
            }
            Material::OakLog => {
                println!("Oak Logs are gathered from oak trees and can be used in firemaking.")
            }
            Material::BirchLog => {
                println!("Birch Logs are gathered from birch trees and can be used in firemaking.")
            }
            Material::Apple => {
                println!("Apples are gathered from woodcutting and can be eaten to heal.")
            }
            Material::WoodenAxe => {
                println!("Wooden axe is a basic tool used to gather logs from trees.")
            }
            Material::WoodenDagger => {
                println!("Wooden dagger is a basic wooden weapon used to fight monsters.")
            }
            Material::WoodenSword => {
                println!("Wooden sword is a better wooden weapon used to fight monsters.")
            }
            Material::WoodenShield => {
                println!("Wooden shield is a basic wooden weapon used to protect against monsters.")
            }
            Material::BronzeAxe => {
                println!("Bronze axe is a slightly better tool used to gather logs from trees.")
            }
            Material::LeatherGloves => {
                println!("Leather gloves can be worn on the hands to firemake.");
            }
            _ => println!("No description"),
        }
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", self.get_name()),
        }
    }
}

impl FromStr for Material {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Log" => Ok(Material::Log),
            "OakLog" => Ok(Material::OakLog),
            "BirchLog" => Ok(Material::BirchLog),
            "Apple" => Ok(Material::Apple),
            "WoodenAxe" => Ok(Material::WoodenAxe),
            "WoodenDagger" => Ok(Material::WoodenDagger),
            "WoodenSword" => Ok(Material::WoodenSword),
            "WoodenShield" => Ok(Material::WoodenShield),
            "BronzeAxe" => Ok(Material::BronzeAxe),
            "LeatherGloves" => Ok(Material::LeatherGloves),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct Item {
    material: Material,
    quantity: i32,
}

impl Item {
    pub fn new(material: Material, quantity: i32) -> Item {
        Item { material, quantity }
    }

    pub fn get_material(&self) -> Material {
        self.material.clone()
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn set_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
    }

    pub fn add_quantity(&mut self, quantity: i32) {
        self.quantity += quantity;
    }

    pub fn get_value(&self) -> u64 {
        self.material.get_value() * self.quantity as u64
    }

    pub fn serialize(&self) -> Value {
        json!({
            "material": self.material,
            "quantity": self.quantity,
        })
    }

    pub fn deserialize(json: &serde_json::Value) -> Item {
        Item {
            material: json["material"].as_str().unwrap().parse().unwrap(),
            quantity: json["quantity"].as_i64().unwrap() as i32,
        }
    }
}
