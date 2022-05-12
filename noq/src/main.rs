use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
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
    println!("{}", swap);
}
