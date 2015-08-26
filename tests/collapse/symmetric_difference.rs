use rules::parse::Ast::*;
use rules::parse::Membership::*;

use super::simplify;

#[test]
fn char_class() {
    // Set of chars inside `[]`
    let set = Set(vec![Char('a')].into(), Inclusive);
    // A single class which is the union of all subsets.
    assert_eq!(vec![set], simplify(r"<[ \d abc ] ^ [ \d bcde ] ^ [ de ]>"));
}
