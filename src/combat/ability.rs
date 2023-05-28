use super::{FightState};

pub struct Ability {
    name: String,
    description: String,
    combat_style: String,
    level: u64,
    cost: f32,
    pub activate: fn(state: &mut FightState),
}

impl Ability {
    pub fn new(
        name: String,
        description: String,
        combat_style: String,
        level: u64,
        cost: f32,
        activate: fn(state: &mut FightState),
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

    pub fn get_level(&self) -> u64 {
        self.level
    }

    pub fn get_cost(&self) -> f32 {
        self.cost
    }
}
