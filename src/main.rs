extern crate serde;
extern crate serde_json;

//#[macro_use] extern crate serde_derive;
extern crate serde_derive;
use serde::{Deserialize};
//use std::collections::HashMap;
use std::io;





fn mrp() -> Result<(String), Box<dyn std::error::Error>> {
    //#[derive(Serialize, Deserialize, Debug)]
    #[derive(Deserialize, Debug)]

    struct Ip {
        colors: Vec<String>,
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
    //let request_url = "https://httpbin.org/ip";
    println!("{}", request_url);

    //let resp = reqwest::blocking::get(request_url)?
    //    .json::<HashMap<String, String>>()?;

    let resp = reqwest::blocking::get(&request_url)?
        .json::<Ip>()?;

    //let resp = reqwest::blocking::get(&request_url)?
    //    .text()?;

    //let json: Ip = reqwest::blocking::get("http://httpbin.org/ip")
    //    .json();
    //let ip = reqwest::blocking::get("http://httpbin.org/ip")?
    //    .json::<Ip>()?;
    //for i in resp.color
    //let col = resp.colors[0].json::<Color>()?;
    //if resp.colors[0][0] == 239 {
    //resp.colors[0].remove(0);
    //}
    let deserialized: Color = serde_json::from_str(&resp.colors[0]).unwrap();
    println!("deserialized = {:#?}", deserialized.name);
    //println!("{}", resp.colors);

    //Ok(())
    return Ok(request_url);
    //return Ok (resp.colors[0]);
}

fn main() {
    let ans = mrp();
    println!("{:?}", ans);
}
