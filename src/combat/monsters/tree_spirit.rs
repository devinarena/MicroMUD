use rand::random;

use crate::{
    item::{Item, Material},
    player::Player,
};

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

    fn get_melee(&self) -> u64 {
        5
    }

    fn get_ranged(&self) -> u64 {
        5
    }

    fn get_magic(&self) -> u64 {
        5
    }

    fn get_hitpoints(&self) -> u64 {
        5
    }

    fn get_defense(&self) -> u64 {
        5
    }

    fn get_drops(&self) -> Vec<(Material, u32, u32, f32)> {
        vec![
            (Material::TreeSpiritRemains, 1, 1, 1.0),
            (Material::Log, 2, 4, 0.5),
            (Material::OakLog, 1, 1, 0.25),
            (Material::BirchLog, 1, 1, 0.2),
            (Material::BronzeAxe, 1, 1, 0.1),
        ]
    }

    fn get_gold(&self) -> u64 {
        random::<u64>() % 50
    }

    fn get_reqs(&self) -> String {
        "woodcutting: 10".to_string()
    }

    fn can_fight(&self, player: &Player) -> String {
        if player.get_level(&"woodcutting".to_string()) < 10 {
            "You need a woodcutting level of 10 to fight this monster.".to_string()
        } else {
            "".to_string()
        }
    }
}
