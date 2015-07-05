use super::generate;

#[test]
fn partial_overlap() {
    let set   = generate(vec![('2', '7')]);
    let left  = generate(vec![('0', '3')]);
    let right = generate(vec![('6', '8')]);

    let intersection_left  = set.intersection(&left);
    let intersection_right = set.intersection(&right);

    let other_left  = generate(vec![('2', '3')]);
    let other_right = generate(vec![('6', '7')]);

    assert_eq!(intersection_left, other_left);
    assert_eq!(intersection_right, other_right);
}
#[test]
fn subset() {
    let set = generate(vec![('2', '7')]);

    let subset      = generate(vec![('3', '6')]);
    let exact_left  = generate(vec![('2', '6')]);
    let exact_right = generate(vec![('3', '7')]);

    let intersection_subset = set.intersection(&subset);
    let intersection_left   = set.intersection(&exact_left);
    let intersection_right  = set.intersection(&exact_right);
    let intersection_both   = set.intersection(&set);

    assert_eq!(intersection_subset, subset);
    assert_eq!(intersection_left, exact_left);
    assert_eq!(intersection_right, exact_right);
    assert_eq!(intersection_both, set);
}
#[test]
fn superset() {
    let set      = generate(vec![('2', '7')]);
    let superset = generate(vec![('1', '8')]);

    let intersection = set.intersection(&superset);

    let other = generate(vec![('2', '7')]);
    assert_eq!(intersection, other);
}
#[test]
fn disjoint() {
    let set = generate(vec![('3', '4')]);

    let low  = generate(vec![('1', '2')]);
    let high = generate(vec![('5', '6')]);

    let intersection_low  = set.intersection(&low);
    let intersection_high = set.intersection(&high);

    let other_low  = generate(vec![]);
    let other_high = generate(vec![]);

    assert_eq!(intersection_low, other_low);
    assert_eq!(intersection_high, other_high);
}
