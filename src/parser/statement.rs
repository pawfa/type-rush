pub enum Statement {
    FunctionDeclaration(String, Vec<Statement>, Box<Statement>),
    ExpressionStatement,
    TypedArgument(String, Box<Statement>),
    Interface
}