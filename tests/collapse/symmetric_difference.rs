use rules::parse::Ast::*;
use rules::parse::Membership::*;
use rules::parse::ToCharSet;

use super::{new_deque, simplify};

#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set = vec![Char('a')].to_char_set();
    // Deque of ops and sets inside `<>`
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class which is the union of all subsets.
    assert_eq!(vec![Class(deque.clone())], simplify(r"< ^ [ a ] ^ >"));
    assert_eq!(vec![Class(deque)], simplify(r"<[ \d abc ] ^ [ \d bcde ] ^ [ de ]>"));
}
