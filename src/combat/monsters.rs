use crate::item::{Item, Material};

pub mod rat;
pub mod tree_spirit;
pub trait MonsterData {
    fn get_name(&self) -> String;
    fn get_attack_style(&self) -> String;
    fn get_melee(&self) -> u32;
    fn get_ranged(&self) -> u32;
    fn get_magic(&self) -> u32;
    fn get_hitpoints(&self) -> u32;
    fn get_defence(&self) -> u32;
    fn get_drops(&self) -> Vec<(Material, u32, u32, f32)>;
    fn get_gold(&self) -> u64;
}
