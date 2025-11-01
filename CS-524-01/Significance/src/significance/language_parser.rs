use std::fs::{self, File};
use crate::significance::tokenizer::{Tokenizer, Token, TokenWithPos};
use crate::significance::ast_parser::{AstParser, ParseError, Program};
use crate::significance::semantic_analyzer::SemanticAnalyzer;
use crate::significance::executor::Executor;
use crate::significance::RunTimeError;


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

    pub fn parse_repl(mut self, input: &str) -> Result<String, ParseError> {

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

        // Check for runtime errors
        let errors = self.executor.get_errors();
        if let Some(first_error) = errors.first() {
            let (message, position) = match first_error {
                RunTimeError::DivisionByZero(pos) => 
                    ("Division by zero".to_string(), pos.clone()),
                RunTimeError::UndefinedVariable(name, pos) => 
                    (format!("Undefined variable: {}", name), pos.clone()),
            };
            return Err(ParseError { message, position });
        }

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
        let contents = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        
        let mut tokenizer = Tokenizer::new(contents.as_str());
        let tokens = tokenizer.tokenize()?;

        let ast = AstParser::new().parse_program(tokens).map_err(|e| format!("Failed to parse file '{}': {}", filename, e))?;

        write_ast_to_file(&ast, "ast.json").map_err(|e| format!("Failed to write AST to file: {}", e))?;

        SemanticAnalyzer::new().analyze_program(&ast);
        Executor::new().execute_program(&ast);
        
        Ok(0) // placeholder
    }

    
    /// Parse and evaluate a string containing Significance language code
    pub fn parse_string(_input: &str) -> Result<i32, String> {
        // TODO: Implement actual parsing and evaluation
        unimplemented!("String parsing not yet implemented")
    }
}


fn write_ast_to_file(program: &Program, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, program)?;
    Ok(())
}