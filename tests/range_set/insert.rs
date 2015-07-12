use super::generate;

#[test]
fn partial_overlap() {
    let left  = generate(vec![('3', '5'), ('4', '6')]);
    let right = generate(vec![('3', '5'), ('1', '4')]);

    let other_left  = generate(vec![('3', '6')]);
    let other_right = generate(vec![('1', '5')]);

    assert_eq!(left, other_left);
    assert_eq!(right, other_right);
}
#[test]
fn subset() {
    let inner = generate(vec![('1', '5'), ('2', '3')]);
    let left  = generate(vec![('1', '5'), ('1', '3')]);
    let right = generate(vec![('1', '5'), ('2', '5')]);
    let both  = generate(vec![('1', '5'), ('1', '5')]);

    let other = generate(vec![('1', '5')]);

    assert_eq!(inner, other);
    assert_eq!(left, other);
    assert_eq!(right, other);
    assert_eq!(both, other);
}
#[test]
fn superset() {
    let set   = generate(vec![('3', '5'), ('2', '6')]);
    let other = generate(vec![('2', '6')]);

    assert_eq!(other, set);
}
#[test]
// Useless test but added for completeness.
fn disjoint() {
    let set   = generate(vec![('2', '4'), ('6', '8')]);
    let other = generate(vec![('2', '4'), ('6', '8')]);

    assert_eq!(other, set);
}
#[test]
fn disjoint_extend() {
    let left  = generate(vec![('5', '8'), ('2', '4')]);
    let right = generate(vec![('2', '4'), ('5', '8')]);
    let other = generate(vec![('2', '8')]);

    assert_eq!(other, left);
    assert_eq!(other, right);
}
#[test]
#[should_panic]
fn panic_on_decreasing_order() {
    generate(vec![('5', '3')]);
}
