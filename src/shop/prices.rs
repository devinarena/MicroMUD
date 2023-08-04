use crate::item::Material;

impl Material {
    pub fn get_value(&self) -> u64 {
        match self {
            // Woodcutting
            Material::Log => 5,
            Material::OakLog => 20,
            Material::BirchLog => 22,
            Material::Apple => 25,
            Material::WoodenAxe => 50,
            // Firemaking
            Material::LeatherGloves => 100,
            Material::TreeSpiritRemains => 200,
            // Melee Equipment
            Material::WoodenDagger => 50,
            Material::WoodenSword => 75,
            Material::WoodenShield => 75,
            Material::WoodenHelmet => 100,
            Material::WoodenChestplate => 150,
            Material::WoodenLeggings => 125,
            Material::WoodenBoots => 100,
            // Ranged Equipment
            // Magic Equipment
            Material::NoviceWand => 200,
        }
    }
}
