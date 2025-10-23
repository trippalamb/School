use super::*;
use crate::significance::ast_parser::{Program, Statement, VarType, Expression, BinaryOp, UnaryOp};
use crate::significance::tokenizer::Position;

// Helper function to create a dummy position for testing
fn dummy_pos() -> Position {
    Position { line: 1, column: 1 }
}

// Helper function to create a simple program with statements
fn create_program(statements: Vec<Statement>) -> Program {
    Program { statements }
}

#[test]
fn test_executor_new() {
    let executor = Executor::new();
    assert!(executor.get_var("x").is_none());
}

#[test]
fn test_variable_declaration() {
    let mut executor = Executor::new();
    
    let declaration = Statement::VarDeclaration {
        name: "x".to_string(),
        var_type: VarType::Real,
        pos: dummy_pos(),
    };
    
    executor.execute_statement(&declaration);
    
    let var = executor.get_var("x").unwrap();
    assert_eq!(var.get_value(), 0.0); // Default value should be 0.0
}

#[test]
fn test_variable_assignment() {
    let mut executor = Executor::new();
    
    // First declare the variable
    let declaration = Statement::VarDeclaration {
        name: "x".to_string(),
        var_type: VarType::Real,
        pos: dummy_pos(),
    };
    executor.execute_statement(&declaration);
    
    // Then assign a value
    let assignment = Statement::Assignment {
        name: "x".to_string(),
        value: Expression::NumberWithUncertainty {
            value: 5.5,
            error: 0.1,
            pos: dummy_pos(),
        },
        pos: dummy_pos(),
    };
    
    executor.execute_statement(&assignment);
    
    let var = executor.get_var("x").unwrap();
    assert_eq!(var.get_value(), 5.5);
}

#[test]
fn test_evaluate_number() {
    let mut executor = Executor::new();
    
    let number_expr = Expression::NumberWithUncertainty {
        value: 42.5,
        error: 0.5,
        pos: dummy_pos(),
    };
    
    let result = executor.evaluate_expression(&number_expr);
    assert_eq!(result, 42.5);
}

#[test]
fn test_evaluate_variable() {
    let mut executor = Executor::new();
    
    // Set up a variable
    let declaration = Statement::VarDeclaration {
        name: "test_var".to_string(),
        var_type: VarType::Real,
        pos: dummy_pos(),
    };
    let assignment = Statement::Assignment {
        name: "test_var".to_string(),
        value: Expression::NumberWithUncertainty {
            value: 3.14,
            error: 0.01,
            pos: dummy_pos(),
        },
        pos: dummy_pos(),
    };
    
    executor.execute_statement(&declaration);
    executor.execute_statement(&assignment);
    
    // Now evaluate a variable expression
    let var_expr = Expression::Variable("test_var".to_string());
    let result = executor.evaluate_expression(&var_expr);
    assert_eq!(result, 3.14);
}

#[test]
fn test_binary_operations() {
    let mut executor = Executor::new();
    
    // Test addition
    let add_expr = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 10.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 5.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    assert_eq!(executor.evaluate_expression(&add_expr), 15.0);
    
    // Test subtraction
    let sub_expr = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 10.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Sub,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 3.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    assert_eq!(executor.evaluate_expression(&sub_expr), 7.0);
    
    // Test multiplication
    let mul_expr = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 4.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Mul,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 3.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    assert_eq!(executor.evaluate_expression(&mul_expr), 12.0);
    
    // Test division
    let div_expr = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 15.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Div,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 3.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    assert_eq!(executor.evaluate_expression(&div_expr), 5.0);
}

#[test]
fn test_power_and_root_operations() {
    let mut executor = Executor::new();
    
    // Test power (2^3 = 8)
    let power_expr = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 2.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Power,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 3.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    assert_eq!(executor.evaluate_expression(&power_expr), 8.0);
    
    // Test root (8 // 3 = 2, since 8^(1/3) = 2)
    let root_expr = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 8.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Root,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 3.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    assert_eq!(executor.evaluate_expression(&root_expr), 2.0);
}

#[test]
fn test_modulus_operation() {
    let mut executor = Executor::new();
    
    let mod_expr = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 17.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Mod,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 5.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    assert_eq!(executor.evaluate_expression(&mod_expr), 2.0);
}

