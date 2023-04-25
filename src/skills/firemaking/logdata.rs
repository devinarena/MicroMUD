use crate::item::Material;

pub trait LogData {
    fn get_firemaking_xp(&self) -> u64;
    fn get_firemaking_time(&self) -> u32;
    fn get_firemaking_level(&self) -> u64;
}

impl LogData for Material {
    fn get_firemaking_xp(&self) -> u64 {
        match self {
            Material::Log => 20,
            Material::OakLog => 35,
            Material::BirchLog => 45,

            Material::TreeSpiritRemains => 50,
            _ => 0,
        }
    }

    fn get_firemaking_time(&self) -> u32 {
        match self {
            Material::Log => 2500,
            Material::OakLog => 2700,
            Material::BirchLog => 2700,

            Material::TreeSpiritRemains => 3200,
            _ => 0,
        }
    }

    fn get_firemaking_level(&self) -> u64 {
        match self {
            Material::Log => 1,
            Material::OakLog => 15,
            Material::BirchLog => 25,

            Material::TreeSpiritRemains => 5,
            _ => 0,
        }
    }
}
