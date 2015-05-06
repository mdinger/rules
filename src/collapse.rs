use parse::Ast;
use parse::Ast::*;//, Membership, Op};

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
    fn collapse(&mut self){
        let mut vec = vec![];

        loop {
            let a = self.cur();
            match a {
                Char(_) => vec.push(a),
                _ => panic!("incomplete"),
            }

            if !self.next() { break }
        }

        self.data = vec;
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
        collapse::collapse(parse::parse(s))
    }

    #[test]
    fn text() {
        assert_eq!(vec![Char('f'), Char('r'), Char('e'), Char('d')], simplify(r"fred"));
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
