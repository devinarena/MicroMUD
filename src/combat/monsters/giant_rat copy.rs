use rand::random;

use crate::{
    item::{Material},
    player::Player,
};

use super::MonsterData;

pub struct GiantRat {}

impl GiantRat {
    pub fn new() -> GiantRat {
        GiantRat {}
    }
}

impl MonsterData for GiantRat {
    fn get_name(&self) -> String {
        "Giant Rat".to_string()
    }

    fn get_attack_style(&self) -> String {
        "melee".to_string()
    }

    fn get_melee(&self) -> u64 {
        3
    }

    fn get_ranged(&self) -> u64 {
        3
    }

    fn get_magic(&self) -> u64 {
        3
    }

    fn get_hitpoints(&self) -> u64 {
        3
    }

    fn get_defense(&self) -> u64 {
        3
    }

    fn get_drops(&self) -> Vec<(Material, u32, u32, f32)> {
        vec![
            (Material::Log, 1, 1, 0.25),
            (Material::OakLog, 1, 1, 0.25),
            (Material::WoodenSword, 1, 1, 0.25),
            (Material::WoodenShield, 1, 1, 0.25),
        ]
    }

    fn get_gold(&self) -> u64 {
        random::<u64>() % 30
    }

    fn get_reqs(&self) -> String {
        "".to_string()
    }

    fn can_fight(&self, _player: &Player) -> String {
        "".to_string()
    }
}
