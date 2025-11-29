//! High-level interface for the Significance language interpreter.
//!
//! This module provides the main `Significance` struct that orchestrates the complete
//! interpretation pipeline: tokenization → parsing → semantic analysis → execution.
//! It offers two primary modes of operation:
//!
//! - **REPL mode**: Interactive statement-by-statement execution with state persistence
//! - **File mode**: Batch execution of complete programs from source files
//!
//! # Pipeline Overview
//!
//! ```text
//! Source Code
//!     ↓
//! Tokenizer → Token Stream
//!     ↓
//! Parser → Abstract Syntax Tree (AST)
//!     ↓
//! Semantic Analyzer → Validated AST
//!     ↓
//! Executor → Results with Uncertainty
//! ```
//!
//! # Example Usage
//!
//! ```ignore
//! // REPL mode
//! let mut sig = Significance::new();
//! sig.parse_repl("{x : real}")?;
//! sig.parse_repl("x := 12.3 +/- 0.5")?;
//! sig.parse_repl("x * 2")?;  // Prints: 24.6 ± 1.0
//!
//! // File mode
//! Significance::parse_file("program.sig")?;
//! ```
//! 
use std::fs::{self, File};
use crate::significance::tokenizer::{Tokenizer, Token, TokenWithPos};
use crate::significance::ast_parser::{AstParser, Program};
use crate::significance::semantic_analyzer::SemanticAnalyzer;
use crate::significance::executor::Executor;

/// Main interpreter interface for the Significance language.
///
/// This struct encapsulates all the components needed to interpret Significance code,
/// maintaining state across operations to support REPL workflows. It coordinates the
/// complete interpretation pipeline from source text to execution results.
///
/// # Components
///
/// - **Parser**: Converts token streams into ASTs
/// - **Analyzer**: Validates ASTs for semantic correctness
/// - **Executor**: Evaluates validated ASTs with uncertainty propagation
///
/// # State Persistence
///
/// The `Significance` instance maintains state across `parse_repl` calls, enabling:
/// - Variable declarations to persist across multiple inputs
/// - Interactive exploration and debugging
/// - Incremental program development
///
/// # Design Philosophy
///
/// This is a simple educational interpreter that prioritizes:
/// - Clear, understandable implementation over performance
/// - Transparent error reporting at each pipeline stage
/// - Conservative uncertainty estimation over complex simulation
///
/// # Example
///
/// ```ignore
/// let mut interpreter = Significance::new();
///
/// // Declare variables
/// interpreter.parse_repl("{mass : real}")?;
/// interpreter.parse_repl("{velocity : real}")?;
///
/// // Assign values with uncertainty
/// interpreter.parse_repl("mass := 10.5 +/- 0.1")?;
/// interpreter.parse_repl("velocity := 3.0 +/- 0.05")?;
///
/// // Calculate with automatic uncertainty propagation
/// interpreter.parse_repl("mass * velocity")?;  // kinetic energy
/// ```
pub struct Significance{
    /// AST parser for converting token streams into syntax trees
    parser: AstParser,
    
    /// Semantic analyzer for validating ASTs before execution
    analyzer: SemanticAnalyzer,
    
    /// Runtime executor that maintains variable state and performs calculations
    executor: Executor
}

impl Significance {

    /// Creates a new Significance interpreter with empty state.
    ///
    /// Initializes all pipeline components with no variables, functions, or errors.
    /// The interpreter is ready to process statements immediately after creation.
    ///
    /// # Returns
    ///
    /// A new `Significance` instance ready for REPL or file execution
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut sig = Significance::new();
    /// sig.parse_repl("2 + 2")?;  // Prints: 4
    /// ```
    pub fn new() -> Self {
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.import_standard_library();
        
        Self {
            parser: AstParser::new(),
            analyzer,
            executor: Executor::new()
        }
    }

