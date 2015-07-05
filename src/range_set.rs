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
