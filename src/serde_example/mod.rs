/*
use {
    std::{
        fs::read_to_string,
    },
    serde::{Serialize, Deserialize},
};

#[derive(Serialize, Deserialize, Debug)]
struct Json<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    let content: String = read_to_string("example.json").unwrap();
    println!("{}", content);

    let json: Json = serde_json::from_str(&content).unwrap();
    println!("{:?}", json);
}*/
