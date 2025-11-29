//! Abstract Syntax Tree (AST) structures and recursive descent parser for the Significance language.
//!
//! This module defines the AST node types that represent parsed Significance code and provides
//! a recursive descent parser that transforms token streams into an AST. The parser implements
//! proper operator precedence and handles uncertainty notation (`+/-`).
use crate::significance::tokenizer::{Token, TokenWithPos, Position};
use serde::{Serialize, Deserialize};

/// Root node of the Abstract Syntax Tree representing a complete Significance program.
///
/// A program consists of zero or more statements that are executed sequentially.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Represents a single statement in the Significance language.
///
/// Statements are the top-level constructs that can appear in a program. They include
/// variable declarations, assignments, and standalone expressions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Variable declaration: `{name : type}`
    ///
    /// Declares a new variable with the specified name and type. The variable is initialized
    /// to a default value (0.0 for Real types).
    VarDeclaration { name: String, var_type: VarType, pos: Position },
    
    /// Variable assignment: `name := expression`
    ///
    /// Assigns the result of an expression to an existing variable. The variable must be
    /// declared before it can be assigned.
    Assignment { name: String, value: Expression, pos: Position },
    
    /// Standalone expression statement
    ///
    /// An expression evaluated for its side effects or printed result. Common in REPL mode.
    Expression(Expression),
}

/// Variable type annotations in the Significance language.
///
/// Currently supports real numbers (with uncertainty) and function types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VarType {
    /// Real number type that can hold values with uncertainty (e.g., `12.3 +/- 0.5`)
    Real,
    
    /// Function type that operates on real numbers
    RealFunction
}

/// Represents an expression that can be evaluated to produce a value.
///
/// Expressions form the computational core of the language and support arithmetic operations,
/// uncertainty propagation, variables, and function calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Numeric literal with optional uncertainty: `value` or `value +/- error`
    ///
    /// Examples: `42`, `12.3 +/- 0.5`, `1.5e-3`
    NumberWithUncertainty { value: f64, error: f64, pos: Position },
    
    /// Variable reference by name
    ///
    /// References a previously declared variable to retrieve its current value.
    Variable(String),
    
    /// Binary operation: `left op right`
    ///
    /// Applies a binary operator to two sub-expressions. Operator precedence is handled
    /// during parsing (see parsing methods for precedence rules).
    Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression>, pos: Position },
    
    /// Unary operation: `op operand`
    ///
    /// Applies a unary operator (+ or -) as a prefix to an expression.
    Unary { op: UnaryOp, operand: Box<Expression>, pos: Position },
    
    /// Function call: `name(arg1, arg2, ...)`
    ///
    /// Invokes a function with the specified arguments. Functions are resolved from
    /// the standard library or user-defined functions.
    FunctionCall { name: String, args: Vec<Expression>, pos: Position},
}

/// Binary operators supported in the Significance language.
///
/// These operators are parsed with proper precedence levels:
/// - Level 1 (lowest): Add, Sub
/// - Level 2: Mul, Div, Mod
/// - Level 3 (highest): Power, Root
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    /// Addition: `+`
    Add,
    
    /// Subtraction: `-`
    Sub,
    
    /// Multiplication: `*`
    Mul,
    
    /// Division: `/`
    Div,
    
    /// Modulus: `%`
    Mod,
    
    /// Exponentiation: `**`
    ///
    /// Example: `2**3` evaluates to 8
    Power,
    
    /// Root extraction: `//`
    ///
    /// Example: `8//3` evaluates to 2 (cube root of 8)
    Root,
}

/// Unary operators that can be applied as prefixes to expressions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// Unary plus: `+expr`
    ///
    /// Identity operation (returns the value unchanged).
    Plus,
    
    /// Unary minus: `-expr`
    ///
    /// Negates the value (preserves uncertainty magnitude).
    Minus,
}

/// Error type for parsing failures.
///
/// Contains a descriptive message and the source position where the error occurred,
/// enabling helpful error reporting to users.
#[derive(Debug)]
pub struct ParseError {
    /// Human-readable description of what went wrong
    pub message: String,
    
