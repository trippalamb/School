//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

//! Semantic analysis for the Significance language.
//!
//! This module performs semantic validation of parsed AST nodes, checking for errors that
//! cannot be detected during parsing such as:
//! - Use of undeclared variables
//! - Duplicate variable declarations
//! - Calls to undefined functions
//!
//! The semantic analyzer maintains a symbol table to track declared variables and functions,
//! and collects errors for reporting without halting analysis (allowing multiple errors to
//! be reported at once).
//! 
use crate::significance::tokenizer::Position;
use crate::significance::ast_parser::{Program, Statement, VarType, Expression};
use std::collections::HashMap;

/// Semantic errors that can occur during analysis.
///
/// These errors represent violations of the language's semantic rules that are detected
/// after parsing but before execution. Each error includes position information for
/// helpful error reporting.\
#[derive(Clone)]
pub enum SemanticError {
    /// Attempt to use a variable that has not been declared.
    ///
    /// Contains the variable name and the position where it was referenced.
    ///
    /// # Example
    ///
    /// ```ignore
    /// x := 5  // Error: x not declared
    /// ```
    VariableNotDeclared(String, Position),
    
    /// Attempt to declare a variable that already exists.
    ///
    /// Contains the variable name and the position of the duplicate declaration.
    ///
    /// # Example
    ///
    /// ```ignore
    /// {x : real}
    /// {x : real}  // Error: x already declared
    /// ```
    VariableAlreadyDeclared(String, Position),

    /// Attempt to assign to a variable that has already been assigned to.
    ///
    /// Contains the variable name and the position of the assignment.
    ///
    /// # Example
    ///
    /// ```ignore
    /// x := 5
    /// x := 10  // Error: x already assigned
    /// ```
    VariableAlreadyAssigned(String, Position),

    /// Attempt to use a variable that has not been assigned to.
    ///
    /// Contains the variable name and the position where it was referenced.
    ///
    /// # Example
    ///
    /// ```ignore
    /// {x:real}
    /// x  // Error: x not assigned
    /// ```
    VariableNotAssigned(String, Position),
    
    /// Attempt to call a function that has not been declared or imported.
    ///
    /// Contains the function name and the position of the call.
    ///
    /// # Example
    ///
    /// ```ignore
    /// result := unknown_func(5)  // Error: unknown_func not declared
    /// ```
    FunctionNotDeclared(String, Position),
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::VariableNotDeclared(name, pos) => 
                write!(f, "Error at {}:{}: Variable '{}' not declared", pos.line, pos.column, name),
            SemanticError::VariableAlreadyDeclared(name, pos) => 
                write!(f, "Error at {}:{}: Variable '{}' already declared", pos.line, pos.column, name),
            SemanticError::VariableAlreadyAssigned(name, pos) => 
                write!(f, "Error at {}:{}: Variable '{}' already assigned", pos.line, pos.column, name),
            SemanticError::FunctionNotDeclared(name, pos) => 
                write!(f, "Error at {}:{}: Function '{}' not declared", pos.line, pos.column, name),
            SemanticError::VariableNotAssigned(name, pos) => 
                write!(f, "Error at {}:{}: Variable '{}' not assigned", pos.line, pos.column, name),
        }
    }
}

/// Semantic analyzer for the Significance language.
///
/// Performs single-pass semantic analysis over an AST, validating that:
/// - All variables are declared before use
/// - No duplicate declarations exist
/// - All function calls reference valid functions
///
/// The analyzer uses a symbol table (HashMap) to track all declared identifiers
/// (variables and functions) along with their types and declaration positions.
/// Errors are collected during analysis and can be retrieved afterward.
///
/// # Analysis Strategy
///
/// The analyzer performs a depth-first traversal of the AST, maintaining state in
/// the symbol table. It follows these rules:
/// 1. Variable declarations add entries to the symbol table
/// 2. Variable references are checked against the symbol table
/// 3. Function calls are validated against imported/standard library functions
/// 4. Errors are logged but analysis continues to find multiple errors
///
/// # Example
///
/// ```ignore
/// let mut analyzer = SemanticAnalyzer::new();
/// analyzer.import_standard_library();  // Add sin, cos, sqrt, etc.
/// analyzer.analyze_program(&ast);
///
/// if !analyzer.get_errors().is_empty() {
///     // Handle semantic errors
/// }
/// ```

