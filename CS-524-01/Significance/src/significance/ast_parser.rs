use crate::significance::tokenizer::{Token, TokenWithPos};

/// Abstract Syntax Tree node types
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDeclaration { name: String, var_type: VarType },
    Assignment { name: String, value: Expression },
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Real,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    NumberWithUncertainty { value: f64, error: f64 },
    Variable(String),
    Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression> },
    Unary { op: UnaryOp, operand: Box<Expression> },
    FunctionCall { name: String, args: Vec<Expression> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Power,    // **
    Root,     // //
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Plus,     // +
    Minus,    // -
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}:{}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Recursive descent parser
pub struct Parser {
    tokens: Vec<TokenWithPos>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self { tokens:vec![], current: 0 }
    }
    
    /// Parse the entire program
    pub fn parse(&mut self, tokens: Vec<TokenWithPos>) -> Result<Program, ParseError> {
        self.tokens = tokens;
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            self.skip_newlines_and_comments();
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program { statements })
    }
    
    /// Parse a single statement
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        
        match self.current_token() {
            Token::LeftBrace => {
                self.parse_var_declaration()
            },
            Token::Identifier(_) => {
                self.parse_starting_identifier()
                
            },
            _ => {
                Ok(Statement::Expression(self.parse_expression()?))
            }
        }

    }

    fn parse_var_declaration(&mut self) -> Result<Statement, ParseError> {

        self.expect_token(Token::LeftBrace)?;

        let name = self.consume_identifier()?;
        
        self.expect_token(Token::Colon)?;

        let var_type = self.consume_var_type()?;

        self.expect_token(Token::RightBrace)?;

        Ok(Statement::VarDeclaration { name, var_type })
        
    }

    fn parse_starting_identifier(&mut self) -> Result<Statement, ParseError> {
        let peek_token = self.peek_token();
                
        match peek_token {
            Token::Assign => {
                self.parse_assignment()
            },
            _ => {
                Ok(Statement::Expression(self.parse_expression()?))
            }
        }
    }

    fn parse_assignment(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume_identifier()?;

        self.expect_token(Token::Assign)?;

        let expression = self.parse_expression()?;

        Ok(Statement::Assignment { name, value: expression })
    }
    
    /// Parse expressions (handles precedence: term level + -)
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {

        let mut left = self.parse_term()?;

        while let Some(op) = self.try_consume_term_operator() {

            let right = self.parse_term()?;

            left = Expression::Binary { 
                left: Box::new(left),
                op,
                right: Box::new(right) 
            };
        }

        Ok(left)

    }
    
    /// Parse term level (handles * / %)
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_factor()?;

        while let Some(op) = self.try_consume_factor_operator() {

            let right = self.parse_factor()?;

            left = Expression::Binary { 
                left: Box::new(left),
                op,
                right: Box::new(right) 
            };
        }

        Ok(left)
    }
    
    /// Parse factor level (handles ** //)  
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_unary()?;

        if let Some(op) = self.try_consume_power_operator() {

            let right = self.parse_factor()?;

            Ok(Expression::Binary { 
                left: Box::new(left),
                op,
                right: Box::new(right) 
            })
        }
        else{
            Ok(left)
        }

    }
    
    /// Parse unary expressions (+ - prefix)
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {

        match self.try_consume_unary_operator() {
            Some(op) => Ok(Expression::Unary { op, operand: Box::new(self.parse_unary()?) }),
            None => self.parse_primary(),
        }

    }
    
    /// Parse primary expressions (numbers, variables, function calls, parentheses)
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {

        let current = self.advance().clone();
        let next = self.peek_token();

        match (current, next) {
            (Token::LeftParen, _) => {
                let expr = self.parse_expression()?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            },
            (Token::Identifier(name), Token::LeftParen) => {
                self.parse_function_call(name)
            }
            (Token::Identifier(name), _) => Ok(Expression::Variable(name.to_string())), 
            (Token::Number(n), Token::PlusMinus) => { 
                self.parse_number_with_uncertainty(n) 
            },
            (Token::Number(n), _) => { 
                Ok(Expression::NumberWithUncertainty { value: n, error: 0.0 }) 
            },
            _ => Err(self.error("Expected expression")),
        }

    }

    fn parse_number_with_uncertainty(&mut self, number: f64) -> Result<Expression, ParseError> {

        self.expect_token(Token::PlusMinus)?;
        match self.advance() {
            Token::Number(error) => {
                Ok(Expression::NumberWithUncertainty { value:number , error: *error })
            },
            _ => {
                Err(self.error("Expected uncertainty number"))
            }
        }

    }

    fn parse_function_call(&mut self, name: String) -> Result<Expression, ParseError> {

        self.expect_token(Token::LeftParen)?;
        let args = self.parse_argument_list()?;
        self.expect_token(Token::RightParen)?;
        Ok(Expression::FunctionCall { name, args })
    }

    fn parse_argument_list(&mut self) -> Result<Vec<Expression>, ParseError> {

        let mut args = Vec::new();
        if !self.current_token_is(&Token::RightParen) {
            args.push(self.parse_expression().unwrap());
            while self.current_token_is(&Token::Comma) {
                self.advance();
                args.push(self.parse_expression()?);
            }
        }
        Ok(args)
    }
    
    // Utility functions for parser state management
    
    fn skip_newlines_and_comments(&mut self) {
        while !self.is_at_end() && matches!(self.current_token(), Token::Newline | Token::Comment(_)) {
            self.advance();
        }
    }

    fn current_token(&self) -> &Token {
        if self.is_at_end() {
            &Token::EOF
        } else {
            &self.tokens[self.current].token
        }
    }
    
    fn current_token_is(&self, expected: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(expected)
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }
    
    fn previous_token(&self) -> &Token {
        &self.tokens[self.current - 1].token
    }

    fn peek_token(&self) -> &Token {
        if self.is_at_end() {
            &Token::EOF
        } else {
            &self.tokens[self.current].token
        }
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::EOF)
    }

    fn try_consume_term_operator(&mut self) -> Option<BinaryOp> {
        match self.current_token() {
            Token::Plus => { self.advance(); Some(BinaryOp::Add) },
            Token::Minus => { self.advance(); Some(BinaryOp::Sub) },
            _ => None,  // No operator found, don't advance
        }
    }

    fn try_consume_unary_operator(&mut self) -> Option<UnaryOp> {
        match self.current_token() {
            Token::Plus => { self.advance(); Some(UnaryOp::Plus) },
            Token::Minus => { self.advance(); Some(UnaryOp::Minus) },
            _ => None,  // No operator found, don't advance
        }
    }

    fn try_consume_factor_operator(&mut self) -> Option<BinaryOp> {
        match self.current_token() {
            Token::Multiply => { self.advance(); Some(BinaryOp::Mul) },
            Token::Divide => { self.advance(); Some(BinaryOp::Div) },
            Token::Modulus => { self.advance(); Some(BinaryOp::Mod) },
            _ => None,  // No operator found, don't advance
        }
    }

    fn try_consume_power_operator(&mut self) -> Option<BinaryOp> {
        match self.current_token() {
            Token::Power => { self.advance(); Some(BinaryOp::Power) },
            Token::Root => { self.advance(); Some(BinaryOp::Root) },
            _ => None,  // No operator found, don't advance
        }
    }
    
    fn current_position(&self) -> (usize, usize) {
        if self.current < self.tokens.len() {
            let pos = self.tokens[self.current].position;
            (pos.line, pos.column)
        } else {
            let pos = self.tokens.last().unwrap().position;
            (pos.line, pos.column)
        }
    }

    fn consume_var_type(&mut self) -> Result<VarType, ParseError> {
        let token = self.advance();
        match token {
            Token::Real => Ok(VarType::Real),
            _ => Err(self.error("Expected variable type")), //TODO: improve error
        }
    }

    fn consume_identifier(&mut self) -> Result<String, ParseError> {
        match self.current_token() {
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                Ok(name)
            },
            _ => Err(self.error("Expected identifier")), //TODO: improve error
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token_is(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("Expected {:?}", expected)))
        }
    }

    fn error(&self, message: &str) -> ParseError {
        let (line, column) = self.current_position();
        ParseError { 
            message: message.to_string(), 
            line, 
            column 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::significance::tokenizer::{Token, TokenWithPos, Position};
    
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
        let mut parser = Parser::new();
        parser.parse(tokens_with_pos)
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
            Statement::Expression(Expression::NumberWithUncertainty { value, error }) => {
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
            Statement::Expression(Expression::NumberWithUncertainty { value, error }) => {
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
            Statement::VarDeclaration { name, var_type } => {
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
            Statement::Assignment { name, value } => {
                assert_eq!(name, "x");
                match value {
                    Expression::NumberWithUncertainty { value, error } => {
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
            Statement::Expression(Expression::Binary { left, op, right }) => {
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
            Statement::Expression(Expression::Unary { op, operand }) => {
                assert_eq!(*op, UnaryOp::Minus);
                match operand.as_ref() {
                    Expression::NumberWithUncertainty { value, error } => {
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
            Statement::Expression(Expression::NumberWithUncertainty { value, error }) => {
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
            Statement::Expression(Expression::FunctionCall { name, args }) => {
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
            Statement::Expression(Expression::FunctionCall { name, args }) => {
                assert_eq!(name, "pow");
                assert_eq!(args.len(), 2);
                
                match (&args[0], &args[1]) {
                    (Expression::NumberWithUncertainty { value: v1, error: e1 },
                     Expression::NumberWithUncertainty { value: v2, error: e2 }) => {
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
            Statement::VarDeclaration { name, var_type } => {
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
            Statement::Expression(Expression::Binary { left, op, right }) => {
                assert_eq!(*op, BinaryOp::Add);
                
                // Left should be 2.0
                match left.as_ref() {
                    Expression::NumberWithUncertainty { value, .. } => assert_eq!(*value, 2.0),
                    _ => panic!("Expected number as left operand"),
                }
                
                // Right should be (3.0 * 4.0)
                match right.as_ref() {
                    Expression::Binary { left: inner_left, op: inner_op, right: inner_right } => {
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
}