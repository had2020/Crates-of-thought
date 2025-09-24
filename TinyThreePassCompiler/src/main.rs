use preloaded::{Ast, Operator, Source};
pub struct Compiler {
    para_keys: Vec<char>,
    toks: Vec<Tok>,
    i: usize,
}
impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, PartialEq)]
enum Tok {
    Var(usize),
    Const(u32),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}
pub fn infix_bp(op: &Tok) -> Option<(u8, u8)> {
    match op {
        Tok::Plus => Some((5, 5)),
        Tok::Minus => Some((5, 5)),
        Tok::Star => Some((7, 7)),
        Tok::Slash => Some((7, 7)),
        _ => None,
    }
}
pub fn find_para_key(para_keys: Vec<char>, key: char) -> usize {
    let mut iter: usize = 0;
    for i in 0..para_keys.len() {
        if para_keys[i] == key {
            break;
        }
        iter += 1;
    }
    iter as usize
}
impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            para_keys: Vec::new(),
            toks: Vec::new(),
            i: 0,
        }
    }
    pub fn tokenize(&mut self, program: &str) -> Vec<Tok> {
        let mut toks = Vec::new();
        let mut para_fin: bool = false;
        for c in program.chars() {
            match c {
                ']' => break,
                'a'..='z' | 'A'..='Z' => self.para_keys.push(c),
                _ => {}
            }
        }
        while let Some(c) = program.chars().next() {
            if !para_fin {
                match c {
                    ']' => break,
                    'a'..='z' | 'A'..='Z' => self.para_keys.push(c),
                    _ => {}
                }
            } else {
                match c {
                    'a'..='z' | 'A'..='Z' => toks.push(Tok::Var(find_para_key(self.para_keys, c))),
                    '0'..='9' => toks.push(Tok::Const(c.to_digit(10).unwrap())),
                    '+' => toks.push(Tok::Plus),
                    '-' => toks.push(Tok::Minus),
                    '*' => toks.push(Tok::Star),
                    '/' => toks.push(Tok::Slash),
                    '(' => toks.push(Tok::LParen),
                    ')' => toks.push(Tok::RParen),
                    _ => {}
                }
            }
        }
        toks
    }
    pub fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }
    pub fn pass1(&mut self, program: &str) -> Ast {
        let tokens = self.tokenize(program);
    }
    pub fn pass2(&mut self, ast: &Ast) -> Ast {
        todo!();
    }
    pub fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        todo!();
    }
}
