use crate::parser::statement::Statement;
use crate::type_checker::type_errors::TypeError;

pub struct TypeChecker {
    statement: Statement,
}

impl TypeChecker {
    pub fn new(statement: Statement) -> Self {
        Self {
            statement,
        }
    }

    pub fn check(&mut self) -> Result<Statement, TypeError> {
        if let Statement::Program(global_statements) = self.statement.clone() {
            for statement in &global_statements {
                match statement {
                    Statement::Call(ref call_name, ref args) => {
                        match global_statements.iter().find_map(|d| match d {
                            Statement::FunctionDeclaration(funcName, args, _) => {
                                if call_name == funcName {
                                    return Some(args);
                                }
                                None
                            }
                            _ => None,
                        }) {
                            Some(function_params) => {
                                for (index, param) in function_params.iter().enumerate() {
                                    if let Some(arg) = args.get(index) {
                                        if let Statement::Primitive(argument) = arg {
                                            if let Statement::TypedParameter(_, param_type) = param {
                                                if !(argument.to_type() == param_type) {
                                                    let message = format!(
                                                        "Wrong type used in call {}, expected type {}, received type {}",
                                                        call_name, param_type,
                                                        argument.to_type()
                                                    );
                                                    return Err(TypeError::Message(message));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        return Ok(self.statement.clone());
    }
}