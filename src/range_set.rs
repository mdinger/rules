#![allow(dead_code)]

use std::collections::BTreeSet;
use std::fmt::{self, Display};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Range(pub char, pub char);

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Set(BTreeSet<Range>);

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Set(ref set) = *self;
        let len = BTreeSet::len(set);

        for (count, s) in set.iter().enumerate() {
            if count < len - 1 { try!(write!(f, "{}, ", s)) }
            else { return write!(f, "{}", s) }
        }

        Ok(())
    }
}

impl Set {
    pub fn new() -> Self { Set(BTreeSet::new()) }
    pub fn insert(&mut self, value: Range) {
        let mut ret = BTreeSet::new();
        // value is a complete subset of one of the other ranges.
        let mut subset = false;

        {   let Set(ref set) = *self;

            let Range(mut min_val, mut max_val) = value;
            if min_val > max_val { panic!("First value cannot be greater than the second.") }

            for &Range(min, max) in &*set {
                // value overlaps at the beginning.
                if min_val < min && max_val >= min && max_val < max { max_val = max }
                // value overlaps at the end.
                else if min_val > min && min_val <= max && max_val > max { min_val = min }
                // value is entirely contained between min and max. Insert original
                // into new array because new is a subset.
                else if min_val > min && max_val < max {
                    ret.insert(Range(min, max)); 
                    subset = true;
                }
                // value is a superset to the current so don't add current.
                else if min_val <= min && max_val >= max {}
                // value is disjoint with current so add current.
                else { ret.insert(Range(min, max)); }
            }

            // Insert value only when it's not a subset.
            if !subset { ret.insert(Range(min_val, max_val)); }
        }

        *self = Set(ret);
    }
    pub fn union(&mut self, value: Self) {
        for x in value.0 { self.insert(x) }
    }
}


#[cfg(test)]
mod test {
    use super::{Range, Set};

    fn generate(vec: Vec<(char, char)>) -> Set {
        let mut set = Set::new();

        for (a, b) in vec {
            set.insert(Range(a, b));
        }

        set
    }
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
        let set   = generate(vec![('1', '5'), ('2', '3')]);
        let other = generate(vec![('1', '5')]);

        assert_eq!(other, set);
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
    #[test]
    fn set_union() {
        let mut set1 = generate(vec![('1', '4'), ('5', '6')]);
        let set2 =     generate(vec![('0', '5'), ('8', '9')]);

        set1.union(set2);

        let other = generate(vec![('0', '6'), ('8', '9')]);
        assert_eq!(set1, other);
    }
}
