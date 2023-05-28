use rand::random;

use crate::{
    combat::{ability::Ability, max_hit_comp, FightState},
    item::Material,
    player::Player,
};

use super::MonsterData;

pub struct TreeSpirit {
    abilities: Vec<Ability>,
    ability_chances: Vec<f32>,
}

impl TreeSpirit {
    pub fn new() -> TreeSpirit {
        TreeSpirit {
            abilities: vec![Ability::new(
                "Branch Smash".to_string(),
                String::new(),
                "melee".to_string(),
                1,
                0.25,
                |state| {
                    state.monster_adrenaline -= 0.25;
                    let max_hit = max_hit_comp(
                        state.monster_attack + 5,
                        state.player.get_bonus(&"defense".to_string()),
                    );
                    let damage = (max_hit as f32 * random::<f32>()) as i32;
                    if random::<f32>() < state.pl_crit_chance {
                        state.monster_health -= damage * 2;
                        println!(
                            "The {} smashes you with its branch, critically dealing {} damage!",
                            state.monster.get_name(),
                            damage * 2
                        );
                    } else {
                        state.monster_health -= damage;
                        println!(
                            "The {} smashes you with its branch, dealing {} damage!",
                            state.monster.get_name(),
                            damage
                        );
                    }
                },
            )],
            ability_chances: vec![0.25],
        }
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
            (Material::TreeSpiritRemains, 1, 1, 0.5),
            (Material::Log, 1, 2, 1.0),
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
