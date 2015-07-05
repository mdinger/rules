#![allow(dead_code)]

//! A set library to aid character set manipulation.
//!
//! `range_set` aims to make it easier to handle set manipulation for characters
//! over ranges. For example, a unicode library may expose character ranges such
//! as `('0', '9')` as a sequence of digits. If I was already later state I would
//! like to add the sequence of digits: `('1', '3')`, it would consider them as
//! distinct and store both. This is a nuisance. It should recognize that `1-3`
//! is encased inside `0-9` and leave it as is.
//!
//! It provides the standard set operations: union, intersection, difference,
//! and symmetric difference.

use std::collections::BTreeSet;
use std::fmt::{self, Display};
use parse::NextPrev;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Range(pub char, pub char);

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

        // Borrowing self blocks later operation. Add a new scope.
        {   let Set(ref set) = *self;

            let Range(mut min_val, mut max_val) = value;
            if min_val > max_val { panic!("First value cannot be greater than the second.") }

            // Loop over set adding old disjoint pieces and supersets back.
            // When partially overlapped, expand value to the union. At the
            // end, insert union after it has been fully expanded.
            for &Range(min, max) in &*set {
                // value overlaps at the beginning.
                if min_val < min && max_val >= min && max_val < max { max_val = max }
                // value overlaps at the end.
                else if min_val > min && min_val <= max && max_val > max { min_val = min }
                // value is entirely contained between min and max. Insert original
                // into new array because new is a subset.
                else if min_val >= min && max_val <= max {
                    ret.insert(Range(min, max));
                    subset = true;
                }
                // value is a superset to the current so don't add current.
                else if min_val < min && max_val > max {}
                // value is disjoint with current so add current.
                else { ret.insert(Range(min, max)); }
            }

            // Insert value only when it's not a subset.
            if !subset { ret.insert(Range(min_val, max_val)); }
        }

        *self = Set(ret);
    }
    pub fn remove(&mut self, value: Range) {
        let mut ret = BTreeSet::new();

        // Borrowing self blocks later modification. Make a new scope to contain it.
        {   let Set(ref set) = *self;

            let Range(min_val, max_val) = value;
            if min_val > max_val { panic!("First value cannot be greater than the second.") }

            // Loop over set inserting whatever doesn't intersect.
            for &Range(min, max) in &*set {
                // value overlaps at the beginning.
                if min_val <= min && max_val >= min && max_val < max { ret.insert(Range(max_val.next(), max)); }
                // value overlaps at the end.
                else if min_val > min && min_val <= max && max_val >= max { ret.insert(Range(min, min_val.prev())); }
                // value is entirely contained between min and max. Split set
                // into two pieces.
                else if min_val > min && max_val < max {
                    ret.insert(Range(min, min_val.prev()));
                    ret.insert(Range(max_val.next(), max));

                    // Current piece was a superset so value cannot be anywhere else.
                    break;
                // value is a superset to the current so don't add current.
                } else if min_val <= min && max_val >= max {}
                // value is disjoint with current so add current.
                else { ret.insert(Range(min, max)); }
            }
        }

        *self = Set(ret)
    }
    // 123 + 345 = 12345.
    pub fn union(&self, value: &Self) -> Self {
        let mut ret = self.clone();

        // Loop over the btreeset of Range(char, char).
        for &x in &value.0 { ret.insert(x) }

        ret
    }
    // Intersection of `A` & `B` is `A - (A - B)`: 123 & 345 = 3.
    pub fn intersection(&self, value: &Self) -> Self {
        let diff = self.difference(value);

        self.difference(&diff)
    }
    // 123 - 345 = 12.
    pub fn difference(&self, value: &Self) -> Self {
        let mut ret = self.clone();

        for &x in &value.0 { ret.remove(x) }

        ret
    }
    // `A` ^ `B` is `(A + B) - (A & B)`: 123 ^ 345 = 1245.
    pub fn symmetric_difference(&self, value: &Self) -> Self {
        let union = self.union(value);
        let intersection = self.intersection(value);

        union.difference(&intersection)
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
    fn insert_partial_overlap() {
        let set1 = generate(vec![('3', '5'), ('4', '6')]);
        let set2 = generate(vec![('3', '5'), ('1', '4')]);

        let other1 = generate(vec![('3', '6')]);
        let other2 = generate(vec![('1', '5')]);

        assert_eq!(other1, set1);
        assert_eq!(other2, set2);
    }
    #[test]
    fn insert_subset() {
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
    fn insert_superset() {
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
        let set1 = generate(vec![('1', '4'), ('5', '6')]);
        let set2 = generate(vec![('0', '5'), ('8', '9')]);

        let union = set1.union(&set2);

        let other = generate(vec![('0', '6'), ('8', '9')]);
        assert_eq!(union, other);
    }
    #[test]
    fn remove_partial_overlap() {
        let mut set1 = generate(vec![('5', '9')]);
        set1.remove(Range('3', '6')); // beginnings different.

        let mut set2 = generate(vec![('5', '9')]);
        set2.remove(Range('5', '6')); // beginnings same.

        let mut set3 = generate(vec![('2', '6')]);
        set3.remove(Range('5', '9')); // end different.

        let mut set4 = generate(vec![('2', '6')]);
        set4.remove(Range('5', '6')); // end same.

        let other12 = generate(vec![('7', '9')]);
        let other34 = generate(vec![('2', '4')]);

        assert_eq!(set1, other12);
        assert_eq!(set2, other12);
        assert_eq!(set3, other34);
        assert_eq!(set4, other34);
    }
    #[test]
    fn remove_subset() {
        let mut set = generate(vec![('1', '9')]);
        set.remove(Range('3', '6'));

        let other = generate(vec![('1', '2'), ('7', '9')]);

        assert_eq!(set, other);
    }
    #[test]
    fn remove_superset() {
        let mut set = generate(vec![('5', '6')]);
        set.remove(Range('3', '8'));

        let other = generate(vec![]);
        assert_eq!(set, other);
    }
    #[test]
    fn remove_disjoint() {
        let mut set = generate(vec![('2', '3')]);
        set.remove(Range('6', '8'));

        let other = generate(vec![('2', '3')]);
        assert_eq!(set, other);
    }
    #[test]
    fn set_difference() {
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
    #[test]
    fn set_intersection_partial_overlap() {
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
    fn set_intersection_subset() {
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
    fn set_intersection_superset() {
        let set      = generate(vec![('2', '7')]);
        let superset = generate(vec![('1', '8')]);

        let intersection = set.intersection(&superset);

        let other = generate(vec![('2', '7')]);
        assert_eq!(intersection, other);
    }
    #[test]
    fn set_intersection_disjoint() {
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
    #[test]
    fn set_symmetric_difference_partial_overlap() {
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
    fn set_symmetric_difference_subset() {
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
    fn set_symmetric_difference_superset() {
        let set      = generate(vec![('2', '7')]);
        let superset = generate(vec![('1', '8')]);

        let sym_dif = set.symmetric_difference(&superset);

        let other = generate(vec![('1', '1'), ('8', '8')]);
        assert_eq!(sym_dif, other);
    }
    fn set_symmetric_difference_disjoint() {
        let set = generate(vec![('3', '4')]);

        let low  = generate(vec![('1', '2')]);
        let high = generate(vec![('5', '6')]);

        let sym_dif_low  = set.symmetric_difference(&low);
        let sym_dif_high = set.symmetric_difference(&high);

        let other_low  = generate(vec![('1', '4')]);
        let other_high = generate(vec![('3', '6')]);

        assert_eq!(sym_dif_low, other_low);
        assert_eq!(sym_dif_high, other_high);
    }
}
