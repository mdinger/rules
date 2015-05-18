#![cfg(not(test))]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate rules;

use rules::parse;
use rules::collapse;

fn test_collapse(regex: &str) {
    let parsed = match parse::parse(regex) {
        Ok(vec) => vec,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("Parsed:\n{:?}", &parsed);

    let collapsed = collapse::collapse(parsed);
    println!("Collapsed:\n{:?}", &collapsed);
}

fn main() {
    test_collapse(r" ab<[..]> ");
}
