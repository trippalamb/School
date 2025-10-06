use crate::significance::tokenizer::Position;
use crate::significance::ast_parser::{Program, Statement, VarType, Expression};
use std::collections::HashMap;

pub enum SemanticError {
    VariableNotDeclared(String, Position),   
    VariableAlreadyDeclared(String, Position),
    FunctionNotDeclared(String, Position),
}

pub struct SemanticAnalyzer {
    symbol_table: HashMap<String, VarInfo>,
    errors: Vec<SemanticError>,
}

pub struct VarInfo {
    var_type: VarType,
    declared_at: Position,
}

impl VarInfo {
    pub fn get_type(&self) -> &VarType {
        &self.var_type
    }

    pub fn get_declared_at(&self) -> &Position {
        &self.declared_at
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec<SemanticError> {
        &self.errors
    }

    pub fn reset(&mut self) {
        self.symbol_table.clear();
        self.errors.clear();
    }

    pub fn analyze_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
    }

    pub fn import_standard_library(&mut self) {
        let mut std_symbol_table = HashMap::new();
        std_symbol_table.insert("sin".to_string(), VarInfo { var_type: VarType::RealFunction, declared_at: Position { line: 0, column: 0 } });
        std_symbol_table.insert("cos".to_string(), VarInfo { var_type: VarType::RealFunction, declared_at: Position { line: 0, column: 0 } });
        std_symbol_table.insert("sqrt".to_string(), VarInfo { var_type: VarType::RealFunction, declared_at: Position { line: 0, column: 0 } });
        self.import_library(std_symbol_table);
    }

    pub fn import_library(&mut self, library: HashMap<String, VarInfo>) {
        self.symbol_table.extend(library);
    }

    pub fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDeclaration { name, var_type, pos } => {
                self.declare_variable(name, var_type, pos);
            }
            Statement::Assignment { name, value, pos } => {
                self.analyze_assignment(name, value, pos);
            }
            Statement::Expression(expression) => {
                self.analyze_expression(expression, &Position { line: 0, column: 0 });
            }
        }
    }

    pub fn declare_variable(&mut self, name: &str, var_type: &VarType, pos: &Position) {

        if self.symbol_table.contains_key(name) {
            self.errors.push(SemanticError::VariableAlreadyDeclared(name.to_string(), pos.clone()));
        }

        self.symbol_table.insert(
            name.to_string(),
            VarInfo {
                var_type: var_type.clone(),
                declared_at: pos.clone()
            }
        );
    }

    pub fn analyze_assignment(&mut self, name: &str, value: &Expression, pos: &Position) {
        if self.symbol_table.get(name).is_none() {
            self.errors.push(SemanticError::VariableNotDeclared(name.to_string(), pos.clone()));
        }
        self.analyze_expression(value, pos);
    }

    pub fn analyze_expression(&mut self, expression: &Expression, pos: &Position) {
        match expression {
            Expression::Binary { left, op: _, right, pos } => {
                self.analyze_expression(left, pos);
                self.analyze_expression(right, pos);
            },
            Expression::Unary { op: _, operand, pos } => {
                self.analyze_expression(operand, pos);
            },
            Expression::NumberWithUncertainty { value: _, error: _ , pos:_} => {

            },
            Expression::FunctionCall { name, args, pos } => {
                self.analyze_function_call(name, args, pos); 
            },
            Expression::Variable(name) => {
                if self.symbol_table.get(name).is_none() {
                    self.errors.push(SemanticError::VariableNotDeclared(name.to_string(), pos.clone()));
                }
            }
        }
    }

    pub fn analyze_function_call(&mut self, name: &str, args: &Vec<Expression>, pos: &Position) {
        if self.symbol_table.get(name).is_none() {
            self.errors.push(SemanticError::FunctionNotDeclared(name.to_string(), pos.clone()));
        }
        for arg in args {
            self.analyze_expression(arg, pos);
        }
    }

}