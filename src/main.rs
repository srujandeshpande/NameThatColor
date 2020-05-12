extern crate serde;
extern crate serde_json;

use serde::{Deserialize};
//use std::collections::HashMap;
use std::io;

fn find_name(hex: &str) -> Result< String, Box<dyn std::error::Error>> {
    #[derive(Deserialize, Debug)]
    struct Ip {
        colors: Vec<Color>,
    }

    #[derive(Deserialize, Debug)]
    //Do not convert to snake case - Will Break Deserialization
    struct Color {
        hex: String,
        name: String,
        rgb: Rgb,
        requestedHex: String,
        luminance: f32,
        distance: f32,
    }

    #[derive(Deserialize, Debug)]
    struct Rgb {
        r: u32,
        g: u32,
        b: u32,
    }

    let hex = hex.replace("#", "");
    let request_url = format!("https://api.color.pizza/v1/{hex}", hex=hex);
    //  .json::<HashMap<String, String>>()?;

    let resp = reqwest::blocking::ClientBuilder::new()
    .gzip(true)
    .build()?
    .get(&request_url)
    .send()?
    .json::<Ip>()?;

    let nm = &resp.colors[0].name;
    return Ok(nm.to_string());
}

fn main() {
    println!("Please enter the hex of your color");
    let mut hex = String::new();
    io::stdin()
        .read_line(&mut hex)
        .expect("Failed to read line");
    let ans = find_name(&hex);
    println!("{:?}", ans);
}
