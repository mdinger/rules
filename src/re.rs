use parse::{self, Ast};
use collapse;

#[derive(Debug)]
pub struct Regex(Vec<Ast>);

impl Regex {
    pub fn new(s: &str) -> Regex {
        let vec = parse::parse(s).unwrap();
        let vec = collapse::collapse(vec);

        Regex(vec)
    }
    pub fn is_match(&self, s: &str) -> bool {
        let mut pair = Pair { ast: self.0.clone(), pos_ast: 0,
                              chars: s.chars().collect(), pos_char: 0 };
        let mut count = 0;

        loop {
            if pair.matches_ast() {
                if !pair.next_ast() { return true }
            } else { pair.pos_ast = 0 }

            if !pair.next_char() { break }
            count += 1;
            if count > 20 { break }
        }

        false
    }
}

// A structure to allow a Regex to determine if it matches the &str.
#[derive(Debug)]
struct Pair {
    ast: Vec<Ast>,
    pos_ast: usize,
    chars: Vec<char>,
    pos_char: usize,
}

impl Pair {
    fn cur_ast(&self) -> Ast { self.ast[self.pos_ast].clone() }
    fn cur_char(&self) -> char { self.chars[self.pos_char] }
    fn matches_ast(&mut self) -> bool {
        let cur = self.cur_char();

        match self.cur_ast() {
            Ast::Char(c) => c == cur,
            Ast::Literal(ref lit) => {
                let str_lit: String = lit.iter().map(|&x| x).collect();
                let str_chars: String = self.chars[self.pos_char..].iter().map(|&x| x).collect();

                if let Some(start_pos) = str_chars.find(&str_lit) {
                    self.pos_char += start_pos + str_lit.len() - 1;

                    true
                } else { false }
            },
            _ => unimplemented!(),
        }
    }
    fn next_ast(&mut self) -> bool {
        self.pos_ast += 1;

        self.pos_ast != self.ast.len()
    }
    fn next_char(&mut self) -> bool {
        self.pos_char += 1;

        self.pos_char != self.chars.len()
    }
}
