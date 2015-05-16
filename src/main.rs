#![cfg(not(test))]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate rules;

use rules::parse;
use rules::collapse;

fn test_collapse(regex: &str) {
    let parsed = parse::parse(regex);
    println!("Parsed:\n{:?}", &parsed);

    let collapsed = collapse::collapse(parsed);
    println!("Collapsed:\n{:?}", &collapsed);
}

fn main() {
    test_collapse(r" ab<[..]> ");
}
