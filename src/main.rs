#![cfg(not(test))]
extern crate range_set;

use range_set::{Range, Set};

fn main() {
    let mut set = Set::new();
    set.insert(Range('1', '4'));
    set.union(Range('5', '6'));
    set.union(Range('4', '5'));
    set.union(Range('2', '5'));

    println!("{}", set);
}
