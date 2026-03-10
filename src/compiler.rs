use crate::types::*;

pub fn compile_expr(expr: &Expr, code: &mut Vec<Instr>) {
    match expr {
        Expr::Number(n) => code.push(Instr::Push(*n)),
        Expr::Var(v) => code.push(Instr::Load(v.clone())),

        Expr::Binary(a, op, b) => {
            compile_expr(a, code);
            compile_expr(b, code);

            match op {
                Op::Add => code.push(Instr::Add),
                Op::Sub => code.push(Instr::Sub),
                Op::Mul => code.push(Instr::Mul),
                Op::Div => code.push(Instr::Div),
            }
        }
    }
}

pub fn compile_stmt(stmt: Stmt, code: &mut Vec<Instr>) {
    match stmt {
        Stmt::Let(name, expr) => {
            compile_expr(&expr, code);
            code.push(Instr::Store(name));
        }

        Stmt::PrintText(msg) => code.push(Instr::PrintText(msg)),

        Stmt::PrintValue(msg, Expr::Var(v)) => {
            code.push(Instr::Load(v));
            code.push(Instr::PrintValue(msg));
        }

        Stmt::InputNumber(prompt, var) => {
            code.push(Instr::InputNumber(prompt, var));
        }

        _ => {}
    }
}
