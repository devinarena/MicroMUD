use std::{
    env::join_paths,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde_json;

use crate::player::Player;

pub fn get_all_saves() -> Vec<String> {
    let mut saves = Vec::new();
    for entry in std::fs::read_dir("saves").expect("Unable to read saves directory") {
        let entry = entry.expect("Unable to read entry");
        let path = entry.path();
        let save = path.file_stem().unwrap().to_str().unwrap().to_string();
        saves.push(save);
    }
    saves
}

pub fn read_player_save(save: &String) -> Player {
    let mut file = File::open(format!("saves/{}.json", save))
        .expect(format!("Unable to open saves/{}.json", save).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Unable to open saves/{}.json", save).as_str());
    let player_json = serde_json::from_str(&contents).expect("Unable to parse player.json");
    Player::deserialize(&player_json)
}

pub fn write_player_save(player: &Player) {
    let player_json = player.serialize();
    let save = format!("saves/{}.json", player.get_name());
    let mut file =
        File::create(save.clone()).expect(format!("Unable to create {}.json", save).as_str());
    file.write_all(player_json.as_bytes())
        .expect(format!("Unable to write to {}.json", save).as_str());
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear the screen
}