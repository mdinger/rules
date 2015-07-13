use rules::parse::Ast::*;
use rules::parse::Membership::*;
use rules::parse::ToCharSet;

use super::{new_deque, simplify};

#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set = vec![Char('c')].to_char_set();
    // Deque of ops and sets inside `<>`
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class which is the union of all subsets.
    assert_eq!(vec![Class(deque)], simplify(r"<[ abc ] & [ cef ]>"));
}
#[test]
#[should_panic]
fn char_class_w_empty() {
    // Intersection with nothing results in nothing. An
    // empty class is not allowed.
    simplify(r"< & [ abc ]>");
}
