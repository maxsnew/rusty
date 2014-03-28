extern mod extra;

use std::io::buffered::BufferedReader;
use std::io::stdin;
use std::vec::append;

#[deriving(Clone, Eq)]
enum Tok { 
    Num(int),
    Plus
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
        if word == "+" {
            words.push(Plus)
        } else {
            match from_str(word) {
                None    => return None,
                Some(n) => words.push(Num(n))
            }
        }
    }
    Some(words)
}

fn step(tok: Tok, toks: &[Tok]) -> Option<~[Tok]> {
    None
}

fn run_rpn(toks: &[Tok]) -> Option<int> {
    let mut stack = ~Empty;
    for tok in toks.iter() {
        match (*tok, *stack) {
            (Num(n), o) => stack = o.push(*tok),
            (Plus, Cons(Num(x), ~Cons(Num(y), rest))) =>
                stack = ~Cons(Num(x+y), rest),
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
    assert!(run_rpn([Plus]) == None);
    assert!(run_rpn([Num(1),Num(2)]) == None);
    assert!(run_rpn([Num(1), Num(2), Plus]) == Some(3));
    assert!(run_rpn([Num(1), Num(2), Plus, Num(7), Plus]) == Some(10));
}
