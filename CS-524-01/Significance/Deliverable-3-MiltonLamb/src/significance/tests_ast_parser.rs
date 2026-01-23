//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

use super::*;

// ============================================================================
// Helper Functions for Creating Test Data
// ============================================================================

/// Helper function to create tokens with dummy position info for testing
fn create_tokens(tokens: Vec<Token>) -> Vec<TokenWithPos> {
    tokens.into_iter().enumerate().map(|(i, token)| {
        TokenWithPos {
            token,
            position: Position { line: 1, column: i + 1 },
        }
    }).collect()
}

/// Parse a vector of tokens into a Program
fn parse_tokens(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let tokens_with_pos = create_tokens(tokens);
    let mut parser = AstParser::new();
    parser.parse_program(tokens_with_pos)
}

// ============================================================================
// Position Assertion
// ============================================================================

fn assert_position(actual: &Position, expected: &Position) {
    if actual.line != expected.line || actual.column != expected.column {
        panic!(
            "Position mismatch:\n  Expected: line={}, column={}\n  Actual:   line={}, column={}",
            expected.line, expected.column, actual.line, actual.column
        );
    }
}

// ============================================================================
// Expression Assertions
// ============================================================================

/// Assert that an expression matches the expected expression
fn assert_expression(actual: &Expression, expected: &Expression) {
    match (actual, expected) {
        (
            Expression::NumberWithUncertainty { value: av, error: ae, pos: apos },
            Expression::NumberWithUncertainty { value: ev, error: ee, pos: epos }
        ) => {
            assert_expression_number_with_uncertainty(av, ae, apos, ev, ee, epos);
        }
        (Expression::Variable(aname), Expression::Variable(ename)) => {
            assert_expression_variable(aname, ename);
        }
        (
            Expression::Binary { left: al, op: aop, right: ar, pos: apos },
            Expression::Binary { left: el, op: eop, right: er, pos: epos }
        ) => {
            assert_expression_binary(al, aop, ar, apos, el, eop, er, epos);
        }
        (
            Expression::Unary { op: aop, operand: aoperand, pos: apos },
            Expression::Unary { op: eop, operand: eoperand, pos: epos }
        ) => {
            assert_expression_unary(aop, aoperand, apos, eop, eoperand, epos);
        }
        (
            Expression::FunctionCall { name: aname, args: aargs, pos: apos },
            Expression::FunctionCall { name: ename, args: eargs, pos: epos }
        ) => {
            assert_expression_function_call(aname, aargs, apos, ename, eargs, epos);
        }
        _ => {
            panic!(
                "Expression type mismatch:\n  Expected: {:?}\n  Actual:   {:?}",
                expected, actual
            );
        }
    }
}

/// Assert NumberWithUncertainty expression
fn assert_expression_number_with_uncertainty(
    actual_value: &f64,
    actual_error: &f64,
    actual_pos: &Position,
    expected_value: &f64,
    expected_error: &f64,
    expected_pos: &Position,
) {
    assert_position(actual_pos, expected_pos);
    if (actual_value - expected_value).abs() > f64::EPSILON {
        panic!(
            "NumberWithUncertainty value mismatch:\n  Expected: {}\n  Actual:   {}",
            expected_value, actual_value
        );
    }
    if (actual_error - expected_error).abs() > f64::EPSILON {
        panic!(
            "NumberWithUncertainty error mismatch:\n  Expected: {}\n  Actual:   {}",
            expected_error, actual_error
        );
    }
}

/// Assert Variable expression
fn assert_expression_variable(actual_name: &str, expected_name: &str) {
    if actual_name != expected_name {
        panic!(
            "Variable name mismatch:\n  Expected: {}\n  Actual:   {}",
            expected_name, actual_name
        );
    }
}

/// Assert Binary expression
fn assert_expression_binary(
    actual_left: &Box<Expression>,
    actual_op: &BinaryOp,
    actual_right: &Box<Expression>,
    actual_pos: &Position,
    expected_left: &Box<Expression>,
    expected_op: &BinaryOp,
    expected_right: &Box<Expression>,
    expected_pos: &Position,
) {
    assert_position(actual_pos, expected_pos);
    if actual_op != expected_op {
        panic!(
            "Binary operator mismatch:\n  Expected: {:?}\n  Actual:   {:?}",
            expected_op, actual_op
        );
    }
    assert_expression(actual_left.as_ref(), expected_left.as_ref());
    assert_expression(actual_right.as_ref(), expected_right.as_ref());
}

