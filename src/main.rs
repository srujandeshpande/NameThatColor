extern crate serde;
extern crate serde_json;
//extern crate fs;
//#[macro_use] extern crate serde_derive;
//extern crate serde_derive;

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

    //let request_url = format!("https://api.color.pizza/v1/{hex}", hex=hex);
    let request_url = "https://api.color.pizza/v1/ffffff";
    //let request_url = "https://httpbin.org/ip";
    println!("{}", request_url);

    //let resp = reqwest::blocking::get(&request_url)?
    //    .json::<HashMap<String, <>>>>()?;
        //.json::<HashMap<String, String>>()?;

    //let resp = reqwest::blocking::get(request_url)?
        //.json::<Ip>()?;
        let resp = reqwest::blocking::ClientBuilder::new()
        .gzip(true)
        .build()?
        .get(request_url)
        .send()?
        //.text()?;
        .json::<Ip>()?;

        //println!("{}",resp);

    //let r1 = resp.into_bytes();
    //let r1 = String::from(j).into_bytes();
    //let r2 = String::from(k).into_bytes();
    //assert_eq!(&r1[..], &r2[..]);
    //fs::write("res.json", &resp);
    //let ip = reqwest::blocking::get("http://httpbin.org/ip")?
    //    .json::<Ip>()?;
    ////let deserialized: Ip = serde_json::from_str(k).unwrap();
    ///let deserialized: Ip = serde_json::from_str(&resp).unwrap();
    //let deserialized: Color = serde_json::from_str(&resp.colors[0]).unwrap();
    println!("deserialized = {:#?}", resp.colors[0].name);
    ///println!("deserialized = {:#?}", deserialized.colors[0].name);
    //println!("{}", resp.colors);

    //Ok(())
    return Ok(hex);

}

fn main() {
    let ans = mrp();
    println!("{:?}", ans);
}
