use crate::{item::{Material}, player::Player};

use super::FightState;

pub mod slime;
pub mod rat;
pub mod giant_rat;
pub mod tree_spirit;
pub trait MonsterData {
    fn get_name(&self) -> String;
    fn get_attack_style(&self) -> String;
    fn get_melee(&self) -> u64;
    fn get_ranged(&self) -> u64;
    fn get_magic(&self) -> u64;
    fn get_hitpoints(&self) -> u64;
    fn get_defense(&self) -> u64;
    fn get_drops(&self) -> Vec<(Material, u32, u32, f32)>;
    fn get_gold(&self) -> u64;
    fn get_reqs(&self) -> String;
    fn can_fight(&self, player: &Player) -> String;
    fn choose_ability(&self, state: &mut FightState) -> bool;
}
