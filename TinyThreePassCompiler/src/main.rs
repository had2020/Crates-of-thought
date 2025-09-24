use preloaded::{Ast, Operator, Source};
pub struct Compiler {
    para_keys: Vec<char>,
}
impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, PartialEq)]
enum Tok {
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
pub fn find_para_key(para_keys: Vec<char>, key: char) -> i32 {
    let mut iter: usize = 0;
    for i in 0..para_keys.len() {
        if para_keys[i] == key {
            break;
        }
        iter += 1;
    }
    iter as i32
}
pub fn tokenize(program: &str) -> Vec<Tok> {
    let mut toks = Vec::new();
    while let Some(c) = program.chars().next() {
        match c {
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
    toks
}
impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            para_keys: Vec::new(),
        }
    }
    pub fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }
    pub fn pass1(&mut self, program: &str) -> Ast {
        let tokens = tokenize(program);
    }
    pub fn pass2(&mut self, ast: &Ast) -> Ast {
        todo!();
    }
    pub fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        todo!();
    }
}
