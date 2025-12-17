use crate::ast::{Program, Statement, Expression, Operator};
use crate::value::{Value, SexagesimalNum, parse_number};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Undefined variable: '{0}'")]
    UndefinedVariable(String),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid operator for types: {0}")]
    InvalidOperator(String),
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }
    
    pub fn eval_program(
        &self, 
        program: &Program, 
        environment: &mut Environment
    ) -> Result<Option<Value>, RuntimeError> {
        let mut result = None;
        
        for statement in &program.statements {
            result = Some(self.eval_statement(statement, environment)?);
        }
        
        Ok(result)
    }
    
    fn eval_statement(
        &self, 
        statement: &Statement, 
        environment: &mut Environment
    ) -> Result<Value, RuntimeError> {
        match statement {
            Statement::Expression(expr) => self.eval_expression(expr, environment),
            Statement::Assignment(assign) => {
                let value = self.eval_expression(&assign.value, environment)?;
                environment.set(assign.variable.clone(), value.clone());
                Ok(value)
            }
        }
    }
    
    fn eval_expression(
        &self, 
        expr: &Expression, 
        environment: &mut Environment
    ) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Number(n_str) => {
                parse_number(n_str)
                    .map_err(|e| RuntimeError::TypeError(e.to_string()))
            }
            Expression::Identifier(id) => {
                environment.get(id)
                    .ok_or_else(|| RuntimeError::UndefinedVariable(id.clone()))
            }
            Expression::Binary(op, left, right) => {
                let left_val = self.eval_expression(left, environment)?;
                let right_val = self.eval_expression(right, environment)?;
                self.eval_binary_operation(op, &left_val, &right_val)
            }
            Expression::Unary(op, expr) => {
                let value = self.eval_expression(expr, environment)?;
                self.eval_unary_operation(op, &value)
            }
            Expression::Grouped(expr) => {
                self.eval_expression(expr, environment)
            }
        }
    }
    
    fn eval_binary_operation(
        &self, 
        op: &Operator, 
        left: &Value, 
        right: &Value
    ) -> Result<Value, RuntimeError> {
        match op {
            Operator::Plus => self.add_values(left, right),
            Operator::Minus => self.subtract_values(left, right),
            Operator::Multiply => self.multiply_values(left, right),
            Operator::Divide => self.divide_values(left, right),
        }
    }
    
    fn eval_unary_operation(
        &self, 
        op: &Operator, 
        value: &Value
    ) -> Result<Value, RuntimeError> {
        match op {
            Operator::Plus => Ok(value.clone()), // +value
            Operator::Minus => self.negate_value(value),
        }
    }
    
    fn add_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            
            // Sexagesimal operations
            (Value::Sexagesimal(a), Value::Sexagesimal(b)) => {
                let result_float = a.to_f64() + b.to_f64();
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Sexagesimal(a), Value::Integer(b)) => {
                let result_float = a.to_f64() + *b as f64;
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Integer(a), Value::Sexagesimal(b)) => {
                let result_float = *a as f64 + b.to_f64();
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Sexagesimal(a), Value::Float(b)) => {
                let result_float = a.to_f64() + b;
                Ok(Value::Float(result_float))
            }
            (Value::Float(a), Value::Sexagesimal(b)) => {
                let result_float = a + b.to_f64();
                Ok(Value::Float(result_float))
            }
            
            _ => Err(RuntimeError::InvalidOperator(
                format!("Cannot add {} and {}", left, right)
            )),
        }
    }
    
    fn subtract_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            
            // Sexagesimal operations
            (Value::Sexagesimal(a), Value::Sexagesimal(b)) => {
                let result_float = a.to_f64() - b.to_f64();
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Sexagesimal(a), Value::Integer(b)) => {
                let result_float = a.to_f64() - *b as f64;
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Integer(a), Value::Sexagesimal(b)) => {
                let result_float = *a as f64 - b.to_f64();
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Sexagesimal(a), Value::Float(b)) => {
                let result_float = a.to_f64() - b;
                Ok(Value::Float(result_float))
            }
            (Value::Float(a), Value::Sexagesimal(b)) => {
                let result_float = a - b.to_f64();
                Ok(Value::Float(result_float))
            }
            
            _ => Err(RuntimeError::InvalidOperator(
                format!("Cannot subtract {} from {}", right, left)
            )),
        }
    }
    
    fn multiply_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            
            // Sexagesimal operations
            (Value::Sexagesimal(a), Value::Integer(b)) => {
                let result_float = a.to_f64() * *b as f64;
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Integer(a), Value::Sexagesimal(b)) => {
                let result_float = *a as f64 * b.to_f64();
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Sexagesimal(a), Value::Float(b)) => {
                let result_float = a.to_f64() * b;
                Ok(Value::Float(result_float))
            }
            (Value::Float(a), Value::Sexagesimal(b)) => {
                let result_float = a * b.to_f64();
                Ok(Value::Float(result_float))
            }
            (Value::Sexagesimal(a), Value::Sexagesimal(b)) => {
                let result_float = a.to_f64() * b.to_f64();
                Ok(Value::Float(result_float)) // Multiplication of sexagesimals gives float
            }
            
            _ => Err(RuntimeError::InvalidOperator(
                format!("Cannot multiply {} and {}", left, right)
            )),
        }
    }
    
    fn divide_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        // Check for division by zero
        match right {
            Value::Integer(0) => return Err(RuntimeError::DivisionByZero),
            Value::Float(n) if *n == 0.0 => return Err(RuntimeError::DivisionByZero),
            Value::Sexagesimal(sex) if sex.to_f64() == 0.0 => return Err(RuntimeError::DivisionByZero),
            _ => {}
        }
        
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if a % b == 0 {
                    Ok(Value::Integer(a / b))
                } else {
                    Ok(Value::Float(*a as f64 / *b as f64))
                }
            }
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a / *b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            
            // Sexagesimal operations
            (Value::Sexagesimal(a), Value::Integer(b)) => {
                let result_float = a.to_f64() / *b as f64;
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Integer(a), Value::Sexagesimal(b)) => {
                let result_float = *a as f64 / b.to_f64();
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
            (Value::Sexagesimal(a), Value::Float(b)) => {
                let result_float = a.to_f64() / b;
                Ok(Value::Float(result_float))
            }
            (Value::Float(a), Value::Sexagesimal(b)) => {
                let result_float = a / b.to_f64();
                Ok(Value::Float(result_float))
            }
            (Value::Sexagesimal(a), Value::Sexagesimal(b)) => {
                let result_float = a.to_f64() / b.to_f64();
                Ok(Value::Float(result_float)) // Division of sexagesimals gives float
            }
            
            _ => Err(RuntimeError::InvalidOperator(
                format!("Cannot divide {} by {}", left, right)
            )),
        }
    }
    
    fn negate_value(&self, value: &Value) -> Result<Value, RuntimeError> {
        match value {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Float(n) => Ok(Value::Float(-n)),
            Value::Sexagesimal(sex) => {
                let result_float = -sex.to_f64();
                Ok(Value::Sexagesimal(SexagesimalNum::from_f64(result_float)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Program, Statement, Expression, Assignment};

    #[test]
    fn test_eval_integer_arithmetic() {
        let mut env = Environment::new();
        let interpreter = Interpreter::new();
        
        // Test 1 + 2
        let expr = Expression::Binary(
            Operator::Plus,
            Box::new(Expression::Number("1".to_string())),
            Box::new(Expression::Number("2".to_string())),
        );
        let result = interpreter.eval_expression(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Integer(3));
        
        // Test 5 * 3
        let expr = Expression::Binary(
            Operator::Multiply,
            Box::new(Expression::Number("5".to_string())),
            Box::new(Expression::Number("3".to_string())),
        );
        let result = interpreter.eval_expression(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Integer(15));
    }
    
    #[test]
    fn test_eval_float_arithmetic() {
        let mut env = Environment::new();
        let interpreter = Interpreter::new();
        
        // Test 1.5 + 2.5
        let expr = Expression::Binary(
            Operator::Plus,
            Box::new(Expression::Number("1.5".to_string())),
            Box::new(Expression::Number("2.5".to_string())),
        );
        let result = interpreter.eval_expression(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Float(4.0));
        
        // Test 5.0 / 2.0
        let expr = Expression::Binary(
            Operator::Divide,
            Box::new(Expression::Number("5.0".to_string())),
            Box::new(Expression::Number("2.0".to_string())),
        );
        let result = interpreter.eval_expression(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Float(2.5));
    }
    
    #[test]
    fn test_eval_sexagesimal_arithmetic() {
        let mut env = Environment::new();
        let interpreter = Interpreter::new();
        
        // Test 1;30 + 0;30 = 2;0
        let expr = Expression::Binary(
            Operator::Plus,
            Box::new(Expression::Number("1;30".to_string())),
            Box::new(Expression::Number("0;30".to_string())),
        );
        let result = interpreter.eval_expression(&expr, &mut env).unwrap();
        if let Value::Sexagesimal(sex) = result {
            assert_eq!(sex.integer_part, 2);
            assert_eq!(sex.fractional_part, 0);
        } else {
            panic!("Expected Sexagesimal result");
        }
    }
    
    #[test]
    fn test_eval_assignment() {
        let mut env = Environment::new();
        let interpreter = Interpreter::new();
        
        // Test x = 42
        let assign = Assignment {
            variable: "x".to_string(),
            value: Expression::Number("42".to_string()),
        };
        let stmt = Statement::Assignment(assign);
        
        let result = interpreter.eval_statement(&stmt, &mut env).unwrap();
        assert_eq!(result, Value::Integer(42));
        
        // Verify variable is stored
        let retrieved = env.get("x").unwrap();
        assert_eq!(retrieved, Value::Integer(42));
    }
    
    #[test]
    fn test_eval_variable_reference() {
        let mut env = Environment::new();
        let interpreter = Interpreter::new();
        
        // Set variable
        env.set("y".to_string(), Value::Integer(100));
        
        // Reference variable in expression
        let expr = Expression::Binary(
            Operator::Plus,
            Box::new(Expression::Identifier("y".to_string())),
            Box::new(Expression::Number("50".to_string())),
        );
        let result = interpreter.eval_expression(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Integer(150));
    }
    
    #[test]
    fn test_division_by_zero() {
        let mut env = Environment::new();
        let interpreter = Interpreter::new();
        
        let expr = Expression::Binary(
            Operator::Divide,
            Box::new(Expression::Number("5".to_string())),
            Box::new(Expression::Number("0".to_string())),
        );
        
        let result = interpreter.eval_expression(&expr, &mut env);
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }
    
    #[test]
    fn test_undefined_variable() {
        let mut env = Environment::new();
        let interpreter = Interpreter::new();
        
        let expr = Expression::Identifier("undefined_var".to_string());
        let result = interpreter.eval_expression(&expr, &mut env);
        
        assert!(matches!(result, Err(RuntimeError::UndefinedVariable(_))));
    }
}