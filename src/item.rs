use serde_json::json;


pub struct Item {
    name: String,
    description: String,
    value: i32,
}

impl Item {
    pub fn new(name: String, description: String, value: i32) -> Item {
        Item {
            name,
            description,
            value,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_value(&self) -> &i32 {
        &self.value
    }

    pub fn serialize(&self) -> String {
        json!({
            "name": self.name,
            "description": self.description,
            "value": self.value,
        })
        .to_string()
    }

    pub fn deserialize(json: &serde_json::Value) -> Item {
        Item {
            name: json["name"].as_str().unwrap().to_string(),
            description: json["description"].as_str().unwrap().to_string(),
            value: json["value"].as_i64().unwrap() as i32,
        }
    }
}