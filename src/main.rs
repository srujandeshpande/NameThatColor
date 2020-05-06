use serde::{Deserialize};
//use std::collections::HashMap;
use std::io;

fn mrp() -> Result<(String), Box<dyn std::error::Error>> {
    #[derive(Deserialize)]
    struct Ip {
        origin: String,
    }

    println!("Enter the Hex");
    let mut hex = String::new();
    io::stdin()
        .read_line(&mut hex)
        .expect("Failed to read line");

    println!("Your color is {}", hex);

    //let request_url = format!("https://api.color.pizza/v1/{hex}", hex=hex);
    let request_url = "https://httpbin.org/ip";
    println!("{}", request_url);

    //let resp = reqwest::blocking::get(request_url)?
    //    .json::<HashMap<String, String>>()?;

    let resp = reqwest::blocking::get(request_url)?
        .json::<Ip>()?;

    //let json: Ip = reqwest::blocking::get("http://httpbin.org/ip")
    //    .json();
    //let ip = reqwest::blocking::get("http://httpbin.org/ip")?
    //    .json::<Ip>()?;

    println!("{}", resp.origin);
    //Ok(())
    return Ok (resp.origin);
}

fn main() {
    let ans = mrp();
    println!("{:?}", ans);
}

/*

extern crate reqwest;
    extern crate serde;

     use reqwest::Error;
     use serde::Deserialize;

    // This `derive` requires the `serde` dependency.
     #[derive(Deserialize)]
     struct Ip {
         origin: String,
     }
/*
     async fn main() -> Result<(), Error> {
         println!("Here3");

     let ip = reqwest::get("http://httpbin.org/ip")
         .await?
         .json::<Ip>()
         .await?;

     println!("ip: {}", ip.origin);
      Ok(())
      }
*/

fn main(){
    #[derive(Deserialize)]

struct Ip {
    origin: String,
}

let json: Ip = reqwest::blocking::get("http://httpbin.org/ip").json();
//reqwest::get("http://httpbin.org/ip")?.json()?;
}
*/
/*
    #[derive(Deserialize)]
struct Ip {
    origin: String,
}

let ip = reqwest::get("http://httpbin.org/ip").json::<Ip>();

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = reqwest::get("http://httpbin.org/ip")
         .await?
         .bytes()
         .await?;

     println!("bytes: {:?}", bytes);
     Ok(())
     }

}
*/

/*
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
*/
