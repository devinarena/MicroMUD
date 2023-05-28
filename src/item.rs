use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub mod armor;
pub mod food;
pub mod weapon;

#[derive(PartialEq)]
pub enum MaterialType {
    Log,
    Food,
    Axe,
    Weapon,
    Shield,
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Gloves,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Material {
    // Woodcutting
    Log,
    OakLog,
    BirchLog,
    Apple,
    // Equipment
    WoodenAxe,
    WoodenDagger,
    WoodenSword,
    WoodenShield,
    WoodenHelmet,
    WoodenChestplate,
    WoodenLeggings,
    WoodenBoots,
    LeatherGloves,
    // IronAxe,
    // SteelAxe,
    // GoldenAxe,
    // CrystalAxe,
    // EnchantedAxe,
    // Monsters
    TreeSpiritRemains,
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
            Material::WoodenHelmet => "Wooden Helmet".to_string(),
            Material::WoodenChestplate => "Wooden Chestplate".to_string(),
            Material::WoodenLeggings => "Wooden Leggings".to_string(),
            Material::WoodenBoots => "Wooden Boots".to_string(),
            Material::LeatherGloves => "Leather Gloves".to_string(),
            Material::TreeSpiritRemains => "Tree Spirit Remains".to_string(),
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
            Material::WoodenShield => MaterialType::Shield,
            Material::WoodenHelmet => MaterialType::Helmet,
            Material::WoodenChestplate => MaterialType::Chestplate,
            Material::WoodenLeggings => MaterialType::Leggings,
            Material::WoodenBoots => MaterialType::Boots,
            Material::LeatherGloves => MaterialType::Gloves,
            Material::TreeSpiritRemains => MaterialType::Log,
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
            Material::WoodenHelmet => 100,
            Material::WoodenChestplate => 150,
            Material::WoodenLeggings => 125,
            Material::WoodenBoots => 100,
            Material::LeatherGloves => 100,
            Material::TreeSpiritRemains => 200,
        }
    }

    pub fn get_required_level_equip(&self) -> (&str, u64) {
        match self {
            _ => ("melee", 1),
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
                println!("Apples are gathered from woodcutting and can be eaten to heal.\nWhen eaten:\n\tHealth: +10")
            }
            Material::WoodenAxe => {
                println!("Wooden axe is a basic tool used to gather logs from trees.\nWhen equipped:\n\tWoodcutting: +1\n\tAttack: +1")
            }
            Material::WoodenDagger => {
                println!("Wooden dagger is a basic wooden weapon used to fight monsters.\nWhen equipped:\n\tAttack: +1")
            }
            Material::WoodenSword => {
                println!("Wooden sword is a better wooden weapon used to fight monsters.\nWhen equipped:\n\tAttack: +2")
            }
            Material::WoodenShield => {
                println!("Wooden shield is a basic wooden weapon used to protect against monsters.\nWhen equipped:\n\tDefense: +1")
            }
            Material::WoodenHelmet => {
                println!("Wooden helmet is a basic wooden armor used to protect against monsters.\nWhen equipped:\n\tDefense: +1")
            }
            Material::WoodenChestplate => {
                println!("Wooden chestplate is a basic wooden armor used to protect against monsters.\nWhen equipped:\n\tDefense: +2")
            }
            Material::WoodenLeggings => {
                println!("Wooden leggings are a basic wooden armor used to protect against monsters.\nWhen equipped:\n\tDefense: +1")
            }
            Material::WoodenBoots => {
                println!("Wooden boots are a basic wooden armor used to protect against monsters.\nWhen equipped:\n\tDefense: +1")
            }
            Material::LeatherGloves => {
                println!("Leather gloves can be worn on the hands to firemake.\nWhen equipped:\n\tDefense: +1");
            }
            Material::TreeSpiritRemains => {
                println!("Tree Spirit Remains are gathered from Tree Spirits and can be used in firemaking.")
            }
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
            "WoodenHelmet" => Ok(Material::WoodenHelmet),
            "WoodenChestplate" => Ok(Material::WoodenChestplate),
            "WoodenLeggings" => Ok(Material::WoodenLeggings),
            "WoodenBoots" => Ok(Material::WoodenBoots),
            "LeatherGloves" => Ok(Material::LeatherGloves),
            "TreeSpiritRemains" => Ok(Material::TreeSpiritRemains),
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
