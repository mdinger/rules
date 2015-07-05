use super::generate;
use rules::range_set::Range;

#[test]
fn partial_overlap() {
    let mut set1 = generate(vec![('5', '9')]);
    set1.remove(Range('3', '6')); // beginnings different.

    let mut set2 = generate(vec![('5', '9')]);
    set2.remove(Range('5', '6')); // beginnings same.

    let mut set3 = generate(vec![('2', '6')]);
    set3.remove(Range('5', '9')); // end different.

    let mut set4 = generate(vec![('2', '6')]);
    set4.remove(Range('5', '6')); // end same.

    let other12 = generate(vec![('7', '9')]);
    let other34 = generate(vec![('2', '4')]);

    assert_eq!(set1, other12);
    assert_eq!(set2, other12);
    assert_eq!(set3, other34);
    assert_eq!(set4, other34);
}
#[test]
fn subset() {
    let mut set = generate(vec![('1', '9')]);
    set.remove(Range('3', '6'));

    let other = generate(vec![('1', '2'), ('7', '9')]);

    assert_eq!(set, other);
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
    let mut set = generate(vec![('2', '3')]);
    set.remove(Range('6', '8'));

    let other = generate(vec![('2', '3')]);
    assert_eq!(set, other);
}
