#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Literal(i64),
    Variable(String),
    Call(String, Vec<Expression>)
}


#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
}


#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expression,
}


#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Declaration(Prototype),
    Definition(Function),
}


#[derive(PartialEq, Clone, Debug)]
pub struct Module {
    pub statements: Vec<Statement>,
}
