use crate::parser::statement::Statement;
use crate::engine::ts_value::TSValue;
use crate::lexer::token_kinds::arithmetic_operator::ArithmeticOperator;
use crate::engine::environment::environment_record::EnvironmentRecord;
use crate::engine::environment::environment_type::EnvironmentType;
use crate::parser::value::PrimitiveValue;
use crate::engine::environment::run_errors::RunError;
use crate::engine::environment::environment_record_binding::EnvironmentRecordBinding;
use crate::engine::expressions::function::Function;
use wasm_bindgen::prelude::*;

type RunResult = Result<TSValue, RunError>;

#[wasm_bindgen]
extern "C" {
    type PubSub;

    #[wasm_bindgen(js_namespace = ["window","PubSub"])]
    fn emit(func: &str);
}

pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&mut self, statement: Statement) {
        let global_scope = &mut EnvironmentRecord::new(EnvironmentType::Global);
        if let Statement::Program(global_statements) = statement {
            for statement in global_statements {
                match self.run(statement, global_scope) {
                    Ok(val) => {
                        if val != TSValue::Undefined {
                            PubSub::emit(val.to_string().as_str());
                            println!("{}",val)
                        }
                    }
                    Err(e) => {
                        let mut msg = String::new();
                        msg.push_str("Error: ");
                        msg.push_str(e.to_string().as_str());
                        PubSub::emit( msg.as_str());
                    }
                }
            }
        }
    }

    fn run(&mut self, statement: Statement, scope: &mut EnvironmentRecord) -> RunResult {
        match statement {
            Statement::FunctionDeclaration(ref name, ref args, ref body) => {
                let ts_function = TSValue::Function(
                    Function::new(body.clone(), args.clone())
                );
                let binding = EnvironmentRecordBinding {
                    value: ts_function
                };
                scope.records.insert(name.clone(),binding );
                return Ok(TSValue::Undefined);
            },
            Statement::Primitive(PrimitiveValue::Boolean(b)) => {
                Ok(TSValue::Boolean(b))
            },
            Statement::Primitive(PrimitiveValue::String(s)) => {
                Ok(TSValue::String(s))
            },
            Statement::ConstDeclaration(ref name, ref statements) => {
                let last_const_statement = statements
                    .clone()
                    .pop()
                    .ok_or(RunError::Message("Variable reference error"))?;
                let const_declaration = self.run(last_const_statement, scope)?;
                let binding = EnvironmentRecordBinding {
                    value: const_declaration
                };
                scope.records.insert(name.clone(), binding);
                Ok(TSValue::Undefined)
            },
            Statement::Call(ref name, ref args) => {
                let call_binding = scope.records.get(name).ok_or(RunError::Message("Call error - there is no function with this name"))?;

                let val = call_binding.value.clone();
                if let TSValue::Function(val) = val {
                    let func_expression = *val.clone().expression;
                    let func_args = val.clone().args;
                    if let Statement::Block(func_expression) = func_expression {
                        let func_scope = &mut EnvironmentRecord::new(EnvironmentType::Function);

                        for (count, arg) in func_args.iter().enumerate() {
                            if let Statement::TypedParameter(name, _) = arg {
                                let passed_arg_value = match args.get(count) {
                                    Some(arg_statement) => self.run(arg_statement.clone(), scope)?,
                                    None => return Err(RunError::Message("Call error"))
                                };
                                func_scope.records.insert(name.clone(), EnvironmentRecordBinding { value: passed_arg_value });
                            }
                        }

                        for statement in func_expression {
                            let function_value = self.run(statement.clone(), func_scope)?;
                            if let Statement::Return(_) = statement {
                                return Ok(function_value);
                            }
                        }
                    }

                    Ok(TSValue::Undefined)
                } else {
                    Err(RunError::Message("Call error"))
                }
            }
            Statement::VariableRef(ref name) => {
                let test = scope.records.get(name).and_then(|v| Option::from(v.value.clone())).ok_or(RunError::Message("Variable reference error"))?;
                Ok(test)
            }

            Statement::Primitive(PrimitiveValue::Num(n)) => Ok(TSValue::Num(n)),
            Statement::ArithmeticOperation(op, first_val, second_val) => {
                let first_val = self.run(*first_val, scope).or(Err(RunError::Message("Wrong first arithmetic value")))?;
                let second_val = self.run(*second_val, scope).or(Err(RunError::Message("Wrong second arithmetic value")))?;

                let calculated_val = self.run_arithmetic_operation(op, first_val, second_val);
                Ok(calculated_val)
            }
            Statement::Return(v) => {
                self.run(*v, scope)
            }
            v => {
                println!("not found {}", v);
                Err(RunError::Message("Not found statement to run"))
            }
        }
    }

    fn run_arithmetic_operation(&self, op: ArithmeticOperator, first_val: TSValue, second_val: TSValue) -> TSValue {
        return match op {
            ArithmeticOperator::PLUS => first_val + second_val,
            ArithmeticOperator::MINUS => first_val - second_val,
            ArithmeticOperator::MULTIPLICATION => first_val * second_val,
            ArithmeticOperator::DIVISION => first_val / second_val
        };
    }
}