/// Assert Unary expression
fn assert_expression_unary(
    actual_op: &UnaryOp,
    actual_operand: &Box<Expression>,
    actual_pos: &Position,
    expected_op: &UnaryOp,
    expected_operand: &Box<Expression>,
    expected_pos: &Position,
) {
    assert_position(actual_pos, expected_pos);
    if actual_op != expected_op {
        panic!(
            "Unary operator mismatch:\n  Expected: {:?}\n  Actual:   {:?}",
            expected_op, actual_op
        );
    }
    assert_expression(actual_operand.as_ref(), expected_operand.as_ref());
}

/// Assert FunctionCall expression
fn assert_expression_function_call(
    actual_name: &str,
    actual_args: &Vec<Expression>,
    actual_pos: &Position,
    expected_name: &str,
    expected_args: &Vec<Expression>,
    expected_pos: &Position,
) {
    assert_position(actual_pos, expected_pos);
    if actual_name != expected_name {
        panic!(
            "Function name mismatch:\n  Expected: {}\n  Actual:   {}",
            expected_name, actual_name
        );
    }
    if actual_args.len() != expected_args.len() {
        panic!(
            "Function argument count mismatch:\n  Expected: {} args\n  Actual:   {} args",
            expected_args.len(), actual_args.len()
        );
    }
    for (aarg, earg) in actual_args.iter().zip(expected_args.iter()) {
        assert_expression(aarg, earg);
    }
}

// ============================================================================
// Statement Assertions
// ============================================================================

/// Assert that a statement matches the expected statement
fn assert_statement(actual: &Statement, expected: &Statement) {
    match (actual, expected) {
        (
            Statement::VarDeclaration { name: an, var_type: at, pos: ap },
            Statement::VarDeclaration { name: en, var_type: et, pos: ep }
        ) => {
            assert_statement_var_declaration(an, at, ap, en, et, ep);
        }
        (
            Statement::Assignment { name: an, value: av, pos: ap },
            Statement::Assignment { name: en, value: ev, pos: ep }
        ) => {
            assert_statement_assignment(an, av, ap, en, ev, ep);
        }
        (Statement::Expression(aexpr), Statement::Expression(eexpr)) => {
            assert_statement_expression(aexpr, eexpr);
        }
        _ => {
            panic!(
                "Statement type mismatch:\n  Expected: {:?}\n  Actual:   {:?}",
                expected, actual
            );
        }
    }
}

/// Assert VarDeclaration statement
fn assert_statement_var_declaration(
    actual_name: &str,
    actual_type: &VarType,
    actual_pos: &Position,
    expected_name: &str,
    expected_type: &VarType,
    expected_pos: &Position,
) {
    assert_position(actual_pos, expected_pos);
    if actual_name != expected_name {
        panic!(
            "Variable declaration name mismatch:\n  Expected: {}\n  Actual:   {}",
            expected_name, actual_name
        );
    }
    if actual_type != expected_type {
        panic!(
            "Variable declaration type mismatch:\n  Expected: {:?}\n  Actual:   {:?}",
            expected_type, actual_type
        );
    }
}

/// Assert Assignment statement
fn assert_statement_assignment(
    actual_name: &str,
    actual_value: &Expression,
    actual_pos: &Position,
    expected_name: &str,
    expected_value: &Expression,
    expected_pos: &Position,
) {
    assert_position(actual_pos, expected_pos);
    if actual_name != expected_name {
        panic!(
            "Assignment name mismatch:\n  Expected: {}\n  Actual:   {}",
            expected_name, actual_name
        );
    }
    assert_expression(actual_value, expected_value);
}

/// Assert Expression statement
fn assert_statement_expression(actual_expr: &Expression, expected_expr: &Expression) {
    assert_expression(actual_expr, expected_expr);
}

// ============================================================================
// Tests Using the New Assertion Helpers
// ============================================================================

