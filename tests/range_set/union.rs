use super::generate;

#[test]
fn test() {
    let set1 = generate(vec![('1', '4'), ('5', '6')]);
    let set2 = generate(vec![('0', '5'), ('8', '9')]);

    let union = set1.union(&set2);

    let other = generate(vec![('0', '6'), ('8', '9')]);
    assert_eq!(union, other);
}
