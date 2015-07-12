use super::generate;

#[test]
fn partial_overlap() {
    let set   = generate(vec![('2', '7')]);
    let left  = generate(vec![('0', '3')]);
    let right = generate(vec![('6', '8')]);

    let other_left  = generate(vec![('2', '3')]);
    let other_right = generate(vec![('6', '7')]);

    assert_eq!(set.intersection(&left), other_left);
    assert_eq!(set.intersection(&right), other_right);
}
#[test]
fn subset() {
    let set = generate(vec![('2', '7')]);

    let subset      = generate(vec![('3', '6')]);
    let exact_left  = generate(vec![('2', '6')]);
    let exact_right = generate(vec![('3', '7')]);

    assert_eq!(set.intersection(&subset), subset);
    assert_eq!(set.intersection(&exact_left), exact_left);
    assert_eq!(set.intersection(&exact_right), exact_right);
    assert_eq!(set.intersection(&set), set);
}
#[test]
fn superset() {
    let set      = generate(vec![('2', '7')]);
    let superset = generate(vec![('1', '8')]);

    assert_eq!(set.intersection(&superset), set);
}
#[test]
fn disjoint() {
    let set = generate(vec![('3', '4')]);

    let left  = generate(vec![('1', '2')]);
    let right = generate(vec![('5', '6')]);

    let empty = generate(vec![]);

    assert_eq!(set.intersection(&left), empty);
    assert_eq!(set.intersection(&right), empty);
}
