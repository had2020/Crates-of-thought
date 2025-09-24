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
pub enum Tok {
    Var(i32),
    Const(i32),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
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
    pub fn peek(&self) -> &Tok {
        &self.toks[self.i]
    }
    pub fn bump(&mut self) -> Tok {
        let t = self.toks[self.i].clone();
        self.i += 1;
        t
    }
    fn infix_bp(op: &Tok) -> Option<(u8, u8, Operator)> {
        match op {
            Tok::Plus => Some((5, 5, Operator::Add)),
            Tok::Minus => Some((5, 5, Operator::Sub)),
            Tok::Star => Some((7, 7, Operator::Mul)),
            Tok::Slash => Some((7, 7, Operator::Div)),
            _ => None,
        }
    }
    pub fn tokenize(&mut self, program: &str) -> Vec<Tok> {
        let mut toks = Vec::new();
        let mut para_fin: bool = false;
        for c in program.chars() {
            if !para_fin {
                match c {
                    ']' => para_fin = true,
                    'a'..='z' | 'A'..='Z' => self.para_keys.push(c),
                    _ => {}
                }
            } else {
                match c {
                    'a'..='z' | 'A'..='Z' => {
                        toks.push(Tok::Var(find_para_key(self.para_keys.clone(), c) as i32))
                    }
                    '0'..='9' => toks.push(Tok::Const(c.to_digit(10).unwrap() as i32)),
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
    pub fn parse_expr(&mut self, min_bp: u8) -> Ast {
        let mut lhs = match self.bump() {
            Tok::Const(n) => Ast::Value(Source::Imm, n as i32),
            Tok::Var(k) => Ast::Value(Source::Arg, k as i32),
            Tok::Minus => Ast::BinOp(
                Operator::Sub,
                Box::new(Ast::Value(Source::Imm, 0)),
                Box::new(self.parse_expr(10)),
            ),
            Tok::Plus => self.parse_expr(10),
            Tok::LParen => self.parse_expr(0),
            _ => panic!(""),
        };
        loop {
            let (lbp, rbp, op) = match Self::infix_bp(self.peek()) {
                Some(x) => x,
                None => break,
            };
            if lbp < min_bp {
                break;
            }
            self.bump();
            let rhs = self.parse_expr(rbp);
            lhs = Ast::BinOp(op, Box::new(lhs), Box::new(rhs));
        }
        lhs
    }
    pub fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }
    pub fn pass1(&mut self, program: &str) -> Ast {
        self.toks = self.tokenize(program);
        self.parse_expr(0)
    }
    pub fn pass2(&mut self, ast: &Ast) -> Ast {
        todo!();
    }
    pub fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        todo!();
    }
}
