use crate::parser::statement::Statement;
use crate::engine::js_value::JSValue;
use crate::lexer::tokens::arithmetic_operator::ArithmeticOperator;
use crate::engine::environment::environment_record::EnvironmentRecord;
use crate::engine::environment::environment_type::EnvironmentType;
use crate::parser::value::PrimitiveValue;
use crate::engine::environment::run_errors::RunError;
use crate::engine::environment::environment_record_binding::EnvironmentRecordBinding;
use crate::engine::expressions::function::Function;

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
        let mut global_scope = &mut EnvironmentRecord::new(EnvironmentType::Global);
        for statement in statements {

            let result = match self.run(statement, global_scope) {
                Ok(v) => println!("result {}", v),
                Err(e) => println!("result error {}", e)
            };
        }
    }

    fn run(&mut self, statement: Statement, scope: &mut EnvironmentRecord) -> RunResult {
        // for env in scope.records.clone() {
        //     match env.1.value.clone() {
        //         Some(t) => println!("name {}, binding {}", env.0, t),
        //         _ => println!("name {}", env.0)
        //     }
        // }
        // println!("statement {}", statement);
        match statement {
            Statement::FunctionDeclaration(ref name, ref args,ref body) => {
                scope.records.insert(name.clone(), EnvironmentRecordBinding { value: Option::from(JSValue::Function(Function::new(body.clone(), args.clone()))) });

                return Ok(JSValue::Undefined)
            },
            Statement::ConstDeclaration(ref name, ref statements) => {
                let last_const_statement = statements.clone().pop().ok_or(RunError::Message("Variable reference error"))?;
                let const_declaration = self.run(last_const_statement, scope)?;
                scope.records.insert(name.clone(), EnvironmentRecordBinding { value: Option::from(const_declaration) });
                Ok(JSValue::Undefined)
            },
            Statement::VariableRef(ref name) => {
                 let test = scope.records.get(name).and_then(|v| v.value.clone()).ok_or(RunError::Message("Variable reference error"))?;
                println!("VariableRef {}", test);
                Ok(test)
            },
            Statement::Call(ref name, ref args) => {
                //handle function call - how to send arguments to function declaration
                let call_binding =  scope.records.get(name).ok_or(RunError::Message("Call error - there is no function with this name"))?;
                let val = call_binding.value.clone();
                if let Some(JSValue::Function(val)) = val {
                    val.call(args.clone());
                    Ok(JSValue::Function(val))
                } else {
                    Err(RunError::Message("Call error"))
                }
            },
            Statement::Primitive(PrimitiveValue::Boolean(b)) => Ok(JSValue::Boolean(b)),
            Statement::Primitive(PrimitiveValue::String(s)) => Ok(JSValue::String(s)),
            Statement::Primitive(PrimitiveValue::Num(n)) => Ok(JSValue::Num(n)),
            Statement::ArithmeticOperation(op, first_val, second_val) => {
                let first_val = self.run(*first_val, scope).or(Err(RunError::Message("Wrong first arithmetic value")))?;
                let second_val = self.run(*second_val, scope).or(Err(RunError::Message("Wrong second arithmetic value")))?;

                let calculated_val = self.run_arithmetic_operation(op, first_val, second_val);
                Ok(calculated_val)
            },
            v => {
                println!("not found {}",v);
                Err(RunError::Message("Not found statement to run"))
            }
        }
    }

    fn run_arithmetic_operation(&self, op: ArithmeticOperator, first_val: JSValue, second_val: JSValue) -> JSValue {
        return match op {
            ArithmeticOperator::PLUS => first_val + second_val,
            ArithmeticOperator::MINUS => first_val - second_val,
            ArithmeticOperator::MULTIPLICATION => first_val * second_val,
            ArithmeticOperator::DIVISION => first_val / second_val
        }
    }
}