use std::io;

fn main() {
    println!("Enter the Hex");

    let mut hex = String::new();

    io::stdin()
        .read_line(&mut hex)
        .expect("Failed to read line");

    println!("Your color is {}", hex);
}
