#[derive(PartialEq, Clone, Debug)]
pub enum Pattern<'a> {
    NamedSubPattern {
        name: &'a str,
        subpattern: &'a Pattern<'a>,
    },
    RecordPattern(Vec<(&'a str, &'a Pattern<'a>)>),
    TuplePattern(Vec<&'a Pattern<'a>>),
    NamePattern(&'a str),
    EnumPattern {
        constructor: &'a str, 
        arguments: Vec<&'a Pattern<'a>>,
    },
}


#[derive(PartialEq, Clone, Debug)]
pub struct LetBinding<'a> {
    name: &'a str,
    arguments: Vec<&'a Pattern<'a>>,
    body: &'a Expression<'a>,
}


#[derive(PartialEq, Clone, Debug)]
pub struct CaseAlternative<'a> {
    pattern: &'a Pattern<'a>,
    body: &'a Expression<'a>,
}


#[derive(PartialEq, Clone, Debug)]
pub enum Expression<'a> {
    Literal(i64),
    Variable(&'a str),
    Call(Vec<&'a Expression<'a>>),
    Tuple(Vec<&'a Expression<'a>>),
    List(Vec<&'a Expression<'a>>),
    Let {
        bindings: Vec<&'a LetBinding<'a>>,
        body: &'a Expression<'a>,
    },
    Case(Vec<&'a CaseAlternative<'a>>),
}


#[derive(PartialEq, Clone, Debug)]
pub struct Prototype<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}


#[derive(PartialEq, Clone, Debug)]
pub struct Function<'a> {
    pub prototype: &'a Prototype<'a>,
    pub body: &'a Expression<'a>,
}


#[derive(PartialEq, Clone, Debug)]
pub enum Statement<'a> {
    Declaration(&'a Prototype<'a>),
    Definition(&'a Function<'a>),
}


#[derive(PartialEq, Clone, Debug)]
pub struct Module<'a> {
    pub statements: Vec<&'a Statement<'a>>,
}