    /// Location in the source where the error occurred
    pub position: Position,
}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        ParseError {
            message,
            position: Position { line: 0, column: 0 }
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}:{}: {}", self.position.line, self.position.column, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Recursive descent parser for the Significance language.
///
/// This parser transforms a token stream into an Abstract Syntax Tree (AST) using
/// recursive descent parsing with proper operator precedence. It maintains internal
/// state (current position in token stream) and provides methods for parsing different
/// grammatical constructs.
///
/// # Parsing Strategy
///
/// The parser uses recursive descent with the following precedence levels (highest to lowest):
/// 1. Primary (literals, variables, parentheses, function calls)
/// 2. Unary operators (+, -)
/// 3. Power/Root operators (**, //)
/// 4. Factor operators (*, /, %)
/// 5. Term operators (+, -)
///
/// # Example
///
/// ```ignore
/// let mut parser = AstParser::new();
/// let tokens = tokenizer.tokenize()?;
/// let program = parser.parse_program(tokens)?;
/// ```
pub struct AstParser {
    /// Token stream being parsed
    tokens: Vec<TokenWithPos>,
    
    /// Current position in the token stream (index into `tokens`)
    current: usize,
}

impl AstParser {
    /// Creates a new parser instance with an empty token stream.
    ///
    /// The parser must be initialized with tokens via `parse_program` or
    /// `parse_statement_from_tokens` before use.
    pub fn new() -> Self {
        Self { tokens:vec![], current: 0 }
    }
    
