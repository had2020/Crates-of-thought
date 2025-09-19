use preloaded::{Ast, Operator, Source};
pub struct Compiler {
    para_keys: Vec<char>,
}
impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(PartialEq)]
pub enum Op {
    Par,
    Mut,
    Div,
    Add,
    Sub,
    Missing,
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
pub fn to_binop(cur_op: (Op, &str, &str), para_keys: Vec<char>) -> Ast {
    let oper: Operator = match cur_op.0 {
        Op::Add => Operator::Add,
        Op::Sub => Operator::Sub,
        Op::Mut => Operator::Mul,
        Op::Div => Operator::Div,
        _ => Operator::Sub,
    };
    let operhand1: i32 = find_para_key(para_keys, cur_op.1.chars().nth(0).unwrap());
    let operhand2: i32 = find_para_key(para_keys, cur_op.2.chars().nth(0).unwrap());
    match (
        cur_op.1.chars().nth(0).unwrap().is_ascii_alphabetic(),
        cur_op.2.chars().nth(0).unwrap().is_ascii_alphabetic(),
    ) {
        (true, true) => Ast::binop(
            oper,
            Ast::Value(Source::Arg, operhand1),
            Ast::Value(Source::Arg, operhand2),
        ),
        (true, false) => Ast::binop(
            oper,
            Ast::Value(Source::Arg, operhand1),
            Ast::Value(
                Source::Imm,
                cur_op.2.chars().nth(0).unwrap().to_digit(10).unwrap() as i32,
            ),
        ),
        (false, true) => Ast::binop(
            oper,
            Ast::Value(
                Source::Imm,
                cur_op.1.chars().nth(0).unwrap().to_digit(10).unwrap() as i32,
            ),
            Ast::Value(Source::Arg, operhand2),
        ),
        (false, false) => Ast::binop(
            oper,
            Ast::Value(
                Source::Imm,
                cur_op.1.chars().nth(0).unwrap().to_digit(10).unwrap() as i32,
            ),
            Ast::Value(
                Source::Imm,
                cur_op.2.chars().nth(0).unwrap().to_digit(10).unwrap() as i32,
            ),
        ),
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
    pub fn pass1(&mut self, program: &str) -> Ast {
        let mut syn_tree: Vec<Vec<Ast>> = Vec::new();
        let mut cur_op: (Op, &str, &str) = (Op::Missing, "", "");
        let mut pemdas: Op = Op::Par;
        let tokens = self.tokenize(program); // Pemdas
        let mut fin_paras: bool = false;
        while pemdas != Op::Missing {
            for t in tokens {
                if !fin_paras && t.chars().nth(0).unwrap().is_ascii_alphabetic() {
                    self.para_keys.push(t.chars().nth(0).unwrap());
                } else if !fin_paras && t == "]" {
                    fin_paras = true;
                } else if fin_paras {
                    match (t.chars().nth(0).unwrap(), pemdas) {
                        ('a'..='z' | 'A'..='Z' | '0'..='9', _) => {
                            if cur_op.1 == "" {
                                for i in 0..self.para_keys.len() {
                                    cur_op.1 = t;
                                }
                            } else if cur_op.2 == "" {
                                for i in 0..self.para_keys.len() {
                                    cur_op.2 = t;
                                }
                            } else {
                                syn_tree.push(Vec::new());
                            }
                        }
                        ('(' | ')', Op::Par) => syn_tree.push(Vec::new()),
                        ('*', Op::Mut) => {
                            if cur_op.0 == Op::Missing {
                                cur_op.0 = Op::Mut;
                            }
                        }
                        ('/', Op::Div) => {
                            if cur_op.0 == Op::Missing {
                                cur_op.0 = Op::Div;
                            }
                        }
                        ('+', Op::Add) => {
                            if cur_op.0 == Op::Missing {
                                cur_op.0 = Op::Add;
                            }
                        }
                        ('-', Op::Sub) => {
                            if cur_op.0 == Op::Missing {
                                cur_op.0 = Op::Sub;
                            }
                        }
                        _ => {}
                    }
                }
            }
            pemdas = match pemdas {
                Op::Par => Op::Mut,
                Op::Mut => Op::Div,
                Op::Div => Op::Add,
                Op::Add => Op::Sub,
                Op::Sub => Op::Missing,
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
