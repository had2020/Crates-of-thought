You are writing a three-pass compiler for a simple programming language into a small assembly language.

The programming language has this syntax:

    function   ::= '[' arg-list ']' expression

    arg-list   ::= /* nothing */
                 | variable arg-list

    expression ::= term
                 | expression '+' term
                 | expression '-' term

    term       ::= factor
                 | term '*' factor
                 | term '/' factor

    factor     ::= number
                 | variable
                 | '(' expression ')'
Variables are strings of alphabetic characters. Numbers are strings of decimal digits representing integers. So, for example, a function which computes a2 + b2 might look like:

    [ a b ] a*a + b*b
A function which computes the average of two numbers might look like:

    [ first second ] (first + second) / 2
You need write a three-pass compiler. All test cases will be valid programs, so you needn't concentrate on error-handling.

The first pass will be the method pass1 which takes a string representing a function in the original programming language and will return a (JSON) object that represents that Abstract Syntax Tree. The Abstract Syntax Tree must use the following representations:

// Each node is an enum of type `AST` with the following fields:
Ast::BinOp(Operator::Add, Box::new(a), Box::new(b)) // add subtree a to subtree b
Ast::BinOp(Operator::Sub, Box::new(a), Box::new(b)) // subtract subtree b from subtree a
Ast::BinOp(Operator::Mul, Box::new(a), Box::new(b)) // multiply subtree a by subtree b
Ast::BinOp(Operator::Div, Box::new(a), Box::new(b)) // divide subtree a from subtree b
Ast::Value(Source::Arg, n) // reference to n-th argument, n integer
Ast::Value(Source::Imm, n) // immediate value n, n integer
// Note: A convenience method Ast::binop(op, a, b) is provided that will box the two
// subtrees.
Note: arguments are indexed from zero. So, for example, the function

[ x y ] ( x + y ) / 2 would look like:

use {Operator::*, Source::*};
Ast::binop(
    Div,
    Ast::binop(Add, Ast::Value(Arg, 0), Ast::Value(Arg, 1)),
    Ast::Value(Imm, 2),
)
The second pass of the compiler will be called pass2. This pass will take the output from pass1 and return a new Abstract Syntax Tree (with the same format) with all constant expressions reduced as much as possible. So, if for example, the function is [ x ] x + 2*5, the result of pass1 would be:

use {Operator::*, Source::*};
Ast::binop(
    Add,
    Ast::Value(Arg, 0),
    Ast::binop(Mul, Ast::Value(Imm, 2), Ast::Value(Imm, 5)),
)
This would be passed into pass2 which would return:

use {Operator::*, Source::*};
Ast::binop(Add, Ast::Value(Arg, 0), Ast::Value(Imm, 10))
The third pass of the compiler is pass3. The pass3 method takes in an Abstract Syntax Tree and returns an array of strings. Each string is an assembly directive. You are working on a small processor with two registers (R0 and R1), a stack, and an array of input arguments. The result of a function is expected to be in R0. The processor supports the following instructions:

    "IM n"     // load the constant value n into R0
    "AR n"     // load the n-th input argument into R0
    "SW"       // swap R0 and R1
    "PU"       // push R0 onto the stack
    "PO"       // pop the top value off of the stack into R0
    "AD"       // add R1 to R0 and put the result in R0
    "SU"       // subtract R1 from R0 and put the result in R0
    "MU"       // multiply R0 by R1 and put the result in R0
    "DI"       // divide R0 by R1 and put the result in R0
So, one possible return value from pass3 given the Abstract Syntax Tree shown above from pass2 is:

    [ "IM 10", "SW", "AR 0", "AD" ]
Here is a simulator for the target machine. It takes an array of assembly instructions and an array of arguments and returns the result.

fn simulate(assembly: &Vec<String>, argv: Vec<i32>) -> i32 {
    let mut r = (0, 0);
    let mut stack: Vec<i32> = vec![];
    let num = |opt: Option<&str>| opt.unwrap().parse::<i32>().unwrap();

    for ins in assembly {
        let mut ws = ins.split_whitespace();
        match ws.next() {
            Some("IM") => r.0 = num(ws.next()),
            Some("AR") => r.0 = argv[num(ws.next()) as usize],
            Some("SW") => r = (r.1, r.0),
            Some("PU") => stack.push(r.0),
            Some("PO") => r.0 = stack.pop().unwrap(),
            Some("AD") => r.0 += r.1,
            Some("SU") => r.0 -= r.1,
            Some("MU") => r.0 *= r.1,
            Some("DI") => r.0 /= r.1,
            _ => panic!("Invalid instruction encountered"),
        }
    }
    r.0
}
