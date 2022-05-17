use std::{collections::HashMap, fmt::Display, iter::Peekable};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Sym(String),
    // Functor (self-referential type)
    Fun(String, Vec<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Expr::Sym(name) => write!(f, "{}", name),
            Expr::Fun(name, args) => {
                write!(f, "{}(", name)?;
                for (index, arg) in args.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rule {
    head: Expr,
    body: Expr,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} = {}", self.head, self.body)
    }
}

fn substitute_bindings(bindings: &Bindings, expr: &Expr) -> Expr {
    use Expr::*;
    match expr {
        Sym(name) => {
            if let Some(value) = bindings.get(name) {
                value.clone()
            } else {
                expr.clone()
            }
        }
        Fun(name, args) => {
            let new_name = match bindings.get(name) {
                Some(Sym(new_name)) => new_name.clone(),
                None => name.clone(),
                Some(_) => panic!("expected symbol in the place of the functor name"),
            };
            let mut new_args = Vec::new();
            for arg in args {
                new_args.push(substitute_bindings(bindings, arg));
            }
            Fun(new_name, new_args)
        }
    }
}

impl Rule {
    #[allow(dead_code)]
    fn apply_all(&self, expr: &Expr) -> Expr {
        use Expr::*;
        if let Some(bindings) = pattern_match(&self.head, &self.body) {
            println!("match: {:?}", bindings);
            substitute_bindings(&bindings, expr)
        } else {
            match expr {
                Sym(_) => expr.clone(),
                Fun(name, args) => {
                    let mut new_args = Vec::new();
                    for arg in args {
                        new_args.push(self.apply_all(arg))
                    }
                    Fun(name.clone(), new_args)
                }
            }
        }
    }
}

type Bindings = HashMap<String, Expr>;

fn pattern_match(pattern: &Expr, value: &Expr) -> Option<Bindings> {
    fn pattern_match_impl(pattern: &Expr, value: &Expr, bindings: &mut Bindings) -> bool {
        use Expr::*;
        match (pattern, value) {
            (Sym(name), _) => {
                if let Some(bound_value) = bindings.get(name) {
                    bound_value == value
                } else {
                    bindings.insert(name.clone(), value.clone());
                    true
                }
            }
            (Fun(name1, args1), Fun(name2, args2)) => {
                if name1 == name2 && args1.len() == args2.len() {
                    for i in 0..args1.len() {
                        if !pattern_match_impl(&args1[i], &args2[i], bindings) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            (Fun(_, _), Sym(_)) => false,
        }
    }

    let mut bindings = HashMap::new();
    if pattern_match_impl(pattern, value, &mut bindings) {
        Some(bindings)
    } else {
        None
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum TokenKind {
    Sym(String),
    OpenParen,
    CloseParen,
    Comma,
    Equals,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    kind: TokenKind,
    text: String,
}

#[allow(dead_code)]
struct Lexer<Chars: Iterator<Item = char>> {
    chars: Peekable<Chars>,
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {
    fn from_iter(chars: Chars) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Lexer<Chars> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    for token in Lexer::from_iter("swap(pair(a, b)) = pair(b, a)".chars()) {
        println!("{:?}", token);
    }

    use Expr::*;
    // swap(pair(a, b)) = pair(b, a)
    let swap = Rule {
        head: Fun(
            "swap".to_string(),
            vec![Fun(
                "pair".to_string(),
                vec![Sym("a".to_string()), Sym("b".to_string())],
            )],
        ),
        body: Fun(
            "pair".to_string(),
            vec![Sym("b".to_string()), Sym("a".to_string())],
        ),
    };

    let expr = Fun(
        "foo".to_string(),
        vec![Fun(
            "swap".to_string(),
            vec![Fun(
                "pair".to_string(),
                vec![
                    Fun("f".to_string(), vec![Sym("a".to_string())]),
                    Fun("g".to_string(), vec![Sym("b".to_string())]),
                ],
            )],
        )],
    );

    println!("rule  : {}", swap);
    println!("expr  : {}", expr);
    println!("expr' : {}", swap.apply_all(&expr));

    //     // swap(pair(a, b))
    //     let pattern = Fun(
    //         "foo".to_string(),
    //         vec![Sym("x".to_string()), Sym("x".to_string())],
    //     );

    //     // swap(pair(f(c), g(d)))
    //     let value = Fun(
    //         "foo".to_string(),
    //         vec![Sym("a".to_string()), Sym("a".to_string())],
    //     );

    //     println!("pattern: {}", pattern);
    //     println!("value: {}", value);
    //     if let Some(bindings) = pattern_match(&pattern, &value) {
    //         println!("match");
    //         for (key, value) in bindings.iter() {
    //             println!("{} => {}", key, value);
    //         }
    //     } else {
    //         println!("no match");
    //     }
}