#[test]
fn test_division_by_zero() {
    let mut executor = Executor::new();
    
    let div_by_zero = Expression::Binary {
        left: Box::new(Expression::NumberWithUncertainty {
            value: 10.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        op: BinaryOp::Div,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 0.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    let result = executor.evaluate_expression(&div_by_zero);
    assert!(result.is_infinite());
    
    let errors = executor.get_errors();
    // Check that error was recorded
    assert_eq!(errors.len(), 1);
    match &errors[0] {
        RunTimeError::DivisionByZero(_) => {}, // Expected
    }
}

#[test]
fn test_complex_expression_with_variables() {
    let mut executor = Executor::new();
    
    // Set up variables: x = 5, y = 3
    executor.execute_statement(&Statement::VarDeclaration {
        name: "x".to_string(),
        var_type: VarType::Real,
        pos: dummy_pos(),
    });
    executor.execute_statement(&Statement::Assignment {
        name: "x".to_string(),
        value: Expression::NumberWithUncertainty { value: 5.0, error: 0.0, pos: dummy_pos() },
        pos: dummy_pos(),
    });
    
    executor.execute_statement(&Statement::VarDeclaration {
        name: "y".to_string(),
        var_type: VarType::Real,
        pos: dummy_pos(),
    });
    executor.execute_statement(&Statement::Assignment {
        name: "y".to_string(),
        value: Expression::NumberWithUncertainty { value: 3.0, error: 0.0, pos: dummy_pos() },
        pos: dummy_pos(),
    });
    
    // Test expression: x * y + 2 = 5 * 3 + 2 = 17
    let complex_expr = Expression::Binary {
        left: Box::new(Expression::Binary {
            left: Box::new(Expression::Variable("x".to_string())),
            op: BinaryOp::Mul,
            right: Box::new(Expression::Variable("y".to_string())),
            pos: dummy_pos(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expression::NumberWithUncertainty {
            value: 2.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    let result = executor.evaluate_expression(&complex_expr);
    assert_eq!(result, 17.0);
}

#[test]
fn test_execute_program() {
    let mut executor = Executor::new();
    
    let program = create_program(vec![
        Statement::VarDeclaration {
            name: "result".to_string(),
            var_type: VarType::Real,
            pos: dummy_pos(),
        },
        Statement::Assignment {
            name: "result".to_string(),
            value: Expression::Binary {
                left: Box::new(Expression::NumberWithUncertainty {
                    value: 10.0,
                    error: 0.0,
                    pos: dummy_pos(),
                }),
                op: BinaryOp::Mul,
                right: Box::new(Expression::NumberWithUncertainty {
                    value: 2.0,
                    error: 0.0,
                    pos: dummy_pos(),
                }),
                pos: dummy_pos(),
            },
            pos: dummy_pos(),
        },
    ]);
    
    executor.execute_program(&program);
    
    let result_var = executor.get_var("result").unwrap();
    assert_eq!(result_var.get_value(), 20.0);
}

#[test]
fn test_executor_reset() {
    let mut executor = Executor::new();
    
    // Add a variable
    executor.execute_statement(&Statement::VarDeclaration {
        name: "temp".to_string(),
        var_type: VarType::Real,
        pos: dummy_pos(),
    });
    
    assert!(executor.get_var("temp").is_some());
    
    // Reset should clear everything
    executor.reset();
    assert!(executor.get_var("temp").is_none());
}

#[test]
fn test_unary_operations() {
    let mut executor = Executor::new();
    
    let unary_expr = Expression::Unary {
        op: UnaryOp::Minus,
        operand: Box::new(Expression::NumberWithUncertainty {
            value: 5.0,
            error: 0.0,
            pos: dummy_pos(),
        }),
        pos: dummy_pos(),
    };
    
    let result = executor.evaluate_expression(&unary_expr);
    assert_eq!(result, -5.0);
}

#[test]
#[should_panic(expected = "Variable not declared")]
fn test_undefined_variable_access() {
    let mut executor = Executor::new();
    
    let var_expr = Expression::Variable("undefined_var".to_string());
    executor.evaluate_expression(&var_expr); // Should panic
}

#[test]
#[should_panic(expected = "Variable not declared")]
fn test_assignment_to_undeclared_variable() {
    let mut executor = Executor::new();
    
    let assignment = Statement::Assignment {
        name: "undeclared".to_string(),
        value: Expression::NumberWithUncertainty {
            value: 10.0,
            error: 0.0,
            pos: dummy_pos(),
        },
        pos: dummy_pos(),
    };
    
    executor.execute_statement(&assignment); // Should panic
}