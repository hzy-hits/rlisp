pub mod errors {

    #[derive(Debug)]
    pub enum RispError {
        Reason(String),
        Syntax(String),
        Parse(String),
        UnmatchedParenthesis,
        UnexpectedToken(String),
        UnclosedStringLiteral,
    }
}

pub mod expr {
    use super::errors::RispError;
    use std::fmt;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Expr {
        Number(f64),
        Symbol(String),
        List(Vec<Expr>),
        Boolean(bool),
        Func(fn(&[Expr]) -> Result<Expr, RispError>),
    }

    impl std::fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let str = match self {
                Expr::Symbol(s) => s.clone(),
                Expr::Boolean(b) => b.to_string(),
                Expr::Number(n) => n.to_string(),
                Expr::List(list) => {
                    let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                    format!("({})", xs.join(", "))
                }
                Expr::Func(_) => "function {}".to_string(),
            };
            write!(f, "{}", str)
        }
    }
}

pub mod environment {
    use crate::types::errors::RispError;
    use crate::types::expr::Expr;
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct RispEnv {
        pub data: HashMap<String, Expr>,
    }

    pub fn default_env() -> RispEnv {
        let mut data = HashMap::new();
        data.insert(
            "+".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, RispError> {
                let sum = prase_list_of_numbers(args)?.iter().sum();
                Ok(Expr::Number(sum))
            }),
        );
        data.insert(
            "-".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, RispError> {
                let nums = prase_list_of_numbers(args)?;
                let first = *nums.first().ok_or(RispError::Reason(
                    "expected at least one number".to_string(),
                ))?;
                let sum_of_the_rest: f64 = nums[1..].iter().sum();
                Ok(Expr::Number(first - sum_of_the_rest))
            }),
        );
        data.insert(
            "*".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, RispError> {
                let product = prase_list_of_numbers(args)?.iter().product();
                Ok(Expr::Number(product))
            }),
        );
        data.insert(
            "/".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, RispError> {
                let nums = prase_list_of_numbers(args)?;
                let first = *nums.first().ok_or(RispError::Reason(
                    "expected at least one number".to_string(),
                ))?;
                let product_of_the_rest: f64 = nums[1..].iter().product();
                Ok(Expr::Number(first / product_of_the_rest))
            }),
        );
        RispEnv { data }
    }

    fn prase_list_of_numbers(args: &[Expr]) -> Result<Vec<f64>, RispError> {
        args.iter().map(|x| parse_single_number(x)).collect()
    }

    fn parse_single_number(expr: &Expr) -> Result<f64, RispError> {
        match expr {
            Expr::Number(num) => Ok(*num),
            _ => Err(RispError::Reason("expected a number".to_string())),
        }
    }
}
