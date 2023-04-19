use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Material {
    LOG,
}

impl FromStr for Material {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LOG" => Ok(Material::LOG),
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

    pub fn serialize(&self) -> String {
        json!({
            "material": self.material,
            "quantity": self.quantity,
        })
        .to_string()
    }

    pub fn deserialize(json: &serde_json::Value) -> Item {
        Item {
            material: json["material"].as_str().unwrap().parse().unwrap(),
            quantity: json["quantity"].as_i64().unwrap() as i32,
        }
    }
}
