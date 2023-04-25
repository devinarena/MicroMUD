use super::Material;


impl Material {
    pub fn get_attack_bonus(&self) -> u32 {
        match self {
            _ => 0,
        }
    }

    pub fn get_defense_bonus(&self) -> u32 {
        match self {
            Material::LeatherGloves => 1,
            _ => 0,
        }
    }
}