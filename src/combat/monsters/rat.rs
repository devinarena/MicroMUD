use rand::random;

use crate::{
    combat::{ability::Ability, FightState},
    item::Material,
    player::Player,
};

use super::MonsterData;

pub struct Rat {
    abilities: Vec<Ability>,
    ability_chances: Vec<f32>,
}

impl Rat {
    pub fn new() -> Rat {
        Rat {
            abilities: vec![],
            ability_chances: vec![],
        }
    }
}

impl MonsterData for Rat {
    fn get_name(&self) -> String {
        "Rat".to_string()
    }

    fn get_attack_style(&self) -> String {
        "melee".to_string()
    }

    fn get_melee(&self) -> u64 {
        1
    }

    fn get_ranged(&self) -> u64 {
        1
    }

    fn get_magic(&self) -> u64 {
        1
    }

    fn get_hitpoints(&self) -> u64 {
        1
    }

    fn get_defense(&self) -> u64 {
        1
    }

    fn get_drops(&self) -> Vec<(Material, u32, u32, f32)> {
        vec![
            (Material::Log, 1, 1, 0.2),
            (Material::WoodenBoots, 1, 1, 0.1),
            (Material::Apple, 1, 1, 0.1),
        ]
    }

    fn get_gold(&self) -> u64 {
        random::<u64>() % 10
    }

    fn get_reqs(&self) -> String {
        "".to_string()
    }

    fn can_fight(&self, _player: &Player) -> String {
        "".to_string()
    }

    fn choose_ability(&self, state: &mut FightState) -> bool {
        for _ in 0..self.abilities.len() {
            let index = random::<usize>() % self.abilities.len();
            if state.monster_adrenaline >= self.abilities[index].get_cost()
                && random::<f32>() <= self.ability_chances[index]
            {
                (self.abilities[index].activate)(state);
                return true;
            }
        }
        false
    }
}
