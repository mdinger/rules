use parse::Ast;
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
                Ast::Class(mut deque) => Some(self.collapse_class(&mut deque)),
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
    fn collapse_class(&mut self, deque: &mut VecDeque<Ast>) -> Ast {
        let mut left = deque.pop_front()
                            .unwrap();

        while let Some(op) = deque.pop_front() {
            let right = deque.pop_front()
                             .unwrap();

            left = match op {
                Ast::Op(op) => op.apply(left, right),
                // Only operators should ever appear here.
                _ => unreachable!(),
            };
        }

        // Empty intersections like `< & [a] >` are not allowed.
        if let Ast::Empty = left { panic!("An empty class `<[]>` is not allowed!") }

        left
    }
    fn next(&mut self) -> bool {
        self.pos += 1;

        self.pos != self.data.len()
    }
}
