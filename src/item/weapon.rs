use super::Material;


impl Material {
    pub fn get_melee_bonus(&self) -> u64 {
        match self {
            Material::WoodenDagger => 1,
            Material::WoodenAxe => 1,
            Material::WoodenSword => 1,
            _ => 0,
        }
    }
}