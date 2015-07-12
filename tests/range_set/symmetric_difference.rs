use super::generate;

#[test]
fn partial_overlap() {
    let set = generate(vec![('3', '6')]);

    let low  = generate(vec![('1', '4')]);
    let high = generate(vec![('5', '9')]);

    let other_low  = generate(vec![('1', '2'), ('5', '6')]);
    let other_high = generate(vec![('3', '4'), ('7', '9')]);

    assert_eq!(set.symmetric_difference(&low), other_low);
    assert_eq!(set.symmetric_difference(&high), other_high);
}
#[test]
fn subset() {
    let set = generate(vec![('2', '7')]);

    let subset      = generate(vec![('3', '6')]);
    let exact_left  = generate(vec![('2', '6')]);
    let exact_right = generate(vec![('3', '7')]);

    let other_subset = generate(vec![('2', '2'), ('7', '7')]);
    let other_left   = generate(vec![('7', '7')]);
    let other_right  = generate(vec![('2', '2')]);
    let other_both   = generate(vec![]);

    assert_eq!(set.symmetric_difference(&subset), other_subset);
    assert_eq!(set.symmetric_difference(&exact_left), other_left);
    assert_eq!(set.symmetric_difference(&exact_right), other_right);
    assert_eq!(set.symmetric_difference(&set), other_both);
}
#[test]
fn superset() {
    let set      = generate(vec![('2', '7')]);
    let superset = generate(vec![('1', '8')]);

    let other = generate(vec![('1', '1'), ('8', '8')]);
    assert_eq!(set.symmetric_difference(&superset), other);
}
#[test]
fn disjoint() {
    let set = generate(vec![('4', '5')]);

    let low  = generate(vec![('1', '2')]);
    let high = generate(vec![('7', '8')]);

    let other_low  = generate(vec![('1', '2'), ('4', '5')]);
    let other_high = generate(vec![('4', '5'), ('7', '8')]);

    assert_eq!(set.symmetric_difference(&low), other_low);
    assert_eq!(set.symmetric_difference(&high), other_high);
}
#[test]
fn disjoint_extend() {
    let set = generate(vec![('3', '4')]);

    let low  = generate(vec![('1', '2')]);
    let high = generate(vec![('5', '6')]);

    let other_low  = generate(vec![('1', '4')]);
    let other_high = generate(vec![('3', '6')]);

    assert_eq!(set.symmetric_difference(&low), other_low);
    assert_eq!(set.symmetric_difference(&high), other_high);
}
