use super::Material;


impl Material {
    pub fn get_combat_style(&self) -> &'static str {
        match self {
            // melee
            Material::WoodenDagger => &"melee",
            Material::WoodenAxe => &"melee",
            Material::WoodenSword => &"melee",
            Material::WoodenShield => &"melee",
            // ranged
            // magic
            Material::NoviceWand => &"magic",
            _ => &"melee",
        }
    }
}