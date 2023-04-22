use crate::item::{Item, Material};

pub trait MonsterData {
    fn get_name(&self) -> String;
    fn get_max_hp(&self) -> u32;
    fn get_attack_style(&self) -> String;
    fn get_melee(&self) -> u32;
    fn get_ranged(&self) -> u32;
    fn get_magic(&self) -> u32;
    fn get_drops(&self) -> Vec<(Item, f32)>;
}
