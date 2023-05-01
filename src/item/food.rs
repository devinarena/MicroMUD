use super::Material;


impl Material {
    pub fn get_food_heal(&self) -> u32 {
        match self {
            Material::Apple => 10,
            _ => 0,
        }
    }
}