use rand::random;

use crate::item::{Item, Material};

use super::MonsterData;

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

    fn get_hitpoints(&self) -> u32 {
        1
    }

    fn get_defence(&self) -> u32 {
        1
    }

    fn get_drops(&self) -> Vec<(Material, u32, u32, f32)> {
        vec![(Material::Log, 1, 1, 0.1)]
    }

    fn get_gold(&self) -> u64 {
        random::<u64>() % 10
    }
}
