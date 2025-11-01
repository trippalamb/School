use std::collections::HashMap;
use crate::Real;
use crate::significance::ast_parser::{Program, Statement, VarType, Expression, BinaryOp, UnaryOp};
use crate::significance::tokenizer::Position;
use crate::significance::std_lib_call;

#[derive(Debug, Clone)]
pub enum RunTimeError {
    DivisionByZero(Position),
    UndefinedVariable(String, Position)
}

#[derive(Debug, Clone)]
pub struct VarRunTime{
    //var_type: VarType,
    value: Real
}

impl VarRunTime {
    pub fn get_value(&self) -> &Real {
        &self.value
    }
}

pub struct Executor {
    run_time_vars: HashMap<String, VarRunTime>,
    errors: Vec<RunTimeError>
}


impl Executor{
    pub fn new() -> Self {
        Self {
            run_time_vars: HashMap::new(),
            errors: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        self.run_time_vars.clear();
    }

    pub fn get_var(&self, name: &str) -> Option<VarRunTime> {
        self.run_time_vars.get(name).cloned()
    }

    pub fn execute_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.execute_statement(statement);
        }
    }

    pub fn execute_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDeclaration { name, var_type, pos } => {
                self.declare_variable(name, var_type, pos);
            }
            Statement::Assignment { name, value, pos } => {
                self.assign_variable(name, value, pos);
            }
            Statement::Expression(expression) => {
                let value = self.evaluate_expression(expression);
                print!("{}\n", value);
            }
        }
    }

    pub fn declare_variable(&mut self, name: &str, _var_type: &VarType, _: &Position) {
        //due to semantic analyzer I already know that the variable is not previously declared
        self.run_time_vars.insert(
            name.to_string(),
            VarRunTime {
                //var_type: var_type.clone(),
                value: Real::new(0.0)
            }
        );
    }

    fn assign_variable(&mut self, name: &str, value: &Expression, pos: &Position) {
        let value = self.evaluate_expression(value);
        
        if let Some(var) = self.run_time_vars.get_mut(name) {
            var.value = value;
        } else {
            self.errors.push(RunTimeError::UndefinedVariable(name.to_string(), pos.clone()));
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> Real {
        match expression {
            Expression::NumberWithUncertainty { value, error, pos:_ } => Real::with_error(value.clone(), error.clone()),
            Expression::Variable(name) => self.evaluate_variable(name),
            Expression::Binary { left, op, right, pos } => {
                self.evaluate_expression_binary(left, op, right, pos)
            },
            Expression::Unary { op, operand, pos } => {
                self.evaluate_expression_unary(op, operand, pos)
            },
            Expression::FunctionCall { name, args, pos } => {
                self.evaluate_function_call(name, args, pos)
            }
        }
    }

    fn evaluate_variable(&mut self, name: &str) -> Real {
        if let Some(var) = self.run_time_vars.get(name) {
            var.value.clone()
        } else {
            self.errors.push(RunTimeError::UndefinedVariable(
                name.to_string(),
                Position { line: 0, column: 0 } // Position not available here
            ));
            Real::new(0.0) // Return default value after logging error
        }
    }

    fn evaluate_expression_binary(&mut self, left: &Expression, op: &BinaryOp, right: &Expression, pos: &Position) -> Real {
        let left_value = self.evaluate_expression(left);
        let right_value = self.evaluate_expression(right);

        match op {
            BinaryOp::Add => left_value + right_value,
            BinaryOp::Sub => left_value - right_value,
            BinaryOp::Mul => left_value * right_value,
            BinaryOp::Div => {
                if right_value == Real::new(0.0) {
                    self.errors.push(RunTimeError::DivisionByZero(pos.clone()));
                }
                left_value / right_value
            },
            BinaryOp::Mod => left_value % right_value,
            BinaryOp::Power => left_value.power(right_value),
            BinaryOp::Root => left_value.root(right_value),
        }
    }

    fn evaluate_expression_unary(&mut self, op: &UnaryOp, operand: &Expression, _pos: &Position) -> Real {
        let operand_value = self.evaluate_expression(operand);
        match op {
            UnaryOp::Plus => operand_value,
            UnaryOp::Minus => -operand_value,
        }
    }

    fn evaluate_function_call(&mut self, name: &str, args: &[Expression], pos: &Position) -> Real {
        let vals: Vec<Real> = args.iter().map(|arg| self.evaluate_expression(arg)).collect();
        std_lib_call(name, &vals, pos)
    }

    pub fn get_errors(&self) -> Vec<RunTimeError> {
        self.errors.clone()
    }

}