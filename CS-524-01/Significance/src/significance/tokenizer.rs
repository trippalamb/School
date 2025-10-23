use std::fmt;
use serde::{Serialize, Deserialize};

/// All possible tokens in the Significance language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    Identifier(String),
    
    // Keywords/Types
    Real,
    
    // Operators
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Modulus,        // %
    Power,          // **
    Root,           // //
    PlusMinus,      // +/-
    
    // Assignment and Declaration
    Assign,         // :=
    Colon,          // :
    
    // Delimiters
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    Comma,          // ,
    
    // Special
    Comment(String), // # comment text
    Newline,
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

/// Position information for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Token with position information
#[derive(Debug, Clone)]
pub struct TokenWithPos {
    pub token: Token,
    pub position: Position,
}

/// Tokenizer for the Significance language
pub struct Tokenizer {
    input: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Get the next token from the input
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
            // Single character tokens
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
    
    /// Tokenize the entire input into a vector of tokens
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.current]
        }
    }
    fn is_at_end(&self) -> bool { 
        self.current >= self.input.len() 
    }

    fn current_position(&self) -> Position { 
        Position { line: self.line, column: self.column }
    }
    
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

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }
    
    fn read_comment(&mut self) -> String { 
        
        let mut comment_text = String::new();
        while !self.is_at_end() && self.peek() != '\n' {
            comment_text.push(self.advance());
        }
        comment_text

    }

    fn read_number(&mut self, first_char: char) -> Result<f64, String> { 

        let mut number_text = String::new();
        number_text.push(first_char);
            
        self.read_integer_part(&mut number_text);
        self.read_decimal_part(&mut number_text);
        self.read_exponent_part(&mut number_text);


        number_text.parse::<f64>()
            .map_err(|_| format!("Could not parse '{}' as floating point number", number_text))

    }

    fn read_decimal_part(&mut self, number_text: &mut String){
        let cp = self.peek();
        if cp == '.'{
            number_text.push(self.advance());
            self.read_integer_part(number_text);
        }
    }

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

