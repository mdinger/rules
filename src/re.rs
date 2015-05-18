use parse::{self, Ast};//, Membership, Op};
use collapse;
//use std::collections::RingBuf;

struct Regex {
    ast: Ast,
    pos: usize,
}

impl Regex {
    pub fn new(s: &str) {
        let vec = parse::parse(s).unwrap();
        collapse::collapse(vec);
        
        
        //self.simplify(&mut vec)
    }
    // A regex may contain character classes which may contain set ops such
    // as `Intersection`. Character classes will have the ops applied and pushed
    // into a new vector. Everything else will be translated perfectly.
    /*fn simplify(&self, class: Vec<Ast>) -> Vec<Ast> {
        match
        let mut compact = vec![];

        for ast in class.iter_mut() {
            match ast {
                Ast::Class(ref mut class) => compact.push(self.merge_sets(class)),
                _ => compact.push(ast),
            }
        }

        Regex { ast: compact, pos: 0 }
    }
    fn unify_ranges(&self, class: Vec<Ast>) -> Vec<Ast> {
        for triple in class.windows(3) {
            
        }
    }
    fn merge_sets(&self, class: &mut RingBuf<Ast>) -> Ast {
        if class.len() < 1 { panic!("An empty character class is invalid.") }

        while class.len() > 1 {
            // Second element is always a set op.
            match class[1] {
                Op::Difference => Op::difference(&mut class),
                Op::SymmetricDifference => {
                    Op::symmetric_difference(class),
                },
                Op::Intersection => Op::intersection(class),
                Op::Union => Op::union(class),
                _ => panic("`{:?}` is not valid inside `<>` and outside `[]`."),
            }
        }
    }*/
}