    /// Parses and executes a single statement in REPL mode.
    ///
    /// Processes one line of input through the complete interpretation pipeline,
    /// maintaining state for subsequent calls. Handles incomplete statements by
    /// returning success without execution, allowing multi-line input in REPL environments.
    ///
    /// # REPL Features
    ///
    /// - **State Persistence**: Variables and functions persist across calls
    /// - **Incomplete Detection**: Recognizes when input ends with an operator or delimiter
    /// - **Error Recovery**: Non-fatal errors allow the REPL session to continue
    /// - **Immediate Feedback**: Expression statements print their results
    ///
    /// # Arguments
    ///
    /// * `input` - A single line or statement of Significance code
    ///
    /// # Returns
    ///
    /// * `Ok("")` - Statement executed successfully (or detected as incomplete)
    /// * `Err(ParseError)` - Syntax, semantic, or runtime error with position
    ///
    /// # Errors
    ///
    /// Returns an error for:
    /// - Invalid syntax during tokenization or parsing
    /// - Semantic errors (undeclared variables, duplicate declarations)
    /// - Runtime errors (division by zero, undefined variables that escaped semantic analysis)
    ///
    /// # Incomplete Statements
    ///
    /// Input ending with operators (`+`, `-`, `*`, etc.) or opening delimiters (`(`, `{`)
    /// is considered incomplete and returns `Ok("")` without execution. This enables
    /// multi-line input patterns in REPL interfaces.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut sig = Significance::new();
    ///
    /// // Declaration persists
    /// sig.parse_repl("{x : real}")?;
    ///
    /// // Assignment modifies state
    /// sig.parse_repl("x := 5.0")?;
    ///
    /// // Expression uses previous state
    /// sig.parse_repl("x + 10")?;  // Prints: 15
    ///
    /// // Incomplete statement (no execution)
    /// assert_eq!(sig.parse_repl("x +")?, "");
    /// ```
    pub fn parse_repl(&mut self, input: &str) -> Result<Vec<String>, std::io::Error> {

        self.analyzer.clear_errors();
        self.executor.clear_errors();

        let mut errors = Vec::new();
        
        let input = input.trim();
        let mut tokenizer = Tokenizer::new(input);
        
        let tokens = match tokenizer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);  // tokenize already returns String
                return Ok(errors);
            }
        };

        // Find the last meaningful token (skip Newline and EOF)
        let last_meaningful = tokens.iter()
            .rev()
            .find(|t| !matches!(t.token, Token::Newline | Token::EOF));

        match last_meaningful {
            Some(TokenWithPos { token, .. }) if Self::is_incomplete_token(token) => {
                return Ok(Vec::new()) // Signal continuation needed
            },
            _ => (),
        }

        let ast = match self.parser.parse_statement_from_tokens(tokens) {
            Ok(a) => a,
            Err(e) => {
                errors.push(e.to_string());
                return Ok(errors);
            }
        };

        self.analyzer.analyze_statement(&ast);
        errors.extend(self.analyzer.get_errors().iter().map(|e| e.to_string()));
        
        if errors.is_empty() {
            self.executor.execute_statement(&ast);
            errors.extend(self.executor.get_errors().iter().map(|e| e.to_string()));
        }

        Ok(errors)
    }

    /// Checks if a token indicates an incomplete statement.
    ///
    /// Used by REPL mode to detect when input requires continuation. Statements
    /// ending with binary operators, assignment operators, or opening delimiters
    /// are considered incomplete.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to check
    ///
    /// # Returns
    ///
    /// `true` if the token indicates the statement is incomplete, `false` otherwise
    ///
    /// # Incomplete Tokens
    ///
    /// - Arithmetic operators: `+`, `-`, `*`, `/`, `%`, `**`, `//`
    /// - Assignment: `:=`
    /// - Colon: `:`
    /// - Opening delimiters: `(`, `{`
    /// - Comma: `,`
    ///
    /// # Example
    ///
    /// ```ignore
    /// // These would be considered incomplete:
    /// "x +"       // ends with Plus
    /// "func("     // ends with LeftParen
    /// "{x : "     // ends with Colon
    /// "a, "       // ends with Comma
    /// ```
    fn is_incomplete_token(token: &Token) -> bool {
        matches!(token, 
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | 
            Token::Modulus | Token::Power | Token::Root |
            Token::Assign | Token::Colon |
            Token::LeftParen | Token::LeftBrace |
            Token::Comma
        )
    }


    /// Parses and executes a complete program from a source file.
    ///
    /// Reads a `.sig` file, processes it through the complete interpretation pipeline,
    /// and optionally writes the AST to a JSON file for debugging. This is the batch
    /// execution mode for running complete programs.
    ///
    /// # Pipeline Steps
    ///
    /// 1. Read source file to string
    /// 2. Tokenize the complete input
    /// 3. Parse tokens into AST
    /// 4. Write AST to `ast.json` (for debugging/inspection)
    /// 5. Perform semantic analysis
    /// 6. Execute the validated program
    ///
    /// # Arguments
    ///
    /// * `filename` - Path to the source file (typically `.sig` extension)
    ///
    /// # Returns
    ///
    /// * `Ok(0)` - Program executed successfully
    /// * `Err(String)` - Error message describing what went wrong
    ///
    /// # Errors
    ///
    /// Returns an error for:
    /// - File I/O errors (file not found, permission denied, etc.)
    /// - Tokenization errors (invalid characters, malformed numbers)
    /// - Parse errors (syntax violations)
    /// - AST write errors (cannot create `ast.json`)
    ///
    /// # Side Effects
    ///
    /// - Creates/overwrites `ast.json` in the current directory with the parsed AST
    /// - Prints output from expression statements to stdout
    /// - May print error messages if semantic or runtime errors occur
    ///
    /// # Note
    ///
    /// Unlike `parse_repl`, this method does not maintain state. Each file is executed
    /// in a fresh interpreter instance. Runtime and semantic errors are not currently
    /// surfaced in the return value (only parse errors are).
    ///
    /// # Example
    ///
    /// ```ignore
    /// // program.sig:
    /// // {x : real}
    /// // {y : real}
    /// // x := 12.3 +/- 0.5
    /// // y := 2.6 +/- 0.2
    /// // x + y
    ///
    /// Significance::parse_file("program.sig")?;
    /// // Prints: 14.9 ± 0.5385164807134504
    /// // Creates: ast.json
    /// ```
    pub fn parse_file(filename: &str) -> Result<i32, String> {
        let contents = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        
        let mut tokenizer = Tokenizer::new(contents.as_str());
        let tokens = tokenizer.tokenize()?;

        let ast = AstParser::new().parse_program(tokens)
            .map_err(|e| format!("Failed to parse file '{}': {}", filename, e))?;

        write_ast_to_file(&ast, "ast.json")
            .map_err(|e| format!("Failed to write AST to file: {}", e))?;

        SemanticAnalyzer::new().import_standard_library().analyze_program(&ast);
        Executor::new().execute_program(&ast);
        
        Ok(0) // placeholder return value
    }

}

/// Writes a parsed AST to a JSON file for inspection and debugging.
///
/// Serializes the complete program AST using `serde_json` with pretty-printing
/// enabled, making it easy to examine the parsed structure of Significance programs.
///
/// # Arguments
///
/// * `program` - The AST to serialize
/// * `filename` - Path where the JSON file should be written
///
/// # Returns
///
/// * `Ok(())` - File written successfully
/// * `Err(Box<dyn Error>)` - I/O or serialization error
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be created (permission denied, invalid path, etc.)
/// - Serialization fails (should not happen with valid AST)
///
/// # Format
///
/// The output JSON includes:
/// - Complete statement list with types and positions
/// - Expression trees with operator precedence preserved
/// - All source position information for debugging
///
/// ```
fn write_ast_to_file(program: &Program, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, program)?;
    Ok(())
}