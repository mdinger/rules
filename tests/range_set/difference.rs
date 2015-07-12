use super::generate;

#[test]
fn partial_overlap() {
    let set   = generate(vec![('2', '7')]);

    let left  = generate(vec![('0', '3')]);
    let right = generate(vec![('6', '8')]);

    let other_left  = generate(vec![('4', '7')]);
    let other_right = generate(vec![('2', '5')]);

    assert_eq!(set.difference(&left), other_left);
    assert_eq!(set.difference(&right), other_right);
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
    let empty        = generate(vec![]);

    assert_eq!(set.difference(&subset), other_subset);
    assert_eq!(set.difference(&exact_left), other_left);
    assert_eq!(set.difference(&exact_right), other_right);
    assert_eq!(set.difference(&set), empty);
}
#[test]
fn superset() {
    let set      = generate(vec![('2', '7')]);
    let superset = generate(vec![('1', '8')]);

    let empty = generate(vec![]);

    assert_eq!(set.difference(&superset), empty);
}

#[test]
fn disjoint() {
    let set = generate(vec![('3', '4')]);

    let left  = generate(vec![('1', '2')]);
    let right = generate(vec![('5', '6')]);

    assert_eq!(set.difference(&left), set);
    assert_eq!(set.difference(&right), set);
}
