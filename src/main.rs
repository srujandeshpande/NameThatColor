extern crate reqwest;

use std::io;
use serde::{Serialize, Deserialize};
//extern crate serde_derive;
//use serde_json::{json};


fn main() {
    println!("Enter the Hex");

    let mut hex = String::new();

    io::stdin()
        .read_line(&mut hex)
        .expect("Failed to read line");

    println!("Your color is {}", hex);

    let request_url = format!("https://api.color.pizza/v1/{hex}", hex=hex);
    println!("{}", request_url);
    let mut res = reqwest::get(&request_url);

    #[derive(Serialize,Deserialize)]
    struct Ip {
        origin: String,
    }

}
