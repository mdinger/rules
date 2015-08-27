use std::collections::VecDeque;
use rules::parse::{self, Ast};

fn new_deque(vec: Vec<Ast>) -> VecDeque<Ast> {
    let deque: VecDeque<Ast> = vec.into_iter().collect();

    deque
}
fn parse(s: &str) -> Vec<Ast> {
    parse::parse(s).unwrap()
}

mod chars;
mod char_class;
mod assertions;
