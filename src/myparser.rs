use crate::types::errors::RispError;
use crate::types::expr::Expr;

pub struct Parser;

impl Parser {
    pub fn parse<'a>(tokens: &'a [String]) -> Result<(Expr, &'a [String]), RispError> {
        let (token, rest) = tokens
            .split_first()
            .ok_or(RispError::Reason("could not get token".to_string()))?;
        match &token[..] {
            "(" => Parser::read_seq(rest),
            ")" => Err(RispError::Reason("unexpected `)`".to_string())),
            _ => Ok((Parser::parse_atom(token), rest)),
        }
    }

    fn read_seq<'a>(tokens: &'a [String]) -> Result<(Expr, &'a [String]), RispError> {
        let mut res: Vec<Expr> = vec![];
        let mut xs = tokens;
        loop {
            let (next_token, rest) = xs.split_first().ok_or(RispError::UnmatchedParenthesis)?;
            if next_token == ")" {
                return Ok((Expr::List(res), rest));
            }
            let (exp, new_xs) = Parser::parse(xs)?;
            res.push(exp);
            xs = new_xs;
        }
    }

    fn parse_atom(token: &str) -> Expr {
        match token {
            "true" => Expr::Boolean(true),
            "false" => Expr::Boolean(false),
            _ => {
                if token.chars().all(char::is_numeric) {
                    Expr::Number(token.parse().unwrap())
                } else {
                    Expr::Symbol(token.to_string())
                }
            }
        }
    }

    pub fn tokenize(expr: String) -> Vec<String> {
        expr.replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::myparser::Parser; // 导入 Parser 结构体
    use crate::types::expr::Expr; // 导入 Parser 结构体 // 导入 Expr 枚举

    #[test]
    fn test_tokenize() {
        let input = String::from("(+ 1 (* 2 3))");
        let expected = vec!["(", "+", "1", "(", "*", "2", "3", ")", ")"];
        let result = Parser::tokenize(input);
        assert_eq!(result, expected);

        // Test with more complex input
        let input2 = String::from("(define x (lambda (y) (* y 2)))");
        let expected2 = vec![
            "(", "define", "x", "(", "lambda", "(", "y", ")", "(", "*", "y", "2", ")", ")", ")",
        ];
        let result2 = Parser::tokenize(input2);
        assert_eq!(result2, expected2);
    }
    #[test]
    fn test_parse() {
        let input = Parser::tokenize("(+ 1 (* 2 3))".to_string());
        let expected = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Number(2.0),
                Expr::Number(3.0),
            ]),
        ]);
        let (result, _) = Parser::parse(&input).unwrap();
        assert_eq!(result, expected);
    }
}
