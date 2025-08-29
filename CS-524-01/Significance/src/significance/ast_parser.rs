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
        
        // Safety check - ensure EOF is present
        if self.tokens.is_empty() || !matches!(self.tokens.last().unwrap().token, Token::EOF) {
            return Err(ParseError { 
                message: "Token stream must end with EOF".to_string(),
                line: 1, 
                column: 1 
            });
        }
        
        let mut statements = Vec::new();
        self.skip_newlines_and_comments();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
            self.skip_newlines_and_comments();
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
        &self.tokens[self.current].token
    }

    fn is_at_end(&self) -> bool {
        self.current_token() == &Token::EOF
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

