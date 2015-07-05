use super::generate;

#[test]
fn test() {
    let set1 = generate(vec![('2', '7')]);
    let set2 = generate(vec![('0', '1'),   // disjoint left
                             ('1', '2'),   // partial overlap left
                             ('4', '5'),   // subset
                             ('7', '8'),   // partial overlap right
                             ('9', '9')]); // disjoint right
    let letters1 = generate(vec![('c', 'e')]);
    let letters2 = generate(vec![('a', 'g')]); // superset

    let difference_set     = set1.difference(&set2);
    let difference_letters = letters1.difference(&letters2);

    let other_set     = generate(vec![('3', '3'), ('6', '6')]);
    let other_letters = generate(vec![]);

    assert_eq!(difference_set, other_set);
    assert_eq!(difference_letters, other_letters);
}
