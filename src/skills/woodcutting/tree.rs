use crate::item::{Item, Material};

pub trait TreeData {
    fn get_name(&self) -> String;
    fn get_xp(&self) -> i64;
    fn get_result(&self) -> Item;
    fn get_success_rate(&self) -> u32;
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

    fn get_xp(&self) -> i64 {
        8
    }

    fn get_result(&self) -> Item {
        Item::new(Material::LOG, 1).clone()
    }

    fn get_success_rate(&self) -> u32 {
        25
    }
}
