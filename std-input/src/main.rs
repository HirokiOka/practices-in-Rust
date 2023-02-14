use std::io;

fn main() {
    println!("Enter some text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    println!("You typed: {}", input);
}
