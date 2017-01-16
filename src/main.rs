extern crate hyper;

use std::io::Read;

fn main() {
    println!("sup google");
    std::io::copy(&mut hyper::Client::new().get("https://www.google.com/").send().unwrap(), &mut std::io::stdout());
}
