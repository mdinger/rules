use rules::{collapse, parse};
use rules::parse::Ast;

fn simplify(s: &str) -> Vec<Ast> {
    collapse::collapse(parse::parse(s).unwrap())
}

mod union;
mod intersection;
mod difference;
mod symmetric_difference;
