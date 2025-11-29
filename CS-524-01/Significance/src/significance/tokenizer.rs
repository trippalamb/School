//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

//! Lexical analysis (tokenization) for the Significance language.
//!
//! This module provides a tokenizer that converts raw source text into a stream of tokens.
//! The tokenizer handles numeric literals (including scientific notation and uncertainty syntax),
//! identifiers, operators, keywords, comments, and position tracking for error reporting.

use std::fmt;
use serde::{Serialize, Deserialize};

/// All possible tokens in the Significance language.
///
/// Tokens represent the smallest meaningful units of source code. Each token carries its
/// semantic meaning and any associated data (like the value of a number or the name of an
/// identifier).
///
/// # Token Categories
///
/// - **Literals**: Numbers and identifiers
/// - **Keywords**: Language keywords like `real`
/// - **Operators**: Arithmetic and special operators
/// - **Delimiters**: Parentheses, braces, commas
/// - **Special**: Comments, newlines, EOF
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    /// Numeric literal: integer, floating-point, or scientific notation
    ///
    /// Examples: `42`, `3.14`, `1.5e-3`, `6.022e23`
    Number(f64),
    
    /// Identifier: variable or function name
    ///
    /// Must start with letter or underscore, can contain letters, digits, underscores.
    /// Examples: `x`, `my_var`, `temp2`, `_internal`
    Identifier(String),
    
    // Keywords/Types
    /// The `real` keyword used in type declarations
    ///
    /// Example: `{x : real}`
    Real,
    
    // Operators
    /// Addition operator: `+`
    Plus,
    
    /// Subtraction operator: `-`
    Minus,
    
    /// Multiplication operator: `*`
    Multiply,
    
    /// Division operator: `/`
    Divide,
    
    /// Modulus operator: `%`
    Modulus,
    
    /// Exponentiation operator: `**`
    ///
    /// Example: `2**3` equals 8
    Power,
    
    /// Root operator: `//`
    ///
    /// Example: `8//3` equals 2 (cube root)
    Root,
    
    /// Uncertainty operator: `+/-`
    ///
    /// Used to specify measurement uncertainty.
    /// Example: `12.3 +/- 0.5`
    PlusMinus,
    
    // Assignment and Declaration
    /// Assignment operator: `:=`
    ///
    /// Example: `x := 5.0`
    Assign,
    
    /// Colon: `:`
    ///
    /// Used in type declarations between identifier and type.
    /// Example: `{x : real}`
    Colon,
    
    // Delimiters
    /// Left parenthesis: `(`
    LeftParen,
    
    /// Right parenthesis: `)`
    RightParen,
    
    /// Left brace: `{`
    LeftBrace,
    
    /// Right brace: `}`
    RightBrace,
    
    /// Comma: `,`
    ///
    /// Used to separate function arguments.
    Comma,
    
    // Special
    /// Comment: `# comment text`
    ///
    /// Comments start with `#` and continue to end of line.
    /// The comment text does not include the `#` character.
    Comment(String),
    
    /// Newline character
    ///
    /// Used to separate statements. Multiple consecutive newlines are preserved.
    Newline,
    
    /// End of file marker
    ///
    /// Signals the end of the token stream. Always the last token.
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "NUMBER({})", n),
            Token::Identifier(s) => write!(f, "ID({})", s),
            Token::Real => write!(f, "REAL"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulus => write!(f, "%"),
            Token::Power => write!(f, "**"),
            Token::Root => write!(f, "//"),
            Token::PlusMinus => write!(f, "+/-"),
            Token::Assign => write!(f, ":="),
            Token::Colon => write!(f, ":"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Comment(s) => write!(f, "COMMENT({})", s),
            Token::Newline => write!(f, "NEWLINE"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

/// Source code position for error reporting and debugging.
///
/// Tracks the line and column number of tokens in the source text.
/// Line and column numbers are 1-indexed (first line is line 1, first column is column 1).
///
/// # Examples
///
/// ```
/// # use significance::Position;
/// let pos = Position { line: 5, column: 12 };
/// // Refers to line 5, column 12 in the source file
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// Line number in source (1-indexed)
    pub line: usize,
    
    /// Column number in source (1-indexed)
    pub column: usize,
}

/// A token paired with its source position.
///
/// This structure combines a token with position information, enabling precise
/// error reporting and debugging. All tokens produced by the tokenizer are wrapped
/// in this structure.
#[derive(Debug, Clone)]
pub struct TokenWithPos {
    /// The token itself
    pub token: Token,
    
    /// Where this token appears in the source
    pub position: Position,
}

/// Lexical analyzer (tokenizer) for the Significance language.
///
/// The tokenizer converts raw source text into a stream of tokens that can be
/// consumed by the parser. It handles:
/// - Numeric literals with scientific notation
/// - Multi-character operators (`**`, `//`, `+/-`, `:=`)
/// - Identifiers and keywords
/// - Comments (ignored by parser but preserved in token stream)
/// - Position tracking for error reporting
///
/// # Implementation Details
///
/// The tokenizer uses a simple character-by-character scanning approach with
/// single-character lookahead for disambiguation (e.g., distinguishing `*` from `**`).
/// Whitespace (spaces, tabs, carriage returns) is skipped automatically, while
/// newlines are preserved as tokens since they're significant for statement separation.
///
/// # Example
///
/// ```ignore
/// let source = "x := 12.3 +/- 0.5";
/// let mut tokenizer = Tokenizer::new(source);
/// let tokens = tokenizer.tokenize()?;
/// // tokens contains: Identifier("x"), Assign, Number(12.3), PlusMinus, Number(0.5), EOF
/// ```
pub struct Tokenizer {
    /// Input characters as a vector for random access
    input: Vec<char>,
    
    /// Current position in the input (index into `input`)
    current: usize,
    
    /// Current line number (1-indexed)
    line: usize,
    
    /// Current column number (1-indexed)
    column: usize,
}

impl Tokenizer {
    /// Creates a new tokenizer for the given input string.
    ///
    /// The input is converted to a vector of characters to support efficient
    /// random access during tokenization.
    ///
    /// # Arguments
    ///
    /// * `input` - Source code to tokenize
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tokenizer = Tokenizer::new("x := 5");
    /// ```
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Scans and returns the next token from the input.
    ///
    /// This method performs the core tokenization logic, identifying token boundaries
    /// and categorizing characters into appropriate token types. It handles:
    /// - Single-character tokens (e.g., `+`, `(`, `,`)
    /// - Multi-character tokens (e.g., `**`, `+/-`, `:=`)
    /// - Numeric literals with decimal points and scientific notation
    /// - Identifiers and keywords
    /// - Comments
    ///
    /// # Returns
    ///
    /// * `Ok(TokenWithPos)` - Successfully recognized token with position
    /// * `Err(String)` - Error message for unexpected or invalid characters
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An unexpected character is encountered
    /// - A number cannot be parsed as `f64`
    pub fn next_token(&mut self) -> Result<TokenWithPos, String> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(TokenWithPos {
                token: Token::EOF,
                position: self.current_position(),
            });
        }
        
        let start_pos = self.current_position();
        let ch = self.advance();
        
        match ch {
            // Single character tokens with potential multi-character lookahead
            '+' => {
                if self.match_char('/') && self.match_char('-') {
                    Ok(TokenWithPos { token: Token::PlusMinus, position: start_pos })
                } else {
                    Ok(TokenWithPos { token: Token::Plus, position: start_pos })
                }
            },
            '-' => Ok(TokenWithPos { token: Token::Minus, position: start_pos }),
            '*' => {
                if self.match_char('*') {
                    Ok(TokenWithPos { token: Token::Power, position: start_pos })
                } else {
                    Ok(TokenWithPos { token: Token::Multiply, position: start_pos })
                }
            },
            '/' => {
                if self.match_char('/') {
                    Ok(TokenWithPos { token: Token::Root, position: start_pos })
                } else {
                    Ok(TokenWithPos { token: Token::Divide, position: start_pos })
                }
            },
            '%' => Ok(TokenWithPos { token: Token::Modulus, position: start_pos }),
            '(' => Ok(TokenWithPos { token: Token::LeftParen, position: start_pos }),
            ')' => Ok(TokenWithPos { token: Token::RightParen, position: start_pos }),
            '{' => Ok(TokenWithPos { token: Token::LeftBrace, position: start_pos }),
            '}' => Ok(TokenWithPos { token: Token::RightBrace, position: start_pos }),
            ',' => Ok(TokenWithPos { token: Token::Comma, position: start_pos }),
            ':' => {
                if self.match_char('=') {
                    Ok(TokenWithPos { token: Token::Assign, position: start_pos })
                } else {
                    Ok(TokenWithPos { token: Token::Colon, position: start_pos })
                }
            },
            '#' => {
                let comment_text = self.read_comment();
                Ok(TokenWithPos { token: Token::Comment(comment_text), position: start_pos })
            },
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(TokenWithPos { token: Token::Newline, position: start_pos })
            },
            
            // Numbers (including scientific notation)
            '0'..='9' => {
                let number = self.read_number(ch)?;
                Ok(TokenWithPos { token: Token::Number(number), position: start_pos })
            },
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier(ch);
                let token = match identifier.as_str() {
                    "real" => Token::Real,
                    _ => Token::Identifier(identifier),
                };
                Ok(TokenWithPos { token, position: start_pos })
            },
            
            _ => Err(format!("Unexpected character '{}' at {}:{}", ch, self.line, self.column)),
        }
    }
    
    /// Tokenizes the entire input into a vector of tokens.
    ///
    /// Repeatedly calls `next_token()` until EOF is reached, collecting all tokens
    /// into a vector. This is the primary method for converting source code into
    /// a token stream for parsing.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<TokenWithPos>)` - Complete token stream including EOF
    /// * `Err(String)` - Error message if tokenization fails
    ///
    /// # Errors
    ///
    /// Returns an error if `next_token()` encounters invalid input.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut tokenizer = Tokenizer::new("{x : real}\nx := 42");
    /// let tokens = tokenizer.tokenize()?;
    /// assert!(matches!(tokens.last().unwrap().token, Token::EOF));
    /// ```
    pub fn tokenize(&mut self) -> Result<Vec<TokenWithPos>, String> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.token, Token::EOF);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
    
    /// Skips whitespace characters (space, tab, carriage return).
    ///
    /// Newlines are not skipped since they're significant for statement separation.
    /// This method is called before attempting to recognize each token.
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    /// Peeks at the current character without advancing.
    ///
    /// Returns the null character `\0` if at end of input.
    ///
    /// # Returns
    ///
    /// Current character or `\0` if at end
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.current]
        }
    }
    
    /// Checks if we've reached the end of the input.
    ///
    /// # Returns
    ///
    /// `true` if at or past the end of input, `false` otherwise
    fn is_at_end(&self) -> bool { 
        self.current >= self.input.len() 
    }

    /// Returns the current source position.
    ///
    /// Used to tag tokens with their location for error reporting.
    ///
    /// # Returns
    ///
    /// Current position with line and column numbers
    fn current_position(&self) -> Position { 
        Position { line: self.line, column: self.column }
    }
    
    /// Consumes and returns the current character, advancing position.
    ///
    /// Updates both the input position and column number. Returns `\0` if
    /// called at end of input.
    ///
    /// # Returns
    ///
    /// The consumed character or `\0` if at end
    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            let ch = self.input[self.current];
            self.current += 1;
            self.column += 1;
            ch
        } else {
            '\0'
        }
    }

    /// Attempts to match and consume an expected character.
    ///
    /// Used for lookahead to distinguish multi-character tokens (e.g., `*` vs `**`).
    /// If the current character matches, it's consumed and the method returns `true`.
    /// Otherwise, no advancement occurs and it returns `false`.
    ///
    /// # Arguments
    ///
    /// * `expected` - The character to match
    ///
    /// # Returns
    ///
    /// `true` if matched and consumed, `false` otherwise
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }
    
    /// Reads a comment from `#` to end of line.
    ///
    /// Assumes the `#` character has already been consumed. Reads all remaining
    /// characters on the line but does not include the newline character itself.
    ///
    /// # Returns
    ///
    /// The comment text without the leading `#` or trailing newline
    fn read_comment(&mut self) -> String { 
        
        let mut comment_text = String::new();
        while !self.is_at_end() && self.peek() != '\n' {
            comment_text.push(self.advance());
        }
        comment_text

    }

    /// Reads a complete numeric literal.
    ///
    /// Handles integers, floating-point numbers, and scientific notation.
    /// Supports formats like: `42`, `3.14`, `1.5e-3`, `6.022e+23`
    ///
    /// # Grammar
    ///
    /// ```text
    /// number := integer_part ('.' integer_part)? (('e'|'E') ('+'|'-')? integer_part)?
    /// ```
    ///
    /// # Arguments
    ///
    /// * `first_char` - The first digit (already consumed)
    ///
    /// # Returns
    ///
    /// * `Ok(f64)` - Successfully parsed number
    /// * `Err(String)` - Invalid number format
    ///
    /// # Errors
    ///
    /// Returns an error if the collected string cannot be parsed as `f64`.
    fn read_number(&mut self, first_char: char) -> Result<f64, String> { 

        let mut number_text = String::new();
        number_text.push(first_char);
            
        self.read_integer_part(&mut number_text);
        self.read_decimal_part(&mut number_text);
        self.read_exponent_part(&mut number_text);


        number_text.parse::<f64>()
            .map_err(|_| format!("Could not parse '{}' as floating point number", number_text))

    }

    /// Reads the decimal portion of a number (if present).
    ///
    /// Checks for a `.` and if found, consumes it along with the following digits.
    /// Updates the `number_text` string with the decimal point and fractional part.
    ///
    /// # Arguments
    ///
    /// * `number_text` - String accumulator for the number being constructed
    fn read_decimal_part(&mut self, number_text: &mut String){
        let cp = self.peek();
        if cp == '.'{
            number_text.push(self.advance());
            self.read_integer_part(number_text);
        }
    }

    /// Reads the exponent portion of a number (if present).
    ///
    /// Checks for `e` or `E`, and if found, consumes it along with an optional sign
    /// (`+` or `-`) and the exponent digits.
    ///
    /// # Arguments
    ///
    /// * `number_text` - String accumulator for the number being constructed
    fn read_exponent_part(&mut self, number_text: &mut String){
        let cp = self.peek();
        if matches!(cp, 'e' | 'E') {
            number_text.push(self.advance());
            let cp = self.peek();
            if matches!(cp, '+'|'-') {
                number_text.push(self.advance());
            }
            self.read_integer_part(number_text);

        }
    }

    /// Reads a sequence of digits.
    ///
    /// Consumes consecutive digit characters (`0-9`) and appends them to the
    /// provided string. Stops at the first non-digit character.
    ///
    /// # Arguments
    ///
    /// * `number_text` - String accumulator for digits
    fn read_integer_part(&mut self, number_text: &mut String){
        while !self.is_at_end() {
            match self.peek(){
                '0'..='9' => {
                    let c = self.advance();
                    number_text.push(c);
                },
                _ =>{
                    break;
                }
            }
        }
    }

    /// Reads a complete identifier or keyword.
    ///
    /// Identifiers must start with a letter or underscore, and can contain letters,
    /// digits, and underscores. The caller is responsible for matching keywords
    /// (e.g., `real`) after the identifier is read.
    ///
    /// # Grammar
    ///
    /// ```text
    /// identifier := (letter | '_') (letter | digit | '_')*
    /// ```
    ///
    /// # Arguments
    ///
    /// * `first_char` - The first character (already consumed)
    ///
    /// # Returns
    ///
    /// The complete identifier string
    fn read_identifier(&mut self, first_char: char) -> String {

        let mut identifier_text = String::new();
        identifier_text.push(first_char);

        while !self.is_at_end() {
            match self.peek(){
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    let c = self.advance();
                    identifier_text.push(c);
                },
                _ =>{
                    break;
                }
            }
        }

        identifier_text
    }

}