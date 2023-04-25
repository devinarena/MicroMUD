use crate::item::{Item, Material};

pub trait TreeData {
    fn get_name(&self) -> String;
    fn get_xp(&self) -> u64;
    fn get_result(&self) -> Item;
    fn get_success_rate(&self) -> u32;
    fn get_required_level(&self) -> u64;
    fn get_apple_chance(&self) -> u32 {
        0
    }
}

pub struct NormalTree {}

impl NormalTree {
    pub fn new() -> NormalTree {
        NormalTree {}
    }
}

impl TreeData for NormalTree {
    fn get_name(&self) -> String {
        "Normal Tree".to_string()
    }

    fn get_xp(&self) -> u64 {
        8
    }

    fn get_result(&self) -> Item {
        Item::new(Material::Log, 1).clone()
    }

    fn get_success_rate(&self) -> u32 {
        20
    }

    fn get_required_level(&self) -> u64 {
        1
    }
}

pub struct OakTree {}

impl OakTree {
    pub fn new() -> OakTree {
        OakTree {}
    }
}

impl TreeData for OakTree {
    fn get_name(&self) -> String {
        "Oak Tree".to_string()
    }

    fn get_xp(&self) -> u64 {
        15
    }

    fn get_result(&self) -> Item {
        Item::new(Material::OakLog, 1).clone()
    }

    fn get_success_rate(&self) -> u32 {
        15
    }

    fn get_required_level(&self) -> u64 {
        15
    }

    fn get_apple_chance(&self) -> u32 {
        3
    }
}

pub struct BirchTree {

}

impl BirchTree {
    pub fn new() -> BirchTree {
        BirchTree {}
    }
}

impl TreeData for BirchTree {
    fn get_name(&self) -> String {
        "Birch Tree".to_string()
    }

    fn get_xp(&self) -> u64 {
        20
    }

    fn get_result(&self) -> Item {
        Item::new(Material::BirchLog, 1).clone()
    }

    fn get_success_rate(&self) -> u32 {
        14
    }

    fn get_required_level(&self) -> u64 {
        25
    }

    fn get_apple_chance(&self) -> u32 {
        5
    }
}