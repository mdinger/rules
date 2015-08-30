use parse::{Ast, Sign};
use std::collections::VecDeque;

pub fn collapse(v: Vec<Ast>) -> Vec<Ast> {
    let mut c = Collapser { data: v, pos: 0 };
    c.collapse();

    let Collapser { data, .. } = c;

    data
}

#[derive(Debug)]
struct Collapser {
    data: Vec<Ast>,
    pos: usize,
}

impl Collapser {
    fn cur(&self)  -> Ast { self.data[self.pos].clone() }
    fn collapse(&mut self) {
        let mut vec = vec![];

        loop {
            let cur = self.cur();
            let ast = match cur {
                Ast::Char(_) |
                Ast::Literal(_) => Some(cur),
                Ast::CharClass(mut deque, sign) => Some(self.collapse_char_class(&mut deque, sign)),
                Ast::Empty => None,
                _ => unimplemented!(),
            };

            if let Some(val) = ast { vec.push(val) }

            if !self.next() { break }
        }

        self.data = vec;
    }
    // I think this entire thing might be replaceable with a fold but
    // I was running into issues when testing it. `chunks` returns references
    // and DequeVec doesn't implement Deref so I can't call chunks on it. Maybe
    // in the future.
    fn collapse_char_class(&mut self, deque: &mut VecDeque<Ast>, sign: Sign) -> Ast {
        let mut left = deque.pop_front()
                            .expect("Class must have at least one element");

        while let Some(op) = deque.pop_front() {
            let right = deque.pop_front()
                             .expect("A set operator must be followed by another set");

            left = match op {
                // Only operators should appear here.
                Ast::Op(op) => op.apply(left, right),
                _ => unreachable!(),
            };
        }

        // Empty intersections like `< & [a] >` are not allowed.
        if let Ast::Empty = left { panic!("An empty class `<[]>` is not allowed!") }

        if sign == Sign::Negative { left = left.negate() }

        left
    }
    fn next(&mut self) -> bool {
        self.pos += 1;

        self.pos != self.data.len()
    }
}
