extern mod extra;

use std::io::buffered::BufferedReader;
use std::io::stdin;

#[deriving(Clone, Eq)]
enum Tok { 
    Num(int),
    Oper(Op)
}

#[deriving(Clone, Eq)]
enum Op {
    Plus,
    Minus,
    Times
}

impl Op {
    fn interp(self, x: int, y: int) -> int {
        match self {
            Plus  => x+y,
            Minus => x-y,
            Times => x*y
        }
    }
}

#[deriving(Clone, Eq)]
enum Stack {
    Empty,
    Cons(Tok, ~Stack)
}

impl Stack {
    fn push(self, t: Tok) -> ~Stack {
        ~Cons(t, ~self)
    }    
}

fn parse(s: ~str) -> Option<~[Tok]> {
    let mut words = ~[];
    for word in s.words(){
        match word {
            "+" => words.push(Oper(Plus)),
            "-" => words.push(Oper(Minus)),
            "*" => words.push(Oper(Times)),
            _   => 
                match from_str(word) {
                    None    => return None,
                    Some(n) => words.push(Num(n))
                }
        }
    }
    Some(words)
}

fn run_rpn(toks: &[Tok]) -> Option<int> {
    let mut stack = ~Empty;
    for tok in toks.iter() {
        match (*tok, *stack) {
            (Num(_), o) => stack = o.push(*tok),
            (Oper(o), Cons(Num(x), ~Cons(Num(y), rest))) =>
                stack = ~Cons(Num(o.interp(x,y)), rest),
            _ => return None
        }
    }
    match stack {
        ~Cons(Num(x), ~Empty) => Some(x),
        _ => None
    }
}

fn main() {
    let mut stdin = BufferedReader::new(stdin());
    for line in stdin.lines() {
        //print(line);
        let ostack = parse(line);
        println!("Parsed stack: {:?}", ostack);
        match ostack {
            Some(stack) => {
                let res = run_rpn(stack);
                println!("Result: {:?}", res);
            }
            None => {}
        }
    }
}

#[test]
fn run_rpn_test() {
    assert!(run_rpn([]) == None);
    assert!(run_rpn([Num(1)]) == Some(1));
    assert!(run_rpn([Oper(Plus)]) == None);
    assert!(run_rpn([Num(1),Num(2)]) == None);
    assert!(run_rpn([Num(1), Num(2), Oper(Plus)]) == Some(3));
    assert!(run_rpn([Num(4), Num(47), Oper(Minus)]) == Some(43));
    assert!(run_rpn([Num(7), Num(2), Oper(Times)]) == Some(14));
    assert!(run_rpn([Num(1), Num(2), Oper(Plus), Num(7), Oper(Times)]) == Some(21));
}
