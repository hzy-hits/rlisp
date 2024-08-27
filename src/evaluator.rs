
use crate::types::environment::RispEnv;
use crate::types::errors::RispError;
use crate::types::expr::Expr;

pub fn eval(exp: &Expr, env: &mut RispEnv) -> Result<Expr, RispError> {
    match exp {
        Expr::Number(_) | Expr::Boolean(_) => Ok(exp.clone()),
        Expr::Symbol(k) => env
            .data
            .get(k)
            .ok_or(RispError::Reason(format!(
                "unexpected symbol: {}",
                k.to_string()
            )))
            .map(|x| x.clone()),
        Expr::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispError::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                Expr::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<Expr>, RispError>>();
                    f(&args_eval?)
                }
                _ => Err(RispError::Reason(
                    "first form must be a function".to_string(),
                )),
            }
        }
        _ => Err(RispError::Reason("unexpected expression".to_string())),
    }
}
