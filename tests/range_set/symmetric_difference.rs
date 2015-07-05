use super::generate;

#[test]
fn partial_overlap() {
    let set = generate(vec![('3', '6')]);

    let low  = generate(vec![('1', '4')]);
    let high = generate(vec![('5', '9')]);

    let sym_dif_low  = set.symmetric_difference(&low);
    let sym_dif_high = set.symmetric_difference(&high);

    let other_low  = generate(vec![('1', '2'), ('5', '6')]);
    let other_high = generate(vec![('3', '4'), ('7', '9')]);

    assert_eq!(sym_dif_low, other_low);
    assert_eq!(sym_dif_high, other_high);
}
#[test]
fn subset() {
    let set = generate(vec![('2', '7')]);

    let subset      = generate(vec![('3', '6')]);
    let exact_left  = generate(vec![('2', '6')]);
    let exact_right = generate(vec![('3', '7')]);

    let sym_dif_subset = set.symmetric_difference(&subset);
    let sym_dif_left   = set.symmetric_difference(&exact_left);
    let sym_dif_right  = set.symmetric_difference(&exact_right);
    let sym_dif_both   = set.symmetric_difference(&set);

    let other_subset = generate(vec![('2', '2'), ('7', '7')]);
    let other_left   = generate(vec![('7', '7')]);
    let other_right  = generate(vec![('2', '2')]);
    let other_both   = generate(vec![]);

    assert_eq!(sym_dif_subset, other_subset);
    assert_eq!(sym_dif_left, other_left);
    assert_eq!(sym_dif_right, other_right);
    assert_eq!(sym_dif_both, other_both);
}
#[test]
fn superset() {
    let set      = generate(vec![('2', '7')]);
    let superset = generate(vec![('1', '8')]);

    let sym_dif = set.symmetric_difference(&superset);

    let other = generate(vec![('1', '1'), ('8', '8')]);
    assert_eq!(sym_dif, other);
}
/*#[test]
fn disjoint() {
    let set = generate(vec![('3', '4')]);

    let low  = generate(vec![('1', '2')]);
    let high = generate(vec![('5', '6')]);

    let sym_dif_low  = set.symmetric_difference(&low);
    let sym_dif_high = set.symmetric_difference(&high);

    let other_low  = generate(vec![('1', '4')]);
    let other_high = generate(vec![('3', '6')]);

    assert_eq!(sym_dif_low, other_low);
    assert_eq!(sym_dif_high, other_high);
}*/
