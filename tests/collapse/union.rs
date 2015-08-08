use rules::parse::Ast::*;
use rules::parse::Membership::*;

use super::simplify;

#[test]
fn text() {
    assert_eq!(vec![Char('f'), Char('r'), Char('e'), Char('d')], simplify(r"fred"));
}
#[test]
fn empty() {
    // Set of chars inside `[]`
    let set = Set(vec![Char('a')].into(), Inclusive);
    // A single class denoted by `<[]>`
    assert_eq!(vec![set.clone()], simplify(r"< + [ a ] >"));
    assert_eq!(vec![set], simplify(r"<[ a ] + >"));
}
#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set   = Set(vec![Char('a'), Char('b'), Char('c')].into(), Inclusive);
    let empty = Set(vec![].into(), Exclusive);
    // A single class which is the union of all subsets.
    assert_eq!(vec![set], simplify(r"< [ abab ] + [ bc ] + [ abc ] >"));
    assert_eq!(vec![empty.clone()], simplify(r"< [ abc \d \D ] >"));
    assert_eq!(vec![empty], simplify(r"< [ abc ] + [ \d ] + [ \D ] >"));
}
