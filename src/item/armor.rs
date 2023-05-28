use super::Material;


impl Material {
    pub fn get_attack_bonus(&self) -> u64 {
        match self {
            _ => 0,
        }
    }

    pub fn get_defense_bonus(&self) -> u64 {
        match self {
            Material::LeatherGloves => 1,
            Material::WoodenShield => 1,
            Material::WoodenHelmet => 1,
            Material::WoodenChestplate => 2,
            Material::WoodenLeggings => 1,
            Material::WoodenBoots => 1,
            _ => 0,
        }
    }
}