#[test]
fn test_simple_number() {
    let tokens = vec![
        Token::Number(42.5),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::NumberWithUncertainty {
            value: 42.5,
            error: 0.0,
            pos: Position { line: 1, column: 1 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_number_with_uncertainty() {
    let tokens = vec![
        Token::Number(12.3),
        Token::PlusMinus,
        Token::Number(0.5),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::NumberWithUncertainty {
            value: 12.3,
            error: 0.5,
            pos: Position { line: 1, column: 1 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_variable_declaration() {
    let tokens = vec![
        Token::LeftBrace,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Real,
        Token::RightBrace,
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::VarDeclaration {
        name: "x".to_string(),
        var_type: VarType::Real,
        pos: Position { line: 1, column: 1 }
    };
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_assignment() {
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(5.0),
        Token::PlusMinus,
        Token::Number(0.1),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Assignment {
        name: "x".to_string(),
        value: Expression::NumberWithUncertainty {
            value: 5.0,
            error: 0.1,
            pos: Position { line: 1, column: 3 }
        },
        pos: Position { line: 1, column: 1 }
    };
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_variable_reference() {
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::Variable("x".to_string())
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_binary_addition() {
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Identifier("y".to_string()),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::Binary {
            left: Box::new(Expression::Variable("x".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expression::Variable("y".to_string())),
            pos: Position { line: 1, column: 1 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_unary_minus() {
    let tokens = vec![
        Token::Minus,
        Token::Number(5.0),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::Unary {
            op: UnaryOp::Minus,
            operand: Box::new(Expression::NumberWithUncertainty {
                value: 5.0,
                error: 0.0,
                pos: Position { line: 1, column: 2 }
            }),
            pos: Position { line: 1, column: 1 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_parenthesized_expression() {
    let tokens = vec![
        Token::LeftParen,
        Token::Number(42.0),
        Token::RightParen,
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::NumberWithUncertainty {
            value: 42.0,
            error: 0.0,
            pos: Position { line: 1, column: 2 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_function_call_no_args() {
    let tokens = vec![
        Token::Identifier("sqrt".to_string()),
        Token::LeftParen,
        Token::RightParen,
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![],
            pos: Position { line: 1, column: 1 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_function_call_with_args() {
    let tokens = vec![
        Token::Identifier("pow".to_string()),
        Token::LeftParen,
        Token::Number(2.0),
        Token::Comma,
        Token::Number(3.0),
        Token::RightParen,
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::FunctionCall {
            name: "pow".to_string(),
            args: vec![
                Expression::NumberWithUncertainty {
                    value: 2.0,
                    error: 0.0,
                    pos: Position { line: 1, column: 3 }
                },
                Expression::NumberWithUncertainty {
                    value: 3.0,
                    error: 0.0,
                    pos: Position { line: 1, column: 5 }
                }
            ],
            pos: Position { line: 1, column: 1 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_multiple_statements() {
    let tokens = vec![
        Token::LeftBrace,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Real,
        Token::RightBrace,
        Token::Newline,
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(5.0),
        Token::Newline,
        Token::Identifier("x".to_string()),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 3);
    
    // Check declaration
    let expected_decl = Statement::VarDeclaration {
        name: "x".to_string(),
        var_type: VarType::Real,
        pos: Position { line: 1, column: 1 }
    };
    assert_statement(&program.statements[0], &expected_decl);
    
    // Check assignment
    let expected_assign = Statement::Assignment {
        name: "x".to_string(),
        value: Expression::NumberWithUncertainty {
            value: 5.0,
            error: 0.0,
            pos: Position { line: 1, column: 9 }
        },
        pos: Position { line: 1, column: 7 }
    };
    assert_statement(&program.statements[1], &expected_assign);
    
    // Check expression
    let expected_expr = Statement::Expression(
        Expression::Variable("x".to_string())
    );
    assert_statement(&program.statements[2], &expected_expr);
}

#[test]
fn test_operator_precedence() {
    // Test that 2 + 3 * 4 parses as 2 + (3 * 4)
    let tokens = vec![
        Token::Number(2.0),
        Token::Plus,
        Token::Number(3.0),
        Token::Multiply,
        Token::Number(4.0),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::Binary {
            left: Box::new(Expression::NumberWithUncertainty {
                value: 2.0,
                error: 0.0,
                pos: Position { line: 1, column: 1 }
            }),
            op: BinaryOp::Add,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::NumberWithUncertainty {
                    value: 3.0,
                    error: 0.0,
                    pos: Position { line: 1, column: 3 }
                }),
                op: BinaryOp::Mul,
                right: Box::new(Expression::NumberWithUncertainty {
                    value: 4.0,
                    error: 0.0,
                    pos: Position { line: 1, column: 5 }
                }),
                pos: Position { line: 1, column: 3 }
            }),
            pos: Position { line: 1, column: 1 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}

#[test]
fn test_parse_error() {
    let tokens = vec![
        Token::Multiply, // Invalid: can't start with binary operator, unless it is also a unary
        Token::Number(5.0),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_err());
}

#[test]
fn test_skip_comments_and_newlines() {
    let tokens = vec![
        Token::Comment("This is a comment".to_string()),
        Token::Newline,
        Token::Number(42.0),
        Token::Newline,
        Token::Comment("Another comment".to_string()),
        Token::EOF,
    ];
    
    let result = parse_tokens(tokens);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
    
    let expected = Statement::Expression(
        Expression::NumberWithUncertainty {
            value: 42.0,
            error: 0.0,
            pos: Position { line: 1, column: 3 }
        }
    );
    
    assert_statement(&program.statements[0], &expected);
}