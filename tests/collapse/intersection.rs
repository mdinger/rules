use rules::parse::Ast::*;
use rules::parse::Membership::*;

use super::simplify;

#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set = vec![Char('c')].into();
    // A single class which is the union of all subsets.
    assert_eq!(vec![Set(set, Inclusive)], simplify(r"<[ abc ] & [ cef ]>"));
}
#[test]
#[should_panic]
fn char_class_w_empty() {
    // Intersection with nothing results in nothing. An
    // empty class is not allowed.
    simplify(r"< & [ abc ]>");
}
