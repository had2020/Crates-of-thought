use preloaded::{Ast, Operator, Source};

pub struct Compiler {}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
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

    pub fn pass1(&mut self, program: &str) -> Ast {
        let tokens = self.tokenize(program);
        let mut iter = tokens.iter().peekable(); // PANDAS
        let mut r: Ast = ();
        while let Some(&c) = iter.peek() {
            match &c {
                _ => {}
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
