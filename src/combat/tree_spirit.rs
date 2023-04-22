use crate::item::{Material, Item};

use super::monster::MonsterData;


pub struct TreeSpirit {}

impl TreeSpirit {
    pub fn new() -> TreeSpirit {
        Rat {}
    }
}

impl MonsterData for TreeSpirit {
    fn get_name(&self) -> String {
        "Tree Spirit".to_string()
    }

    fn get_max_hp(&self) -> u32 {
        50
    }

    fn get_attack_style(&self) -> String {
        "magic".to_string()
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

    fn get_xp(&self) -> u64 {
        10
    }

    fn get_drops(&self) -> Vec<(Item, f32)> {
        vec![(Item::new(Material::Log, 1).clone(), 1.0)]
    }
}