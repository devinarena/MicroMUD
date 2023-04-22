use crate::item::{Material, Item};

use super::monster::MonsterData;




pub struct Rat {}

impl Rat {
    pub fn new() -> Rat {
        Rat {}
    }
}

impl MonsterData for Rat {
    fn get_name(&self) -> String {
        "Rat".to_string()
    }

    fn get_max_hp(&self) -> u32 {
        30
    }

    fn get_attack_style(&self) -> String {
        "melee".to_string()
    }

    fn get_melee(&self) -> u32 {
        1
    }

    fn get_ranged(&self) -> u32 {
        1
    }

    fn get_magic(&self) -> u32 {
        1
    }

    fn get_drops(&self) -> Vec<(Item, f32)> {
        vec![(Item::new(Material::Log, 1).clone(), 1.0)]
    }
}