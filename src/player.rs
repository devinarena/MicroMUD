use std::{collections::HashMap};

use serde_json::{json, Value};

pub struct Player {
    name: String,
    class: String,
    xp: HashMap<String, i32>,
    health: i32,
    location: String,
}

impl Player {
    pub fn new(name: String, class: String) -> Player {
        let mut player = Player {
            name,
            class,
            xp: HashMap::new(),
            health: 100,
            location: "Littlewood Town".to_string(),
        };

        player.xp.insert("melee".to_string(), 0);
        player.xp.insert("ranged".to_string(), 0);
        player.xp.insert("magic".to_string(), 0);
        player.xp.insert("mining".to_string(), 0);
        player.xp.insert("smithing".to_string(), 0);
        player.xp.insert("woodcutting".to_string(), 0);
        player.xp.insert("fishing".to_string(), 0);
        player.xp.insert("cooking".to_string(), 0);
        player.xp.insert("farming".to_string(), 0);

        return player;
    }

    pub fn deserialize(json: &Value) -> Player {
        let name = json["name"].as_str().unwrap().to_string();
        let class = json["class"].as_str().unwrap().to_string();
        let mut player = Player {
            name,
            class,
            xp: HashMap::new(),
            health: 100,
            location: "Littlewood Town".to_string(),
        };

        player.xp.insert(
            "melee".to_string(),
            json["xp"]["melee"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "ranged".to_string(),
            json["xp"]["ranged"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "magic".to_string(),
            json["xp"]["magic"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "mining".to_string(),
            json["xp"]["mining"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "smithing".to_string(),
            json["xp"]["smithing"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "woodcutting".to_string(),
            json["xp"]["woodcutting"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "fishing".to_string(),
            json["xp"]["fishing"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "cooking".to_string(),
            json["xp"]["cooking"].as_i64().unwrap() as i32,
        );
        player.xp.insert(
            "farming".to_string(),
            json["xp"]["farming"].as_i64().unwrap() as i32,
        );

        return player;
    }

    pub fn serialize(&self) -> String {
        json!({
            "name": self.name,
            "class": self.class,
            "xp": {
                "melee": self.xp["melee"],
                "ranged": self.xp["ranged"],
                "magic": self.xp["magic"],
                "mining": self.xp["mining"],
                "smithing": self.xp["smithing"],
                "woodcutting": self.xp["woodcutting"],
                "fishing": self.xp["fishing"],
                "cooking": self.xp["cooking"],
                "farming": self.xp["farming"],
            },
        })
        .to_string()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_class(&self) -> String {
        self.class.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_xp(&self, skill: &String) -> i64 {
        self.xp[skill] as i64
    }

    pub fn get_level(&self, skill: &String) -> i32 {
        let xp = self.xp[skill];
        let level = (1.0 + 6.0 * ((xp as f64) * 4.7 / 150.0).log2()).floor();
        return level as i32;
    }

    pub fn needed_xp(&self, skill: &String) -> i64 {
        let next_level = self.get_level(skill) + 1;
        let needed_xp = ((150.0 * 2_f32.powf((next_level - 1) as f32) / 6.0) / 4.7) as i64;
        return needed_xp;
    }
}
