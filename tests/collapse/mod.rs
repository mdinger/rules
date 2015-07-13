use std::collections::VecDeque;
use rules::{collapse, parse};
use rules::parse::Ast;

fn new_deque(vec: Vec<Ast>) -> VecDeque<Ast> {
    let deque: VecDeque<Ast> = vec.into_iter().collect();

    deque
}
fn simplify(s: &str) -> Vec<Ast> {
    collapse::collapse(parse::parse(s).unwrap())
}

mod union;
mod intersection;
mod difference;
mod symmetric_difference;
