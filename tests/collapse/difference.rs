use rules::parse::Ast::*;
use rules::parse::Membership::*;
use rules::range_set;

use super::simplify;

#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set: range_set::Set = vec![Char('a')].into();
    // A single class which is the union of all subsets.
    assert_eq!(vec![Set(set.clone(), Exclusive)], simplify(r"<-[ a ] >"));
    assert_eq!(vec![Set(set, Inclusive)], simplify(r"<[ abc ] - [ b ] - [ cde ]>"));
}
