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
            let a = self.cur();
            vec.push(match a {
                Ast::Char(_)      => a,
                Ast::Class(mut deque) => self.collapse_class(&mut deque),
                _ => panic!("incomplete"),
            });

            if !self.next() { break }
        }

        self.data = vec;
    }
    // I think this entire thing might be replaceable with a fold but
    // I was running into issues when testing it. `chunks` returns references
    // and DequeVec doesn't implement Deref so I can't call chunks on it. Maybe
    // in the future.
    fn collapse_class(&mut self, deque: &mut VecDeque<Ast>) -> Ast {
        let mut left = deque.pop_front().unwrap();

        while let Some(op) = deque.pop_front() {
            let right = deque.pop_front().unwrap();

            left = match op {
                Ast::Op(op) => op.apply(left, right),
                // Only operators should ever appear here.
                e => unreachable!(),
            };
        }

        let mut ret = VecDeque::new();
        ret.push_front(left);

        Ast::Class(ret)
    }
    fn next(&mut self) -> bool {
        self.pos += 1;

        self.pos != self.data.len()
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use collapse;
    use parse;
    use parse::Ast;
    use parse::Ast::*;
    use parse::Membership::*;
    use parse::ToCharSet;

    fn new_deque(vec: Vec<Ast>) -> VecDeque<Ast> {
        let deque: VecDeque<Ast> = vec.into_iter().collect();

        deque
    }
    fn simplify(s: &str) -> Vec<Ast> {
        collapse::collapse(parse::parse(s).unwrap())
    }

    #[test]
    fn text() {
        assert_eq!(vec![Char('f'), Char('r'), Char('e'), Char('d')], simplify(r"fred"));
    }
    #[test]
    fn empty_unions() {
        // Set of chars inside `[]`
        let set = vec![Char('a')].to_char_set();
        // Deque of ops and sets inside `<>`
        let deque = new_deque(vec![Set(set, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(deque.clone())], simplify(r"< + [ a ] >"));
        assert_eq!(vec![Class(deque)], simplify(r"<[ a ] + >"));
    }
    #[test]
    fn char_class_set_union() {
        // Set of chars inside `[]`
        let set = vec![Char('a'), Char('b'), Char('c')].to_char_set();
        // Deque of ops and sets inside `<>`
        let deque = new_deque(vec![Set(set, Inclusive)]);
        // A single class which is the union of all subsets.
        assert_eq!(vec![Class(deque)], simplify(r"< [ abab ] + [ bc ] + [ abc ] >"));
    }
    #[test]
    fn char_class_set_difference() {
        // Set of chars inside `[]`
        let set = vec![Char('a')].to_char_set();
        // Deque of ops and sets inside `<>`
        let deque_ex = new_deque(vec![Set(set.clone(), Exclusive)]);
        let deque_in = new_deque(vec![Set(set, Inclusive)]);
        // A single class which is the union of all subsets.
        assert_eq!(vec![Class(deque_ex)], simplify(r"< - [ a ] >"));
        assert_eq!(vec![Class(deque_in)], simplify(r"< [ abc ] - [ b ] - [ c ] - >"));
    }
}