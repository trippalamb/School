use std::fs;
use crate::significance::tokenizer::{Tokenizer, Token, TokenWithPos};
use crate::significance::ast_parser::{AstParser, ParseError};
use crate::significance::semantic_analyzer::SemanticAnalyzer;
use crate::significance::executor::Executor;


/// Main parser and evaluator for the Significance language
pub struct Significance{
    parser: AstParser,
    analyzer: SemanticAnalyzer,
    executor: Executor
}

impl Significance {

    pub fn new() -> Self {
        Self {
            parser: AstParser::new(),
            analyzer: SemanticAnalyzer::new(),
            executor: Executor::new()
        }
    }

    pub fn parse_repl(mut self,input: &str) -> Result<String, ParseError> {

        let input = input.trim();
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;

        // Find the last meaningful token (skip Newline and EOF)
        let last_meaningful = tokens.iter()
            .rev()
            .find(|t| !matches!(t.token, Token::Newline | Token::EOF));

        match last_meaningful {
            Some(TokenWithPos { token, .. }) if Self::is_incomplete_token(token) => {
                return Ok("".to_string()) // Signal continuation needed
            },
            _ => (),
        }

        let ast = self.parser.parse_statement_from_tokens(tokens)?;
        self.analyzer.analyze_statement(&ast);
        self.executor.execute_statement(&ast);

        Ok("".to_string())
    }

    fn is_incomplete_token(token: &Token) -> bool {
        matches!(token, 
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | 
            Token::Modulus | Token::Power | Token::Root |
            Token::Assign | Token::Colon |
            Token::LeftParen | Token::LeftBrace |
            Token::Comma
        )
    }

    /// Parse and evaluate a file containing Significance language code
    pub fn parse_file(filename: &str) -> Result<i32, String> {
        let _contents = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        
        
        // TODO: Implement actual parsing and evaluation
        
        Ok(0) // placeholder
    }
    
    /// Parse and evaluate a string containing Significance language code
    pub fn parse_string(_input: &str) -> Result<i32, String> {
        // TODO: Implement actual parsing and evaluation
        unimplemented!("String parsing not yet implemented")
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file_stub() {
        // This should work but just return 0
        // We'll test with a non-existent file to test error handling
        match Significance::parse_file("nonexistent.sig") {
            Err(_) => assert!(true), // Expected to fail for non-existent file
            Ok(result) => assert_eq!(result, 0),
        }
    }

    #[test]
    #[should_panic(expected = "String parsing not yet implemented")]
    fn test_parse_string_stub() {
        let _result = Significance::parse_string("x = 5 + 3");
    }
}