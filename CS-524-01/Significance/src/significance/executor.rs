use std::collections::HashMap;
use crate::significance::ast_parser::{Program, Statement, VarType, Expression, BinaryOp, UnaryOp};
use crate::significance::tokenizer::Position;

#[derive(Debug, Clone)]
pub enum RunTimeError {
    DivisionByZero(Position)
}

#[derive(Debug, Clone)]
pub struct VarRunTime{
    var_type: VarType,
    value: f64
}

impl VarRunTime {
    pub fn get_value(&self) -> f64 {
        self.value
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
                self.evaluate_expression(expression);
            }
        }
    }

    fn declare_variable(&mut self, name: &str, var_type: &VarType, _: &Position) {
        //due to semantic analyzer I already know that the variable is not previously declared
        self.run_time_vars.insert(
            name.to_string(),
            VarRunTime {
                var_type: var_type.clone(),
                value: 0.0
            }
        );
    }

    fn assign_variable(&mut self, name: &str, value: &Expression, _: &Position) {
        let value = self.evaluate_expression(value);
        let var = self.run_time_vars.get_mut(name).expect("Variable not declared");
        var.value = value;
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> f64 {
        match expression {
            Expression::NumberWithUncertainty { value, error:_, pos:_ } => value.clone(),
            Expression::Variable(name) => {
                let var = self.run_time_vars.get(name).expect("Variable not declared");
                var.value
            },
            Expression::Binary { left, op, right, pos } => {
                let left_value = self.evaluate_expression(left);
                let right_value = self.evaluate_expression(right);

                match op {
                    BinaryOp::Add => left_value + right_value,
                    BinaryOp::Sub => left_value - right_value,
                    BinaryOp::Mul => left_value * right_value,
                    BinaryOp::Div => {
                        if right_value == 0.0 {
                            self.errors.push(RunTimeError::DivisionByZero(pos.clone()));
                        }
                        left_value / right_value
                    },
                    BinaryOp::Mod => left_value % right_value,
                    BinaryOp::Power => left_value.powf(right_value),
                    BinaryOp::Root => left_value.powf(1.0 / right_value),
                }
            },
            Expression::Unary { op, operand, pos:_ } => {
                let operand_value = self.evaluate_expression(operand);
                match op {
                    UnaryOp::Plus => operand_value,
                    UnaryOp::Minus => -operand_value,
                }
            },
            Expression::FunctionCall { name:_, args:_, pos:_ } => todo!(),

        }
    }

    pub fn get_errors(&self) -> Vec<RunTimeError> {
        self.errors.clone()
    }
}