pub struct SemanticAnalyzer {
    /// Symbol table mapping identifier names to their metadata.
    ///
    /// Contains both user-declared variables and imported functions.
    /// Key: identifier name, Value: variable/function information
    symbol_table: HashMap<String, VarInfo>,
    
    /// Accumulated semantic errors found during analysis.
    ///
    /// Errors are collected rather than immediately failing, allowing the
    /// analyzer to report multiple problems in a single pass.
    errors: Vec<SemanticError>,
}

/// Information about a declared variable or function.
///
/// Stored in the symbol table to track metadata about each identifier,
/// including its type and where it was declared (for error reporting).
pub struct VarInfo {
    /// The type of this variable or function
    var_type: VarType,
    
    /// Source position where this identifier was declared
    declared_at: Position,

    /// Whether this variable has been assigned a value
    assigned: bool,
}

impl VarInfo {
    /// Returns the type of this variable or function.
    ///
    /// # Returns
    ///
    /// Reference to the `VarType` (e.g., `Real` or `RealFunction`)
    pub fn get_type(&self) -> &VarType {
        &self.var_type
    }

    /// Returns the position where this identifier was declared.
    ///
    /// Useful for error messages that reference the original declaration.
    ///
    /// # Returns
    ///
    /// Reference to the declaration position
    pub fn get_declared_at(&self) -> &Position {
        &self.declared_at
    }

    /// Marks this variable as having been assigned a value.
    ///
    pub fn mark_assigned(&mut self) {
        self.assigned = true
    }

    /// Returns whether this variable has been assigned a value.
    ///
    /// # Returns
    ///
    /// `true` if the variable has been assigned a value, `false` otherwise
    pub fn is_assigned(&self) -> bool {
        self.assigned
    }
}

/// Creates a HashMap of standard library functions.
/// 
/// # Returns
/// 
/// A HashMap mapping standard libary function names to their metadata
pub fn build_standard_library() -> HashMap<String, VarInfo> {
    let mut std_symbol_table = HashMap::new();
    std_symbol_table.insert("sin".to_string(), VarInfo { var_type: VarType::RealFunction, declared_at: Position { line: 0, column: 0 }, assigned:true });
    std_symbol_table.insert("cos".to_string(), VarInfo { var_type: VarType::RealFunction, declared_at: Position { line: 0, column: 0 }, assigned:true });
    std_symbol_table.insert("sqrt".to_string(), VarInfo { var_type: VarType::RealFunction, declared_at: Position { line: 0, column: 0 }, assigned:true });

    std_symbol_table
}

