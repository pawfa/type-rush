use crate::parser::statement::Statement;
use std::collections::{HashMap, VecDeque};
use crate::engine::environment::environment_record_binding::EnvironmentRecordBinding;
use crate::engine::js_value::JSValue;
use crate::engine::expressions::function::Function;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;

pub struct Runner {
    records: HashMap<String, EnvironmentRecordBinding>,
}

impl Runner {

    pub fn new() -> Self {
        let mut runner = Runner {
            records: HashMap::new(),
        };
        runner
    }

    pub fn start(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.run(statement)
        }
    }

    fn run(&mut self, statement: Statement) {
        println!("statement {}", statement);
        match statement {
            Statement::FunctionDeclaration(ref name, ref args, ref func_statement) => {
                self.records.insert(name.clone(), EnvironmentRecordBinding {value: Option::from(JSValue::Function(Function {expression: func_statement.clone(), args: args.clone()})) });
                println!("{}", "test");
            },
            Statement::ConstDeclaration(ref name, ref statement) => {

            },
            Statement::VariableRef(_) => {},
            Statement::Value(_) => {},
            Statement::Call(ref name, ref args) => {
                match self.records.get(name).unwrap().value.clone() {
                    Some(JSValue::Function(v)) => v.call(args.clone()),
                    _ => println!("{} err", "not a function")
                };

                println!("{}", name);
                println!("{}", args[0]);
            },
            Statement::Return(_) => {},
            Statement::Block(_) => {

            },
            Statement::ArithmeticOperation(op, first_val, second_val) => {
                println!("arithmetic operation {}", first_val);
                let first_val = match *first_val {
                    Statement::VariableRef(var) => match self.records.get(&var).unwrap().value.clone() {
                        Some(v) => v,
                        _=> JSValue::Undefined
                    },
                    _ => JSValue::Undefined
                };
                let second_val = match *second_val {
                    Statement::VariableRef(var) =>  match self.records.get(&var).unwrap().value.clone() {
                        Some(v) => v,
                        _=> JSValue::Undefined
                    },
                    _ => JSValue::Undefined
                };
                let calculated_val = self.run_arithmetic_operation(op, first_val, second_val);
            },
            Statement::TypedArgument(_, _) => {},
        }
    }

    fn get_value(&self, statement: Statement) {

    }

    fn run_arithmetic_operation(&self, op: ArithmeticOperator, first_val: JSValue, second_val: JSValue) -> JSValue {
        println!("arithmetic {}", first_val);
        return match op {
            ArithmeticOperator::PLUS => first_val + second_val,
            ArithmeticOperator::MINUS => first_val - second_val,
            ArithmeticOperator::MULTIPLICATION => first_val * second_val,
            ArithmeticOperator::DIVISION => first_val / second_val
        }
    }
}