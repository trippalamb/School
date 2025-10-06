use super::*;

// Helper function to create tokens with dummy position info for testing
fn create_tokens(tokens: Vec<Token>) -> Vec<TokenWithPos> {
    tokens.into_iter().enumerate().map(|(i, token)| {
        TokenWithPos {
            token,
            position: Position { line: 1, column: i + 1 },
        }
    }).collect()
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let tokens_with_pos = create_tokens(tokens);
    let mut parser = AstParser::new();
    parser.parse_program(tokens_with_pos)
}

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
    
    match &program.statements[0] {
        Statement::Expression(Expression::NumberWithUncertainty { value, error, pos:Position { line:1, column:1 } }) => {
            assert_eq!(*value, 42.5);
            assert_eq!(*error, 0.0);
        }
        _ => panic!("Expected number expression, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::NumberWithUncertainty { value, error, pos:Position { line:1, column:1 } }) => {
            assert_eq!(*value, 12.3);
            assert_eq!(*error, 0.5);
        }
        _ => panic!("Expected number with uncertainty, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::VarDeclaration { name, var_type, pos:Position { line:1, column:1 } } => {
            assert_eq!(name, "x");
            assert_eq!(*var_type, VarType::Real);
        }
        _ => panic!("Expected variable declaration, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Assignment { name, value, pos:Position { line:1, column:1 } } => {
            assert_eq!(name, "x");
            match value {
                Expression::NumberWithUncertainty { value, error, pos:Position { line:1, column:1 } } => {
                    assert_eq!(*value, 5.0);
                    assert_eq!(*error, 0.1);
                }
                _ => panic!("Expected number with uncertainty in assignment"),
            }
        }
        _ => panic!("Expected assignment statement, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::Variable(name)) => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected variable expression, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::Binary { left, op, right, pos:Position { line:1, column:1 } }) => {
            assert_eq!(*op, BinaryOp::Add);
            
            match (left.as_ref(), right.as_ref()) {
                (Expression::Variable(left_name), Expression::Variable(right_name)) => {
                    assert_eq!(left_name, "x");
                    assert_eq!(right_name, "y");
                }
                _ => panic!("Expected variable operands in binary expression"),
            }
        }
        _ => panic!("Expected binary expression, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::Unary { op, operand, pos:Position { line:1, column:1 } }) => {
            assert_eq!(*op, UnaryOp::Minus);
            match operand.as_ref() {
                Expression::NumberWithUncertainty { value, error, pos:Position { line:1, column:1 } } => {
                    assert_eq!(*value, 5.0);
                    assert_eq!(*error, 0.0);
                }
                _ => panic!("Expected number in unary operand"),
            }
        }
        _ => panic!("Expected unary expression, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::NumberWithUncertainty { value, error, pos:Position { line:1, column:1 } }) => {
            assert_eq!(*value, 42.0);
            assert_eq!(*error, 0.0);
        }
        _ => panic!("Expected number expression from parentheses, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::FunctionCall { name, args, pos:Position { line:1, column:1 } }) => {
            assert_eq!(name, "sqrt");
            assert_eq!(args.len(), 0);
        }
        _ => panic!("Expected function call, got {:?}", program.statements[0]),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::FunctionCall { name, args, pos:Position { line:1, column:1 } }) => {
            assert_eq!(name, "pow");
            assert_eq!(args.len(), 2);
            
            match (&args[0], &args[1]) {
                (Expression::NumberWithUncertainty { value: v1, error: e1, pos:Position { line:1, column:1 } },
                    Expression::NumberWithUncertainty { value: v2, error: e2, pos:Position { line:1, column:1 } }) => {
                    assert_eq!(*v1, 2.0);
                    assert_eq!(*e1, 0.0);
                    assert_eq!(*v2, 3.0);
                    assert_eq!(*e2, 0.0);
                }
                _ => panic!("Expected number arguments in function call"),
            }
        }
        _ => panic!("Expected function call, got {:?}", program.statements[0]),
    }
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
    match &program.statements[0] {
        Statement::VarDeclaration { name, var_type, pos:Position { line:1, column:1 } } => {
            assert_eq!(name, "x");
            assert_eq!(*var_type, VarType::Real);
        }
        _ => panic!("Expected variable declaration as first statement"),
    }
    
    // Check assignment
    match &program.statements[1] {
        Statement::Assignment { name, .. } => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected assignment as second statement"),
    }
    
    // Check expression
    match &program.statements[2] {
        Statement::Expression(Expression::Variable(name)) => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected variable expression as third statement"),
    }
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::Binary { left, op, right, pos:Position { line:1, column:1 } }) => {
            assert_eq!(*op, BinaryOp::Add);
            
            // Left should be 2.0
            match left.as_ref() {
                Expression::NumberWithUncertainty { value, .. } => assert_eq!(*value, 2.0),
                _ => panic!("Expected number as left operand"),
            }
            
            // Right should be (3.0 * 4.0)
            match right.as_ref() {
                Expression::Binary { left: inner_left, op: inner_op, right: inner_right, pos:Position { line:1, column:1 } } => {
                    assert_eq!(*inner_op, BinaryOp::Mul);
                    
                    match (inner_left.as_ref(), inner_right.as_ref()) {
                        (Expression::NumberWithUncertainty { value: v1, .. },
                            Expression::NumberWithUncertainty { value: v2, .. }) => {
                            assert_eq!(*v1, 3.0);
                            assert_eq!(*v2, 4.0);
                        }
                        _ => panic!("Expected numbers in multiplication"),
                    }
                }
                _ => panic!("Expected multiplication as right operand"),
            }
        }
        _ => panic!("Expected binary expression"),
    }
}

#[test]
fn test_parse_error() {
    let tokens = vec![
        Token::Plus, // Invalid: can't start with binary operator
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
    
    match &program.statements[0] {
        Statement::Expression(Expression::NumberWithUncertainty { value, .. }) => {
            assert_eq!(*value, 42.0);
        }
        _ => panic!("Expected number expression"),
    }
}
