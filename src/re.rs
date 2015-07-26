use parse::{self, Ast};
use collapse;

#[derive(Debug)]
pub struct Regex {
    ast: Vec<Ast>,
    chars: Vec<char>,
    pos_ast: usize,
    pos_char: usize,
}

impl Regex {
    fn ast_contains(&self) -> bool {
        let cur = self.cur_char();

        match self.cur_ast() {
            Ast::Char(c) => c == cur,
            Ast::Literal(_) => unimplemented!(),
            _ => unimplemented!(),
        }
    }
    fn cur_ast(&self) -> Ast { self.ast[self.pos_ast].clone() }
    fn cur_char(&self) -> char { self.chars[self.pos_char] }
    pub fn new(s: &str) -> Regex {
        let vec = parse::parse(s).unwrap();
        let vec = collapse::collapse(vec);
        
        Regex { ast: vec, chars: vec![], pos_ast: 0, pos_char: 0 }
    }
    fn next_ast(&mut self) -> bool {
        self.pos_ast += 1;

        self.pos_ast != self.ast.len()
    }
    fn next_char(&mut self) -> bool {
        self.pos_char += 1;

        self.pos_char != self.chars.len()
    }
    pub fn is_match(&mut self, s: &str) -> bool {
        self.chars = s.chars().collect();

        loop {
            if self.ast_contains() {
                if !self.next_ast() { return true }
            } else { self.pos_ast = 0 }

            if !self.next_char() { break }
        }

        false
    }
}
