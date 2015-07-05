use super::generate;

#[test]
fn partial_overlap() {
    let set1 = generate(vec![('3', '5'), ('4', '6')]);
    let set2 = generate(vec![('3', '5'), ('1', '4')]);

    let other1 = generate(vec![('3', '6')]);
    let other2 = generate(vec![('1', '5')]);

    assert_eq!(other1, set1);
    assert_eq!(other2, set2);
}
#[test]
fn subset() {
    let set1 = generate(vec![('1', '5'), ('2', '3')]); // Complete overlap.
    let set2 = generate(vec![('1', '5'), ('1', '3')]); // Left is exact.
    let set3 = generate(vec![('1', '5'), ('2', '5')]); // Right is exact.
    let set4 = generate(vec![('1', '5'), ('1', '5')]); // Both are exact.

    let other = generate(vec![('1', '5')]);

    assert_eq!(set1, other);
    assert_eq!(set2, other);
    assert_eq!(set3, other);
    assert_eq!(set4, other);
}
#[test]
fn superset() {
    let set   = generate(vec![('3', '5'), ('2', '6')]);
    let other = generate(vec![('2', '6')]);

    assert_eq!(other, set);
}
#[test]
#[should_panic]
fn panic_on_decreasing_order() {
    generate(vec![('5', '3')]);
}
