use super::Material;


impl Material {
    pub fn get_melee_bonus(&self) -> u64 {
        match self {
            Material::WoodenDagger => 1,
            Material::WoodenAxe => 1,
            Material::WoodenSword => 2,
            _ => 0,
        }
    }

    pub fn get_slot(&self) -> &str {
        match self {
            Material::WoodenDagger => &"main_hand",
            Material::WoodenAxe => &"main_hand",
            Material::WoodenSword => &"main_hand",
            Material::WoodenShield => &"off_hand",
            Material::LeatherGloves => &"main_hand",
            _ => &"none",
        }
    }

    pub fn get_combat_style(&self) -> &str {
        match self {
            Material::WoodenDagger => &"melee",
            Material::WoodenAxe => &"melee",
            Material::WoodenSword => &"melee",
            Material::WoodenShield => &"melee",
            _ => &"none",
        }
    }
}