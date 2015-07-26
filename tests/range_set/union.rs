use super::generate;

#[test]
fn partial_overlap() {
    let left  = generate(vec![('3', '4')]);
    let right = generate(vec![('4', '6')]);

    let other = generate(vec![('3', '6')]);

    assert_eq!(left.union(&right), other);
    assert_eq!(right.union(&left), other);
}
#[test]
fn subset() {
    let set = generate(vec![('1', '5')]);

    let inner = generate(vec![('2', '3')]);
    let left  = generate(vec![('1', '3')]);
    let right = generate(vec![('2', '5')]);
    let both  = generate(vec![('1', '5')]);

    let other = generate(vec![('1', '5')]);

    assert_eq!(set.union(&inner), other);
    assert_eq!(set.union(&left), other);
    assert_eq!(set.union(&right), other);
    assert_eq!(set.union(&both), other);
}
#[test]
fn superset() {
    let set      = generate(vec![('3', '5')]);
    let superset = generate(vec![('2', '6')]);

    assert_eq!(set.union(&superset), superset);
    assert_eq!(superset.union(&set), superset);
}
#[test]
fn disjoint() {
    let left  = generate(vec![('2', '4')]);
    let right = generate(vec![('6', '8')]);

    let other = generate(vec![('2', '4'), ('6', '8')]);

    assert_eq!(left.union(&right), other);
    assert_eq!(right.union(&left), other);
}
#[test]
fn disjoint_extend() {
    let left  = generate(vec![('2', '4')]);
    let right = generate(vec![('5', '8')]);

    let other = generate(vec![('2', '8')]);

    assert_eq!(left.union(&right), other);
    assert_eq!(right.union(&left), other);
}
