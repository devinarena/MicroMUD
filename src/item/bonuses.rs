use super::Material;

impl Material {
    pub fn get_bonus(&self, skill: &String) -> u64 {
        match skill.as_str() {
            "melee" => match self {
                Material::WoodenDagger => 1,
                Material::WoodenAxe => 1,
                Material::WoodenSword => 2,
                _ => 0,
            },
            "ranged" => match self {
                _ => 0,
            },
            "magic" => match self {
                Material::NoviceWand => 2,
                _ => 0,
            },
            "defense" => match self {
                Material::WoodenShield => 1,
                Material::WoodenHelmet => 1,
                Material::WoodenChestplate => 2,
                Material::WoodenLeggings => 1,
                Material::WoodenBoots => 1,
                Material::LeatherGloves => 1,
                _ => 0,
            },
            _ => 0,
        }
    }
}
