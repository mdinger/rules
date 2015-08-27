use rules::parse::Ast::*;
use rules::parse::Assertion::*;
use rules::parse::Sign::*;

use super::parse;

#[test]
fn text() {
    // Positive assertions
    assert_eq!(vec![Assertion(vec![Char('b')], Lookbehind, Positive), Char('p')],
               parse(r"<?before b> p"));
    assert_eq!(vec![Char('b'), Assertion(vec![Char('p')], Lookahead, Positive)],
               parse(r"b <?after p>"));

    // Negative assertions.
    assert_eq!(vec![Assertion(vec![Char('b')], Lookbehind, Negative), Char('p')],
               parse(r"<!before b> p"));
    assert_eq!(vec![Char('b'), Assertion(vec![Char('p')], Lookahead, Negative)],
               parse(r"b <!after p>"));
}
