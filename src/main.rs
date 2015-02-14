#![cfg(not(test))]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate rules;

use rules::parse;

fn test_parse(regex: &str) {
    let vec = parse::parse(regex);
    
    println!("Ast:\n{:?}", vec);
}
/*
use rules::Rule;

fn test_match(regex: &str, s: &str) {
    let rule = match Rule::new(regex) {
        Ok(rule) => rule,
        Err(err) => panic!("{}", err),
    };
    
    //assert_eq!(rule.is_match(s), true);
}
*/
// Matches `a` or `b` or `c`
// <[ a b c ]>

fn main() {
    test_parse(r"< - [ fred ] >");
}
