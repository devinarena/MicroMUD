use crate::player::Player;

use super::monsters::MonsterData;

pub struct Ability {
    name: String,
    description: String,
    combat_style: String,
    level: usize,
    cost: f32,
    pub activate: fn(
        pl: &mut Player,
        health: &mut i32,
        max_health: i32,
        adrenaline: f32,
        monster: &Box<dyn MonsterData>,
        ehealth: &mut i32,
        emax_health: i32,
        elevel: u64,
        eattack: u64,
        ecrit_chance: f32,
        eadrenaline: f32,
    ),
}

impl Ability {
    pub fn new(
        name: String,
        description: String,
        combat_style: String,
        level: usize,
        cost: f32,
        activate: fn(
            pl: &mut Player,
            health: &mut i32,
            max_health: i32,
            adrenaline: f32,
            monster: &Box<dyn MonsterData>,
            ehealth: &mut i32,
            emax_health: i32,
            elevel: u64,
            eattack: u64,
            ecrit_chance: f32,
            eadrenaline: f32,
        ),
    ) -> Self {
        Self {
            name,
            description,
            combat_style,
            level,
            cost,
            activate,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_combat_style(&self) -> &String {
        &self.combat_style
    }

    pub fn get_level(&self) -> usize {
        self.level
    }

    pub fn get_cost(&self) -> f32 {
        self.cost
    }
}
