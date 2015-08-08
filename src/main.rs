#![cfg(not(test))]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate rules;

use rules::re::Regex;

fn main() {
    let re = Regex::new(r"pine  // This is a comment
                          apple // This is another comment");
    println!("match is {}", re.is_match("pineapple"));
}
