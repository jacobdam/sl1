#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Print(Box<PrintStatement>)
}

#[derive(Debug)]
pub struct PrintStatement {
    pub arguments: Vec<Expression>
}

#[derive(Debug)]
pub enum Expression {
    StringLiteral(Box<StringLiteral>),
    NumberLiteral(Box<NumberLiteral>)
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String
}

#[derive(Debug)]
pub struct NumberLiteral {
    pub value: i32
}
