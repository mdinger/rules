use super::generate;
use rules::range_set::Range;

#[test]
fn partial_overlap() {
    let mut left  = generate(vec![('4', '7')]);
    let mut right = left.clone();

    left.remove(Range('3', '5'));
    right.remove(Range('6', '9'));

    let other_left  = generate(vec![('6', '7')]);
    let other_right = generate(vec![('4', '5')]);

    assert_eq!(left,  other_left);
    assert_eq!(right, other_right);
}
#[test]
fn subset() {
    let mut inner  = generate(vec![('3', '8')]);
    let mut left   = inner.clone();
    let mut right  = inner.clone();
    let mut both   = inner.clone();
    
    inner.remove(Range('4', '7'));
    left .remove(Range('3', '7'));
    right.remove(Range('4', '8'));
    both .remove(Range('3', '8'));

    let other_inner = generate(vec![('3', '3'), ('8', '8')]);
    let other_left  = generate(vec![('8', '8')]);
    let other_right = generate(vec![('3', '3')]);
    let other_both  = generate(vec![]);

    assert_eq!(inner, other_inner);
    assert_eq!(left, other_left);
    assert_eq!(right, other_right);
    assert_eq!(both, other_both);
}
#[test]
fn superset() {
    let mut set = generate(vec![('5', '6')]);
    set.remove(Range('3', '8'));

    let other = generate(vec![]);
    assert_eq!(set, other);
}
#[test]
fn disjoint() {
    let mut set = generate(vec![('3', '4')]);
    set.remove(Range('6', '8'));
    set.remove(Range('1', '2'));

    let other = generate(vec![('3', '4')]);
    assert_eq!(set, other);
}
