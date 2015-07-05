use rules::range_set::{Range, Set};

fn generate(vec: Vec<(char, char)>) -> Set {
    let mut set = Set::new();

    for (a, b) in vec {
        set.insert(Range(a, b));
    }

    set
}

mod insert;
mod remove;
mod difference;
mod intersection;
mod symmetric_difference;
mod union;
