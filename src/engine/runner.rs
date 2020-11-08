use crate::parser::statement::Statement;
use crate::engine::js_value::JSValue;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;
use crate::engine::environment::environment_record::EnvironmentRecord;
use crate::engine::environment::environment_type::EnvironmentType;
use crate::parser::value::PrimitiveValue;
use crate::engine::environment::run_errors::RunError;

type RunResult = Result<JSValue, RunError>;
pub struct Runner {
    global: EnvironmentRecord
}

impl Runner {

    pub fn new() -> Self {
        let mut runner = Runner {
            global: EnvironmentRecord::new(EnvironmentType::Global),
        };
        runner
    }

    pub fn start(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            println!("start statement {}", statement);
            let result = match self.run(statement) {
                Ok(v) => println!("result {}", v),
                Err(e) => println!("result error {}", e)
            };
        }
    }

    fn run(&mut self, statement: Statement) -> RunResult {
        println!("current run statement {}", statement);
        match statement {
            Statement::ConstDeclaration(ref name, ref statement) => {
                let const_declaration = self.run(*statement.clone());
                match const_declaration {
                    Ok(ref c) => println!("const declaration {} ", name),
                    Err(ref e) => println!("const declaration error {} ", e)
                }

                return const_declaration
            },
            Statement::Primitive(PrimitiveValue::Boolean(b)) => Ok(JSValue::Boolean(b)),
            Statement::Primitive(PrimitiveValue::String(s)) => Ok(JSValue::String(s)),
            Statement::Primitive(PrimitiveValue::Num(n)) => Ok(JSValue::Num(n)),
            Statement::ArithmeticOperation(op, first_val, second_val) => {
                let first_val = match self.run(*first_val) {
                    Ok(v) => v,
                    _ => return Err(RunError::Message("Arithmetic operation value error"))
                };
                let second_val = match self.run(*second_val) {
                    Ok(v) => v,
                    _ => return Err(RunError::Message("Arithmetic operation value error"))
                };
                let calculated_val = self.run_arithmetic_operation(op, first_val, second_val);
                Ok(calculated_val)
            },
            _ => {
                Err(RunError::Message("Not found statement to run"))
            }
        }
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