    /// Parses a complete program from a token stream.
    ///
    /// Processes all tokens until EOF is reached, collecting statements into a Program node.
    /// Automatically skips newlines and comments between statements.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Complete token stream including EOF token
    ///
    /// # Returns
    ///
    /// * `Ok(Program)` - Successfully parsed program
    /// * `Err(ParseError)` - Parse error with location and message
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Token stream is empty or missing EOF token
    /// - Any statement fails to parse
    /// - Unexpected tokens are encountered
    pub fn parse_program(&mut self, tokens: Vec<TokenWithPos>) -> Result<Program, ParseError> {
        self.tokens = tokens;
        
        //#NOTE: ensure EOF is present
        if self.tokens.is_empty() || !matches!(self.tokens.last().unwrap().token, Token::EOF) {
            return Err(ParseError { 
                message: "Token stream must end with EOF".to_string(),
                position: Position {line: 1, column: 1}
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
    
    /// Parses a single statement from the current token position.
    ///
    /// Determines the statement type based on the first token:
    /// - `{` → Variable declaration
    /// - Identifier followed by `:=` → Assignment
    /// - Anything else → Expression statement
    ///
    /// # Returns
    ///
    /// * `Ok(Statement)` - Successfully parsed statement
    /// * `Err(ParseError)` - Parse error with location
    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.current = 0;
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

    /// Parses a single statement from a provided token stream.
    ///
    /// Used primarily for REPL mode where statements are parsed individually.
    /// Initializes the parser with the given tokens and parses one statement.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Token stream for a single statement
    ///
    /// # Returns
    ///
    /// * `Ok(Statement)` - Successfully parsed statement
    /// * `Err(ParseError)` - Parse error with location
    pub fn parse_statement_from_tokens(&mut self, tokens: Vec<TokenWithPos>) -> Result<Statement, ParseError> {
        self.current = 0;
        self.tokens = tokens;
        self.parse_statement()
    }

    /// Parses a variable declaration: `{name : type}`.
    ///
    /// # Grammar
    ///
    /// ```text
    /// var_declaration := '{' identifier ':' type '}'
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Statement::VarDeclaration)` - Successfully parsed declaration
    /// * `Err(ParseError)` - Missing required tokens or invalid syntax
    fn parse_var_declaration(&mut self) -> Result<Statement, ParseError> {

        let pos = self.current_position();

        self.expect_token(Token::LeftBrace)?;

        let name = self.consume_identifier()?;
        
        self.expect_token(Token::Colon)?;

        let var_type = self.consume_var_type()?;

        self.expect_token(Token::RightBrace)?;

        Ok(Statement::VarDeclaration { name, var_type, pos })
        
    }

    /// Disambiguates statements starting with an identifier.
    ///
    /// Looks ahead to determine if this is an assignment (`:=` follows) or
    /// an expression statement (anything else follows).
    ///
    /// # Returns
    ///
    /// * `Ok(Statement::Assignment)` - If followed by `:=`
    /// * `Ok(Statement::Expression)` - Otherwise
    fn parse_starting_identifier(&mut self) -> Result<Statement, ParseError> {
        let peek_token = self.peek_token_n(1);
                
        match peek_token {
            Token::Assign => {
                self.parse_assignment()
            },
            _ => {
                Ok(Statement::Expression(self.parse_expression()?))
            }
        }
    }

    /// Parses an assignment statement: `name := expression`.
    ///
    /// # Grammar
    ///
    /// ```text
    /// assignment := identifier ':=' expression
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Statement::Assignment)` - Successfully parsed assignment
    /// * `Err(ParseError)` - Invalid syntax or missing tokens
    fn parse_assignment(&mut self) -> Result<Statement, ParseError> {

        let pos = self.current_position();

        let name = self.consume_identifier()?;

        self.expect_token(Token::Assign)?;

        let expression = self.parse_expression()?;

        Ok(Statement::Assignment { name, value: expression, pos })
    }
    
    /// Parses an expression with term-level operators (+ and -).
    ///
    /// This is the lowest precedence level for binary operators. Handles left-associative
    /// chains of addition and subtraction operations.
    ///
    /// # Grammar
    ///
    /// ```text
    /// expression := term (('+' | '-') term)*
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Expression)` - Successfully parsed expression
    /// * `Err(ParseError)` - Invalid syntax in sub-expressions
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {

        let pos = self.current_position();

        let mut left = self.parse_term()?;

        while let Some(op) = self.try_consume_term_operator() {

            let right = self.parse_term()?;

            left = Expression::Binary { 
                left: Box::new(left),
                op,
                right: Box::new(right),
                pos 
            };
        }

        Ok(left)

    }
    
    /// Parses a term with factor-level operators (*, /, %).
    ///
    /// Handles multiplication, division, and modulus operations with left associativity.
    /// Higher precedence than addition/subtraction, lower than power/root.
    ///
    /// # Grammar
    ///
    /// ```text
    /// term := factor (('*' | '/' | '%') factor)*
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Expression)` - Successfully parsed term
    /// * `Err(ParseError)` - Invalid syntax in sub-expressions
    fn parse_term(&mut self) -> Result<Expression, ParseError> {

        let pos = self.current_position();

        let mut left = self.parse_factor()?;

        while let Some(op) = self.try_consume_factor_operator() {

            let right = self.parse_factor()?;

            left = Expression::Binary { 
                left: Box::new(left),
                op,
                right: Box::new(right) , 
                pos
            };
        }

        Ok(left)
    }
    
    /// Parses a factor with power-level operators (** and //).
    ///
    /// Handles exponentiation and root operations. These have the highest precedence
    /// among binary operators and are right-associative.
    ///
    /// # Grammar
    ///
    /// ```text
    /// factor := unary (('**' | '//') factor)?
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Expression)` - Successfully parsed factor
    /// * `Err(ParseError)` - Invalid syntax in sub-expressions
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {

        let pos = self.current_position();

        let left = self.parse_unary()?;

        if let Some(op) = self.try_consume_power_operator() {

            let right = self.parse_factor()?;

            Ok(Expression::Binary { 
                left: Box::new(left),
                op,
                right: Box::new(right),
                pos
            })
        }
        else{
            Ok(left)
        }

    }
    
    /// Parses unary expressions with prefix operators (+ and -).
    ///
    /// Handles unary plus and minus operations. These operators bind more tightly
    /// than binary operators and can be chained (e.g., `--5` is valid).
    ///
    /// # Grammar
    ///
    /// ```text
    /// unary := ('+' | '-')* primary
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Expression)` - Successfully parsed unary expression or primary
    /// * `Err(ParseError)` - Invalid syntax in operand
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {

        let pos = self.current_position();

        match self.try_consume_unary_operator() {
            Some(op) => Ok(Expression::Unary { op, operand: Box::new(self.parse_unary()?), pos }),
            None => self.parse_primary(),
        }

    }
    
    /// Parses primary expressions (the highest precedence constructs).
    ///
    /// Primary expressions include:
    /// - Numeric literals (with optional uncertainty)
    /// - Variable references
    /// - Function calls
    /// - Parenthesized expressions
    ///
    /// # Grammar
    ///
    /// ```text
    /// primary := number ('+/-' number)?
    ///         | identifier '(' argument_list ')'
    ///         | identifier
    ///         | '(' expression ')'
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Expression)` - Successfully parsed primary expression
    /// * `Err(ParseError)` - Unexpected token or invalid syntax
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {

        let pos = self.current_position();

        let current = self.advance().clone();
        let next = self.peek_token();

        match (current, next) {
            (Token::LeftParen, _) => {
                let expr = self.parse_expression()?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            },
            (Token::Identifier(name), Token::LeftParen) => {
                self.parse_function_call(name, pos)
            }
            (Token::Identifier(name), _) => Ok(Expression::Variable(name.to_string())), 
            (Token::Number(n), Token::PlusMinus) => { 
                self.parse_number_with_uncertainty(n, pos) 
            },
            (Token::Number(n), _) => { 
                Ok(Expression::NumberWithUncertainty { value: n, error: 0.0, pos }) 
            },
            _ => Err(self.error("Expected expression")),
        }

    }

    /// Parses a number with uncertainty notation: `value +/- error`.
    ///
    /// Called when a number is followed by the `+/-` token. Expects another
    /// number token representing the uncertainty value.
    ///
    /// # Arguments
    ///
    /// * `number` - The base value
    /// * `pos` - Source position of the number
    ///
    /// # Returns
    ///
    /// * `Ok(Expression::NumberWithUncertainty)` - Successfully parsed
    /// * `Err(ParseError)` - Missing or invalid uncertainty value
    fn parse_number_with_uncertainty(&mut self, number: f64, pos: Position) -> Result<Expression, ParseError> {

        self.expect_token(Token::PlusMinus)?;
        match self.advance() {
            Token::Number(error) => {
                Ok(Expression::NumberWithUncertainty { value:number , error: *error, pos })
            },
            _ => {
                Err(self.error("Expected uncertainty number"))
            }
        }

    }

    /// Parses a function call: `name(arg1, arg2, ...)`.
    ///
    /// Expects parentheses enclosing a comma-separated list of argument expressions.
    ///
    /// # Arguments
    ///
    /// * `name` - Function name
    /// * `pos` - Source position of the function identifier
    ///
    /// # Returns
    ///
    /// * `Ok(Expression::FunctionCall)` - Successfully parsed call
    /// * `Err(ParseError)` - Missing parentheses or invalid arguments
    fn parse_function_call(&mut self, name: String, pos: Position) -> Result<Expression, ParseError> {

        self.expect_token(Token::LeftParen)?;
        let args = self.parse_argument_list()?;
        self.expect_token(Token::RightParen)?;
        Ok(Expression::FunctionCall { name, args, pos })
    }

    /// Parses a comma-separated list of function arguments.
    ///
    /// Handles empty argument lists (e.g., `func()`) and multiple arguments
    /// separated by commas (e.g., `func(a, b, c)`).
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Expression>)` - List of parsed argument expressions
    /// * `Err(ParseError)` - Invalid argument syntax
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
    
    /// Skips over newline tokens and comments.
    ///
    /// Called between statements to ignore whitespace and comments, which are
    /// not semantically significant in the language.
    fn skip_newlines_and_comments(&mut self) {
        while !self.is_at_end() && matches!(self.current_token(), Token::Newline | Token::Comment(_)) {
            self.advance();
        }
    }

    /// Returns a reference to the current token without advancing.
    fn current_token(&self) -> &Token {
        &self.tokens[self.current].token
    }

    /// Checks if we've reached the end of the token stream (EOF token).
    fn is_at_end(&self) -> bool {
        self.current_token() == &Token::EOF
    }
    
    /// Checks if the current token matches the expected token type.
    ///
    /// Uses discriminant comparison to match token types without comparing
    /// their associated values (e.g., all identifiers match regardless of name).
    fn current_token_is(&self, expected: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(expected)
    }
    
    /// Advances to the next token and returns a reference to the previous token.
    ///
    /// Does not advance past EOF.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }
    
    /// Returns a reference to the previous token.
    fn previous_token(&self) -> &Token {
        &self.tokens[self.current - 1].token
    }

    /// Peeks at the current token without advancing (same as `current_token`).
    ///
    /// Returns EOF if at end of stream.
    fn peek_token(&self) -> &Token {
        if self.is_at_end() {
            &Token::EOF
        } else {
            &self.tokens[self.current].token
        }
    }

    /// Peeks ahead by `n` tokens without advancing.
    ///
    /// Returns EOF if the lookahead goes beyond the token stream.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of positions to look ahead
    fn peek_token_n(&self, n: usize) -> &Token {
        if n + self.current < self.tokens.len() {
            return &self.tokens[n + self.current].token;
        } else {
            return &Token::EOF; 
        }
    }

    /// Attempts to consume a term-level operator (+ or -).
    ///
    /// If the current token is a term operator, advances and returns the operator.
    /// Otherwise returns None without advancing.
    ///
    /// # Returns
    ///
    /// * `Some(BinaryOp::Add)` - If `+` token consumed
    /// * `Some(BinaryOp::Sub)` - If `-` token consumed
    /// * `None` - If current token is not a term operator
    fn try_consume_term_operator(&mut self) -> Option<BinaryOp> {
        match self.current_token() {
            Token::Plus => { self.advance(); Some(BinaryOp::Add) },
            Token::Minus => { self.advance(); Some(BinaryOp::Sub) },
            _ => None,
        }
    }

    /// Attempts to consume a unary operator (+ or -).
    ///
    /// Similar to `try_consume_term_operator` but returns unary operators.
    ///
    /// # Returns
    ///
    /// * `Some(UnaryOp::Plus)` - If `+` token consumed
    /// * `Some(UnaryOp::Minus)` - If `-` token consumed
    /// * `None` - If current token is not a unary operator
    fn try_consume_unary_operator(&mut self) -> Option<UnaryOp> {
        match self.current_token() {
            Token::Plus => { self.advance(); Some(UnaryOp::Plus) },
            Token::Minus => { self.advance(); Some(UnaryOp::Minus) },
            _ => None,
        }
    }

    /// Attempts to consume a factor-level operator (*, /, %).
    ///
    /// # Returns
    ///
    /// * `Some(BinaryOp::Mul)` - If `*` token consumed
    /// * `Some(BinaryOp::Div)` - If `/` token consumed
    /// * `Some(BinaryOp::Mod)` - If `%` token consumed
    /// * `None` - If current token is not a factor operator
    fn try_consume_factor_operator(&mut self) -> Option<BinaryOp> {
        match self.current_token() {
            Token::Multiply => { self.advance(); Some(BinaryOp::Mul) },
            Token::Divide => { self.advance(); Some(BinaryOp::Div) },
            Token::Modulus => { self.advance(); Some(BinaryOp::Mod) },
            _ => None,
        }
    }

    /// Attempts to consume a power-level operator (** or //).
    ///
    /// # Returns
    ///
    /// * `Some(BinaryOp::Power)` - If `**` token consumed
    /// * `Some(BinaryOp::Root)` - If `//` token consumed
    /// * `None` - If current token is not a power operator
    fn try_consume_power_operator(&mut self) -> Option<BinaryOp> {
        match self.current_token() {
            Token::Power => { self.advance(); Some(BinaryOp::Power) },
            Token::Root => { self.advance(); Some(BinaryOp::Root) },
            _ => None,
        }
    }
    
    /// Returns the current source position for error reporting.
    fn current_position(&self) -> Position {
        self.tokens[self.current].position.clone()        
    }

    /// Consumes and validates a variable type token.
    ///
    /// # Returns
    ///
    /// * `Ok(VarType)` - Successfully consumed type
    /// * `Err(ParseError)` - Current token is not a valid type
    fn consume_var_type(&mut self) -> Result<VarType, ParseError> {
        let token = self.advance();
        match token {
            Token::Real => Ok(VarType::Real),
            _ => Err(self.error("Expected variable type")),
        }
    }

    /// Consumes and returns an identifier token.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Identifier name
    /// * `Err(ParseError)` - Current token is not an identifier
    fn consume_identifier(&mut self) -> Result<String, ParseError> {
        match self.current_token() {
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                Ok(name)
            },
            _ => Err(self.error("Expected identifier")),
        }
    }

    /// Expects a specific token type and advances if found.
    ///
    /// # Arguments
    ///
    /// * `expected` - The token type that must be present
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Token matched and consumed
    /// * `Err(ParseError)` - Token did not match
    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token_is(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("Expected {:?}", expected)))
        }
    }

    /// Creates a ParseError with a custom message at the current position.
    ///
    /// # Arguments
    ///
    /// * `message` - Error description
    ///
    /// # Returns
    ///
    /// * `ParseError` - Error object with current position
    fn error(&self, message: &str) -> ParseError {
        ParseError { 
            message: message.to_string(), 
            position: self.current_position() 
        }
    }
}