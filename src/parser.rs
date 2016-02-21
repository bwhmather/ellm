use std::str;
use std::collections::HashMap;

use combine;
use combine::{Parser, ParserExt};

use ast;

fn name<I>(input: combine::State<I>) -> combine::ParseResult<String, I>
where I: combine::primitives::Stream<Item=char> {
    combine::many1(combine::letter()).parse_state(input)
}


fn literal<I>(input: combine::State<I>) -> combine::ParseResult<ast::Expression, I>
where I: combine::primitives::Stream<Item=char> {
    combine::many1(combine::digit()).map(
        |string: String| ast::Expression::Literal(string.parse::<i64>().unwrap())
    ).parse_state(input)
}


fn variable<I>(input: combine::State<I>) -> combine::ParseResult<ast::Expression, I>
where I: combine::primitives::Stream<Item=char> {
    combine::many1(combine::letter()).map(
        |string: String| ast::Expression::Variable(string)
    ).parse_state(input)
}


fn expression<I>(input: combine::State<I>) -> combine::ParseResult<ast::Expression, I>
where I: combine::primitives::Stream<Item=char> {
    combine::parser(literal).or(combine::parser(variable)).parse_state(input)
}


fn prototype<I>(input: combine::State<I>) -> combine::ParseResult<ast::Prototype, I>
where I: combine::primitives::Stream<Item=char> {
    let arg = combine::many1::<Vec<_>, _>(combine::token(' ')).with(combine::parser(name));

    (
        combine::parser(name),
        combine::many(arg),
    ).map(|(name, args) : (String, Vec<String>)| {
        ast::Prototype{ name: name, args: args }
    }).parse_state(input)
}


fn function<I>(input: combine::State<I>) -> combine::ParseResult<ast::Function, I>
where I: combine::primitives::Stream<Item=char> {
    (
        combine::parser(prototype),
//        combine::spaces().with(combine::token('=')).with(combine::spaces()),
//        combine::token(' ').with(combine::token('=')).with(combine::token(' ')),
        combine::spaces().with(combine::token('=')).with(combine::spaces()),
        combine::parser(expression)
    ).map(|(prototype, _, body)| {
        ast::Function{ prototype: prototype, body: body }
    }).parse_state(input)
}


fn statement<I>(input: combine::State<I>) -> combine::ParseResult<ast::Statement, I>
where I: combine::primitives::Stream<Item=char> {
    let function_statement = combine::parser(function).map(
        |fun| ast::Statement::Definition(fun)
    );

    let declaration_statement = combine::parser(prototype).map(
        |prototype| ast::Statement::Declaration(prototype)
    );

    function_statement.or(declaration_statement).parse_state(input)
}


fn module<I>(input: combine::State<I>) -> combine::ParseResult<ast::Module, I>
where I: combine::primitives::Stream<Item=char> {
    combine::sep_by(combine::parser(statement), combine::spaces()).map(
        |statements| ast::Module{ statements: statements }
    ).parse_state(input)
}


pub fn parse(source: String) -> ast::Module {
    ast::Module{ statements: Vec::new() }
}



fn assignment<I>(input: combine::State<I>) -> combine::ParseResult<(), I>
where I: combine::primitives::Stream<Item=char> {
    combine::spaces().with(combine::token('=')).with(combine::spaces())
        .map(
        |_| {()}
    ).parse_state(input)
}

#[test]
fn test_temp() {
    assert_eq!(
        combine::parser(assignment).parse("= "),
        Ok(((), ""))
    )
}



#[test]
fn parse_literal_test() {
    assert_eq!(
        combine::parser(literal).parse("1234"),
        Ok((ast::Expression::Literal(1234), ""))
    );
}

#[test]
fn parse_variable_test() {
    assert_eq!(
        combine::parser(variable).parse("foo"),
        Ok((ast::Expression::Variable("foo".to_string()), ""))
    );
}

#[test]
fn parse_expression_test() {
    assert_eq!(
        combine::parser(expression).parse("1234"),
        Ok((ast::Expression::Literal(1234), ""))
    );

    assert_eq!(
        combine::parser(expression).parse("foo"),
        Ok((ast::Expression::Variable("foo".to_string()), ""))
    );
}

#[test]
fn parse_prototype_test() {
    assert_eq!(
        combine::parser(prototype).parse("value"),
        Ok((ast::Prototype{
            name: "value".to_string(),
            args: vec![],
        }, ""))
    );

    assert_eq!(
        combine::parser(prototype).parse("fizzbuzz n"),
        Ok((ast::Prototype{
            name: "fizzbuzz".to_string(),
            args: vec!["n".to_string()],
        }, ""))
    );
}

#[test]
fn parse_function_test() {
    assert_eq!(
        combine::parser(function).parse("var = 1"),
        Ok((ast::Function{
            prototype: ast::Prototype{
                name: "var".to_string(),
                args: vec![],
            },
            body: ast::Expression::Literal(1),
        }, ""))
    );
}

#[test]
fn parse_statement_test() {

}

#[test]
fn parse_module_test() {
    assert_eq!(
        combine::parser(module).parse("var = 1"),
        Ok((ast::Module{
            statements: vec![
                ast::Statement::Definition(ast::Function{
                    prototype: ast::Prototype{
                        name: "var".to_string(),
                        args: vec![],
                    },
                    body: ast::Expression::Literal(1),
                })
            ]
        }, ""))
    );
    combine::parser(module).parse("\nvar = 1\n\nignore x = 0");
}

