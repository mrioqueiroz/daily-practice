use std::{collections::HashMap, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, Clone)]
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

impl Rule {
    #[allow(dead_code)]
    fn apply_all(&self, expr: Expr) -> Expr {
        todo!()
    }
}

type Bindings = HashMap<String, Expr>;

fn pattern_match(pattern: &Expr, value: &Expr) -> Option<Bindings> {
    fn pattern_match_impl(pattern: &Expr, value: &Expr, bindings: &mut Bindings) -> bool {
        use Expr::*;
        match (pattern, value) {
            (Sym(name), _) => {
                bindings.insert(name.clone(), value.clone());
                true
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
            (Fun(_, _), Sym(_)) => todo!(),
        }
    }

    let mut bindings = HashMap::new();
    if pattern_match_impl(&pattern, &value, &mut bindings) {
        Some(bindings)
    } else {
        None
    }
}

fn main() {
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

    // swap(pair(a, b))
    let pattern = swap.head;

    // swap(pair(f(c), g(d)))
    let value = Fun(
        "swap".to_string(),
        vec![Fun(
            "pair".to_string(),
            vec![
                Fun("f".to_string(), vec![Sym("c".to_string())]),
                Fun("g".to_string(), vec![Sym("d".to_string())]),
            ],
        )],
    );

    println!("Pattern: {}", pattern);
    println!("Value: {}", value);
    println!("{:?}", pattern_match(&pattern, &value));
}
