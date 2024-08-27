use risp::{default_env, eval, expr::Expr, Parser, RispEnv, RispError};

fn parse_eval(expr: String, env: &mut RispEnv) -> Result<Expr, RispError> {
    let tokens = Parser::tokenize(expr);
    let (parsed, _) = Parser::parse(&tokens)?;
    eval(&parsed, env)
}

fn slurp_expr() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    println!("Welcome to the Rust Lisp Interpreter!");
    println!("Please enter an expression to evaluate:");
    let mut env = default_env();
    loop {
        let input = slurp_expr();
        match parse_eval(input, &mut env) {
            Ok(res) => println!("// 🔥 => {}", res),
            Err(err) => match err {
                RispError::Reason(msg) => println!("Error: {}", msg),
                _ => println!("Error"),
            },
        }
    }
}
