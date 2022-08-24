use combine::error::StringStreamError;
use combine::parser::char::{string};
use combine::{any, between, choice, eof, many, none_of, Parser, sep_by, token};
use crate::compiler::ast::{PrintStatement, Program, Statement, StringLiteral};
use crate::compiler::ast::Expression;

pub fn parse(source: &str) -> Result<Program, StringStreamError> {


    let string_item = choice!(
        none_of("\\\"".chars()),
        (token('\\'), any()).map(|(_, c)| c)
    );
    let string_literal_p = between(
        token('"'),
        token('"'),
        many::<Vec<char>, _, _>(string_item)
    ).map(|chars| StringLiteral { value: chars.iter().collect::<String>() });
    let expression_p = choice!(
        string_literal_p.map(|expr| Expression::StringLiteral(Box::new(expr)))
    );
    let arguments_p = sep_by(expression_p, token(','));

    let print_statement_p = between(
        string("print"),
        token(';'),
        arguments_p,
    ).map(|arguments|
        PrintStatement {
            arguments
        }
    );
    let statement_p = choice!(
        print_statement_p.map(|print_statememt| Statement::Print(Box::new(print_statememt)))
    );

    let program_p = many(statement_p).map(|statements| Program{statements});
    let mut parser = (program_p, eof()).map(|(program, _)| program);

    parser.parse(source).map(|(program, _)| program)
}

#[test]
fn test() {
    let program_r = parse(r#"print;print"abc";print"a\\b\"c";print"","aa";"#);
    print!("{:#?}", program_r);
    assert!(program_r.is_ok())
}
