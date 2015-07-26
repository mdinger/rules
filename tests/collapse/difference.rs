use rules::parse::Ast::*;
use rules::parse::Membership::*;
use rules::range_set;

use super::{new_deque, simplify};

#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set: range_set::Set = vec![Char('a')].into();
    // Deque of ops and sets inside `<>`
    let deque_ex = new_deque(vec![Set(set.clone(), Exclusive)]);
    let deque_in = new_deque(vec![Set(set, Inclusive)]);
    // A single class which is the union of all subsets.
    assert_eq!(vec![Class(deque_ex)], simplify(r"< - [ a ] >"));
    assert_eq!(vec![Class(deque_in)], simplify(r"< [ abc ] - [ b ] - [ cde ] - >"));
}