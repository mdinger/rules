use rules::parse::Ast::*;
use rules::parse::Membership::*;

use super::{new_deque, simplify};

#[test]
fn text() {
    assert_eq!(vec![Char('f'), Char('r'), Char('e'), Char('d')], simplify(r"fred"));
}
#[test]
fn empty() {
    // Set of chars inside `[]`
    let set = vec![Char('a')].into();
    // Deque of ops and sets inside `<>`
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![Class(deque.clone())], simplify(r"< + [ a ] >"));
    assert_eq!(vec![Class(deque)], simplify(r"<[ a ] + >"));
}
#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set   = vec![Char('a'), Char('b'), Char('c')].into();
    let empty = vec![].into();
    // Deque of ops and sets inside `<>`
    let deque      = new_deque(vec![Set(set, Inclusive)]);
    let everything = new_deque(vec![Set(empty, Exclusive)]);
    // A single class which is the union of all subsets.
    assert_eq!(vec![Class(deque)], simplify(r"< [ abab ] + [ bc ] + [ abc ] >"));
    assert_eq!(vec![Class(everything.clone())], simplify(r"< [ abc \d \D ] >"));
    assert_eq!(vec![Class(everything)], simplify(r"< [ abc ] + [ \d ] + [ \D ] >"));
}
