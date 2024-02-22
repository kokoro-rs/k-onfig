#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

use k_onfig::Konfig;
#[derive(Konfig)]
struct Hello {
    content: String,
}
#[derive(Konfig)]
struct Test {
    hello: Vec<Hello>,
    set: HashSet<Hello>,
    map: HashMap<String, Vec<Hello>>,
    number: i32,
}
fn main() {
    let konfig = Test::konfig();
    let config = Test::konfig();
    assert_eq!(konfig, config);
    println!("{:#?}", konfig);
}