impl SemanticAnalyzer {
    /// Creates a new semantic analyzer with an empty symbol table.
    ///
    /// The analyzer starts with no declared variables or functions. Standard
    /// library functions must be imported separately via `import_standard_library()`.
    ///
    /// # Returns
    ///
    /// A new `SemanticAnalyzer` instance ready for analysis
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            errors: Vec::new(),
        }
    }

    /// Returns all semantic errors found during analysis.
    ///
    /// Errors are accumulated during the analysis pass and can be retrieved
    /// afterward for reporting. An empty vector indicates no semantic errors.
    ///
    /// # Returns
    ///
    /// Reference to the vector of accumulated errors
    pub fn get_errors(&self) -> &Vec<SemanticError> {
        &self.errors
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    /// Resets the analyzer to its initial state.
    ///
    /// Clears both the symbol table and accumulated errors. Useful for
    /// analyzing multiple programs with the same analyzer instance, or
    /// for REPL implementations that need to preserve state between inputs.
    pub fn reset(&mut self) {
        self.symbol_table.clear();
        self.import_standard_library();
        self.errors.clear();
    }

    /// Analyzes a complete program.
    ///
    /// Performs semantic analysis on all statements in the program sequentially.
    /// Each statement is validated against the current symbol table state, with
    /// declarations updating the table for subsequent statements.
    ///
    /// # Arguments
    ///
    /// * `program` - The parsed program AST to analyze
    ///
    /// # Note
    ///
    /// This method does not return errors directly. Use `get_errors()` after
    /// analysis to retrieve any semantic errors found.
    pub fn analyze_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
    }

    /// Imports the standard library functions into the symbol table.
    ///
    /// Adds built-in functions like `sin`, `cos`, and `sqrt` to the symbol table
    /// so they can be called without explicit declaration. These functions are
    /// marked with a special position (line 0, column 0) to indicate they're
    /// not user-declared.
    ///
    /// # Standard Library Functions
    ///
    /// - `sin(x)` - Sine function
    /// - `cos(x)` - Cosine function  
    /// - `sqrt(x)` - Square root function
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut analyzer = SemanticAnalyzer::new();
    /// analyzer.import_standard_library();
    /// // Now sin, cos, sqrt are available for use
    /// ```
    pub fn import_standard_library(&mut self) -> &mut Self{

        self.import_library(build_standard_library());
        self
    }

    /// Imports a custom library (set of functions) into the symbol table.
    ///
    /// Allows extending the analyzer with additional built-in or library functions
    /// beyond the standard library. The provided HashMap is merged into the existing
    /// symbol table.
    ///
    /// # Arguments
    ///
    /// * `library` - HashMap of identifier names to their variable information
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut custom_lib = HashMap::new();
    /// custom_lib.insert("log".to_string(), VarInfo { 
    ///     var_type: VarType::RealFunction,
    ///     declared_at: Position { line: 0, column: 0 },
    ///     assigned: true
    /// });
    /// analyzer.import_library(custom_lib);
    /// ```
    pub fn import_library(&mut self, library: HashMap<String, VarInfo>) {
        self.symbol_table.extend(library);
    }

    /// Analyzes a single statement.
    ///
    /// Dispatches to the appropriate analysis method based on statement type:
    /// - Variable declarations update the symbol table
    /// - Assignments check that variables exist and validate the expression
    /// - Expression statements validate the expression
    ///
    /// # Arguments
    ///
    /// * `statement` - The statement to analyze
    pub fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDeclaration { name, var_type, pos } => {
                self.declare_variable(name, var_type, pos);
            }
            Statement::Assignment { name, value, pos } => {
                self.analyze_assignment(name, value, pos);
            }
            Statement::Expression(expression) => {
                self.analyze_expression(expression, &Position { line: 0, column: 0 });
            }
        }
    }

    /// Analyzes a variable declaration and adds it to the symbol table.
    ///
    /// Checks if a variable with the same name already exists. If so, records
    /// a `VariableAlreadyDeclared` error but still adds the new declaration to
    /// the symbol table (allowing analysis to continue).
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name being declared
    /// * `var_type` - Type of the variable
    /// * `pos` - Source position of the declaration
    ///
    /// # Errors Detected
    ///
    /// Records `VariableAlreadyDeclared` if a variable with this name already exists.
    pub fn declare_variable(&mut self, name: &str, var_type: &VarType, pos: &Position) {

        if self.symbol_table.contains_key(name) {
            self.errors.push(SemanticError::VariableAlreadyDeclared(name.to_string(), pos.clone()));
        }

        self.symbol_table.insert(
            name.to_string(),
            VarInfo {
                var_type: var_type.clone(),
                declared_at: pos.clone(),
                assigned: false
            }
        );
    }

    /// Analyzes an assignment statement.
    ///
    /// Validates that:
    /// 1. The variable being assigned to has been declared
    /// 2. The variable has not already been assigned (immutability)
    /// 3. The value expression is semantically valid
    ///
    /// The expression is always analyzed regardless of whether the variable
    /// exists or has been assigned, allowing multiple errors to be reported
    /// in a single pass.
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name being assigned to
    /// * `value` - Expression being assigned
    /// * `pos` - Source position of the assignment
    ///
    /// # Errors Detected
    ///
    /// - `VariableNotDeclared` if the assignment target doesn't exist
    /// - `VariableAlreadyAssigned` if the variable has already been assigned a value
    pub fn analyze_assignment(&mut self, name: &str, value: &Expression, pos: &Position) {
        // First: check variable state (immutable borrow, released at end of match)
        let should_mark = match self.symbol_table.get(name) {
            None => {
                self.errors.push(SemanticError::VariableNotDeclared(name.to_string(), pos.clone()));
                false
            }
            Some(var_info) => {
                if var_info.is_assigned() {
                    self.errors.push(SemanticError::VariableAlreadyAssigned(name.to_string(), pos.clone()));
                    false
                } else {
                    true
                }
            }
        };

        // Analyze expression (no borrow held)
        let n_err = self.errors.len();
        self.analyze_expression(value, pos);

        // Now mutate if everything was valid
        if should_mark && self.errors.len() == n_err {
            if let Some(var_info) = self.symbol_table.get_mut(name) {
                var_info.mark_assigned();
            }
        }
    }

    /// Recursively analyzes an expression.
    ///
    /// Performs depth-first validation of the expression tree, checking that:
    /// - All variable references are to declared variables
    /// - All function calls reference declared functions
    /// - All sub-expressions are valid
    ///
    /// # Arguments
    ///
    /// * `expression` - The expression to analyze
    /// * `pos` - Source position (used for error reporting when expression doesn't have its own position)
    ///
    /// # Errors Detected
    ///
    /// - `VariableNotDeclared` for references to undeclared variables
    /// - `FunctionNotDeclared` for calls to undeclared functions
    pub fn analyze_expression(&mut self, expression: &Expression, pos: &Position) {
        match expression {
            Expression::Binary { left, op: _, right, pos } => {
                self.analyze_expression(left, pos);
                self.analyze_expression(right, pos);
            },
            Expression::Unary { op: _, operand, pos } => {
                self.analyze_expression(operand, pos);
            },
            Expression::NumberWithUncertainty { value: _, error: _ , pos:_} => {
                // Literals are always valid, no analysis needed
            },
            Expression::FunctionCall { name, args, pos } => {
                self.analyze_function_call(name, args, pos); 
            },
            Expression::Variable(name) => {
                if self.symbol_table.get(name).is_none() {
                    self.errors.push(SemanticError::VariableNotDeclared(name.to_string(), pos.clone()));
                }
                else{
                    let var_info = self.symbol_table.get(name).unwrap();
                    if !var_info.is_assigned() {
                        self.errors.push(SemanticError::VariableNotAssigned(name.to_string(), pos.clone()));
                    }
                }
            }
        }
    }

    /// Analyzes a function call expression.
    ///
    /// Validates that:
    /// 1. The function being called exists in the symbol table
    /// 2. All argument expressions are semantically valid
    ///
    /// Note: This analyzer does not perform type checking or validate argument counts.
    /// Those checks are deferred to runtime or could be added in future versions.
    ///
    /// # Arguments
    ///
    /// * `name` - Function name being called
    /// * `args` - Vector of argument expressions
    /// * `pos` - Source position of the function call
    ///
    /// # Errors Detected
    ///
    /// Records `FunctionNotDeclared` if the function doesn't exist in the symbol table.
    pub fn analyze_function_call(&mut self, name: &str, args: &Vec<Expression>, pos: &Position) {
        if self.symbol_table.get(name).is_none() {
            self.errors.push(SemanticError::FunctionNotDeclared(name.to_string(), pos.clone()));
        }
        for arg in args {
            self.analyze_expression(arg, pos);
        }
    }

}