use super::ast::*;
use crate::compiler::parser::parse;

pub fn evaluate_program(program: &Program) {
    for statement in &program.statements {
        evaluate_statement(&statement);
    }
}

fn evaluate_statement(statement: &Statement) {
    match statement {
        Statement::Print(print_statement) => evaluate_print_statement(print_statement),
    }
}

fn evaluate_print_statement(print_statement: &PrintStatement) {
    for argument in &print_statement.arguments {
        let value = evaluate_expression(&argument);
        match value {
            ExpressionValue::String(s) => print!("{}", s),
            ExpressionValue::Number(n) => print!("{}", n),
        }
    }
    println!()
}

enum ExpressionValue {
    String(&str),
    Number(i32),
}

fn evaluate_expression(expression: &Expression) -> ExpressionValue {
    match expression {
        Expression::StringLiteral(str) => ExpressionValue::String(),
        Expression::NumberLiteral(num) => ExpressionValue::Number(num.value),
    }
}

#[test]
fn test() {
    let source = include_str!("test.sl1");
    let program = parse(source).expect("valid syntax");

    evaluate_program(&program);
}
