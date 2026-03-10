use crate::types::*;

pub fn parse_value(tok: &str) -> Expr {
    if let Ok(n) = tok.parse::<f64>() {
        Expr::Number(n)
    } else {
        Expr::Var(tok.to_string())
    }
}

pub fn parse(tokens: Vec<String>) -> Stmt {
    match tokens[1].as_str() {
        "say" => {
            let msg = tokens[2].trim_matches('"').to_string();

            if tokens.len() == 3 {
                return Stmt::PrintText(msg);
            }

            if tokens[3] == "and" && tokens[4] == "read" {
                let var = tokens[6].clone();
                return Stmt::InputNumber(msg, var);
            }

            if tokens[3] == "and" && tokens[5] == "aloud" {
                let var = tokens[4].clone();
                return Stmt::PrintValue(msg, Expr::Var(var));
            }

            Stmt::PrintText(msg)
        }

        "remember" => {
            let name = tokens[3].clone();

            if tokens[4] == "is" {
                let val = tokens[5].parse::<f64>().unwrap();
                return Stmt::Let(name, Expr::Number(val));
            }

            let a = parse_value(&tokens[6]);

            let op = match tokens[7].as_str() {
                "plus" => Op::Add,
                "minus" => Op::Sub,
                "times" => Op::Mul,
                "by" => Op::Div,
                _ => panic!("unknown operator"),
            };

            let b = parse_value(&tokens[8]);

            Stmt::Let(name, Expr::Binary(Box::new(a), op, Box::new(b)))
        }

        _ => panic!("unknown statement"),
    }
}
