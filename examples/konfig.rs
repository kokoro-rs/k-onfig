#![allow(dead_code)]
use k_onfig::Konfig;
#[derive(Konfig)]
struct Hello {
    content: String,
}
#[derive(Konfig)]
struct Test {
    hello: Vec::<Hello>,
    //hello: Vec<Hello>,
}

fn main() {
    println!("{:#?}", Test::konfig());
}
