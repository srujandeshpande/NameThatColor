extern crate serde;
extern crate serde_json;
//#[macro_use] extern crate serde_derive;
extern crate serde_derive;

use serde::{Deserialize};
use std::collections::HashMap;
use std::io;
use serde_json::Map;

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


    let j = r#"{
        "colors":[
        {
          "hex":"121212",
          "name":"Dark Tone Ink",
          "rgb":{
            "r":18,
            "g":18,
            "b":18
            },
          "requestedHex":"121212",
          "luminance":12.033992853579395,
          "distance":0
          }
          ]
    }"#;


    println!("Enter the Hex");
    let mut hex = String::new();
    io::stdin()
        .read_line(&mut hex)
        .expect("Failed to read line");

    println!("Your color is {}", hex);

    let request_url = format!("https://api.color.pizza/v1/{hex}", hex=hex);
    //let request_url = "https://httpbin.org/ip";
    println!("{}", request_url);

    //let resp = reqwest::blocking::get(&request_url)?
    //    .json::<HashMap<String, <>>>>()?;
        //.json::<HashMap<String, String>>()?;

    //let resp = reqwest::blocking::get(&request_url)?
    //    .json::<Ip>()?;

    //let ip = reqwest::blocking::get("http://httpbin.org/ip")?
    //    .json::<Ip>()?;
    let deserialized: Ip = serde_json::from_str(j).unwrap();
    //let deserialized: Color = serde_json::from_str(&resp.colors[0]).unwrap();
    println!("deserialized = {:#?}", deserialized.colors[0].hex);
    //println!("{}", resp.colors);

    //Ok(())
    return Ok(request_url);

}

fn main() {
    let ans = mrp();
    println!("{:?}", ans);
}
