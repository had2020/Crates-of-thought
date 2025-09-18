use preloaded::{Ast, Operator, Source};
pub struct Compiler {
    para_keys: Vec<char>,
}
impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            para_keys: Vec::new(),
        }
    }

    fn tokenize<'a>(&self, program: &'a str) -> Vec<&'a str> {
        let mut tokens: Vec<&str> = vec![];

        let mut iter = program.chars();
        while let Some(c) = iter.clone().next() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let word = iter.as_str();
                    while iter
                        .clone()
                        .next()
                        .map_or(false, |c| c.is_ascii_alphabetic())
                    {
                        iter.next();
                    }
                    tokens.push(&word[..word.len() - iter.as_str().len()]);
                }
                '0'..='9' => {
                    let number = iter.as_str();
                    while iter.clone().next().map_or(false, |c| c.is_ascii_digit()) {
                        iter.next();
                    }
                    tokens.push(&number[..number.len() - iter.as_str().len()]);
                }
                ' ' => {
                    iter.next();
                }
                _ => {
                    tokens.push(&iter.as_str()[..iter.next().unwrap().len_utf8()]);
                }
            }
        }

        tokens
    }
    pub fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }
    pub enum Op {
        Par,
        Mut,
        Div,
        Add,
        Sub,
    }
    pub fn pass1(&mut self, program: &str) -> Ast {
        let mut syntax_tree: vec<Ast> = Vec::new();
        let mut cur_binop: (Op, )
        let mut pemdas: Op = Op::Par;
        let tokens = self.tokenize(program); // Pemdas
        let mut fin_paras: bool = false;
        for t in tokens {
            if !fin_paras && t.chars().nth(0).unwrap().is_ascii_alphabetic() {
                self.para_keys.push(t.chars().nth(0).unwrap());
            } else if !fin_paras && t == "]" {
                fin_paras = true;
            } else if fin_paras {
                match t.chars().nth(0).unwrap() {
                    'a'..='z' | 'A'..='Z' => {},
                    '0'..='9' => {},
                    '(' => {},
                    ')' => {},
                    '*' => {},
                    '/' => {},
                    '+' => {},
                    '-' => {},
                    _ => {},
                }
            }
        }
    }

    pub fn pass2(&mut self, ast: &Ast) -> Ast {
        todo!();
    }

    pub fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        todo!();
    }
}
