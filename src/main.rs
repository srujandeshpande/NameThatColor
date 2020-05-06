extern crate serde;
extern crate serde_json;

use serde::{Deserialize};
//use std::collections::HashMap;
use std::io;

fn mrp() -> Result<(String), Box<dyn std::error::Error>> {
    //#[derive(Serialize, Deserialize, Debug)]

    #[derive(Deserialize, Debug)]
    struct Ip {
        colors: Vec<Color>,
    }

    #[derive(Deserialize, Debug)]
    struct Color {
        hex: String,
        name: String,
        rgb: rgb,
        requestedHex: String,
        luminance: f32,
        distance: f32,
    }

    #[derive(Deserialize, Debug)]
    struct rgb {
        r: u32,
        g: u32,
        b: u32,
    }

    println!("Enter the Hex");
    let mut hex = String::new();
    io::stdin()
        .read_line(&mut hex)
        .expect("Failed to read line");

    println!("Your color is {}", hex);

    let request_url = format!("https://api.color.pizza/v1/{hex}", hex=hex);

    //let resp = reqwest::blocking::get(&request_url)?
    //    .json::<HashMap<String, <>>>>()?;
        //.json::<HashMap<String, String>>()?;

        let resp = reqwest::blocking::ClientBuilder::new()
        .gzip(true)
        .build()?
        .get(&request_url)
        .send()?
        .json::<Ip>()?;

    //let deserialized: Color = serde_json::from_str(&resp.colors[0]).unwrap();
    println!("deserialized = {:#?}", resp.colors[0].name);
    let nm = &resp.colors[0].name;
    //let nm = "SDS";
    //Ok(())
    return Ok(nm.to_string());

}

fn main() {
    let ans = mrp();
    println!("{:?}", ans);
}
