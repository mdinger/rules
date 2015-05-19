use parse;
use parse::Ast;
use parse::Ast::*;//, Membership, Op};
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
                Char(_)      => a,
                Class(mut deque) => self.collapse_class(&mut deque),
                _ => panic!("incomplete"),
            });

            if !self.next() { break }
        }

        self.data = vec;
    }
    fn collapse_class(&mut self, deque: &mut VecDeque<Ast>) -> Ast {
        let mut left = deque.pop_front().unwrap();

        while let Some(op) = deque.pop_front() {
            let right = deque.pop_front().unwrap();

            left = parse::apply_op(&op, left, right);
        }

        let mut ret = VecDeque::new();
        ret.push_front(left);

        Class(ret)
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

    /*#[test]
    fn char_class_union() {
        // Set of chars inside `[]`
        let set = vec![Char('a'), Char('b')].to_char_set();
        // Deque of ops and sets inside `<>`
        let deque = new_deque(vec![Set(set, Inclusive)]);
        // A single class denoted by `<[]>`
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(deque)], collapse(parse(r"< [ a ] + [ b ] >")));
    }*/
}
