use std::vec;

use serde::{Deserialize,Serialize};

#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    skills: Vec<String>
}


fn main() {
    let person = Person {
        name: String::from("Mourad"),
        age: 24,
        skills: vec![String::from("Rust"), String::from("Typescript")]
    };

    // let person_json = serde_json::to_string(&person).unwrap();
    let person_json = serde_json::to_string_pretty(&person).unwrap();

    println!("{}", person_json);
}
