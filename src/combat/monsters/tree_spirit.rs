use rand::random;

use crate::item::{Material, Item};

use super::MonsterData;

pub struct TreeSpirit {}

impl TreeSpirit {
    pub fn new() -> TreeSpirit {
        TreeSpirit {}
    }
}

impl MonsterData for TreeSpirit {
    fn get_name(&self) -> String {
        "Tree Spirit".to_string()
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

    fn get_hitpoints(&self) -> u32 {
        1
    }

    fn get_defence(&self) -> u32 {
        1
    }

    fn get_drops(&self) -> Vec<(Material, u32, u32, f32)> {
        vec![(Material::Log, 1, 3, 1.0)]
    }

    fn get_gold(&self) -> u64 {
        random::<u64>() % 10
    }
}
