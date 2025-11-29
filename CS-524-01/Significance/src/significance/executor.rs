//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

//! Runtime execution engine for the Significance language.
//!
//! This module provides the executor that evaluates validated AST nodes, managing variable
//! state and computing results with uncertainty propagation. The executor assumes the AST
//! has already been semantically validated, though it still performs defensive runtime checks
//! for certain error conditions.
//!
//! # Execution Model
//!
//! The executor maintains a runtime environment (variable storage) and evaluates statements
//! sequentially. Expression evaluation is recursive, following the AST structure, with
//! uncertainty automatically propagated through all operations.
//!
//! # Error Handling
//!
//! Runtime errors are collected rather than immediately halting execution, allowing the
//! executor to continue and potentially find multiple errors. This is useful for interactive
//! REPL sessions where recovering from errors is important.

use std::collections::HashMap;
use crate::Real;
use crate::significance::ast_parser::{Program, Statement, VarType, Expression, BinaryOp, UnaryOp};
use crate::significance::tokenizer::Position;
use crate::significance::std_lib_call;

/// Runtime errors that can occur during program execution.
///
/// These errors represent conditions that cannot be detected during parsing or semantic
/// analysis, such as division by zero or attempts to use variables that somehow weren't
/// caught during semantic analysis.
#[derive(Debug, Clone)]
pub enum RunTimeError {
    /// Attempt to divide by zero.
    ///
    /// Division by zero produces infinite values, which are recorded but execution continues.
    /// The result will have infinite value and error.
    ///
    /// # Example
    ///
    /// ```ignore
    /// x := 10 / 0  // Runtime error: division by zero
    /// ```
    DivisionByZero(Position),
    
    /// Attempt to access a variable that doesn't exist at runtime.
    ///
    /// This should typically be caught during semantic analysis, but the executor
    /// performs defensive checks. Contains the variable name and position.
    ///
    /// # Example
    ///
    /// ```ignore
    /// y := x  // Runtime error if x wasn't declared (should be caught earlier)
    /// ```
    UndefinedVariable(String, Position)
}
impl std::fmt::Display for RunTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunTimeError::DivisionByZero(position) => write!(f, "Division by zero error at {}:{}.", position.line, position.column),
            RunTimeError::UndefinedVariable(name, position) => write!(f, "Undefined variable '{}' at {}:{}.", name, position.line, position.column),
        }
    }
}

/// Runtime representation of a variable.
///
/// Stores the current value of a variable during execution. The type information
/// is not stored at runtime since all variables in the current implementation are
/// of type `Real` (real numbers with uncertainty).
#[derive(Debug, Clone)]
pub struct VarRunTime{
    /// Current value of the variable (real number with uncertainty)
    value: Real
}

impl VarRunTime {
    /// Returns the current value of this variable.
    ///
    /// # Returns
    ///
    /// Reference to the `Real` value stored in this variable
    pub fn get_value(&self) -> &Real {
        &self.value
    }
}

/// Runtime execution engine for the Significance language.
///
/// The executor interprets a validated AST, maintaining runtime state (variable values)
/// and performing computations with automatic uncertainty propagation. It operates on
/// the assumption that semantic analysis has already validated the program structure.
///
/// # Execution Strategy
///
/// - **Variables**: Stored in a HashMap mapping names to `Real` values
/// - **Expressions**: Evaluated recursively with uncertainty propagated automatically
/// - **Statements**: Executed sequentially, modifying runtime state
/// - **Errors**: Collected non-fatally, allowing execution to continue
///
/// # State Management
///
/// The executor maintains mutable state that persists across statements within a program.
/// This enables:
/// - Variable assignments to affect subsequent statements
/// - REPL mode where state persists across multiple inputs
/// - Interactive debugging and exploration
///
/// # Example
///
/// ```ignore
/// let mut executor = Executor::new();
/// 
/// // Execute a program
/// executor.execute_program(&ast);
/// 
/// // Check for errors
/// if !executor.get_errors().is_empty() {
///     // Handle runtime errors
/// }
/// 
/// // Access variable values
/// if let Some(var) = executor.get_var("result") {
///     println!("Result: {}", var.get_value());
/// }
/// ```
pub struct Executor {
    /// Runtime variable storage mapping names to values.
    ///
    /// Variables are added via declarations and updated via assignments.
    /// All variables store `Real` values (numbers with uncertainty).
    run_time_vars: HashMap<String, VarRunTime>,
    
    /// Accumulated runtime errors encountered during execution.
    ///
    /// Errors are logged but don't halt execution, allowing the executor
    /// to continue and potentially discover multiple issues.
    errors: Vec<RunTimeError>
}

impl Executor{
    /// Creates a new executor with no variables or errors.
    ///
    /// The executor starts with a clean slate and is ready to execute programs.
    /// Variables must be declared via the program being executed.
    ///
    /// # Returns
    ///
    /// A new `Executor` instance ready for program execution
    pub fn new() -> Self {
        Self {
            run_time_vars: HashMap::new(),
            errors: Vec::new()
        }
    }

    /// Resets the executor to its initial state.
    ///
    /// Clears all variables but preserves error history. This is useful for
    /// executing multiple independent programs with the same executor instance
    /// or for resetting REPL state.
    ///
    /// # Note
    ///
    /// This method only clears variables, not accumulated errors. To clear errors,
    /// create a new executor instance.
    pub fn reset(&mut self) {
        self.run_time_vars.clear();
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    /// Retrieves the current value of a variable by name.
    ///
    /// Returns a clone of the variable's runtime representation if it exists.
    /// Used for inspecting variable values after execution or during debugging.
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name to look up
    ///
    /// # Returns
    ///
    /// * `Some(VarRunTime)` - Variable found with its current value
    /// * `None` - Variable does not exist
    pub fn get_var(&self, name: &str) -> Option<VarRunTime> {
        self.run_time_vars.get(name).cloned()
    }

    /// Executes a complete program.
    ///
    /// Processes all statements in the program sequentially, maintaining runtime
    /// state across statements. Each statement may read or modify variables,
    /// evaluate expressions, or produce output.
    ///
    /// # Arguments
    ///
    /// * `program` - The validated AST program to execute
    ///
    /// # Side Effects
    ///
    /// - Modifies runtime variable state
    /// - May print output for expression statements
    /// - May accumulate runtime errors
    pub fn execute_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.execute_statement(statement);
        }
    }

    /// Executes a single statement.
    ///
    /// Dispatches to the appropriate execution method based on statement type:
    /// - Variable declarations initialize new variables
    /// - Assignments update existing variable values
    /// - Expression statements evaluate and print results
    ///
    /// # Arguments
    ///
    /// * `statement` - The statement to execute
    ///
    /// # Side Effects
    ///
    /// - May modify runtime variable state
    /// - Expression statements print their result to stdout
    /// - May accumulate runtime errors
    pub fn execute_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDeclaration { name, var_type, pos } => {
                self.declare_variable(name, var_type, pos);
            }
            Statement::Assignment { name, value, pos } => {
                self.assign_variable(name, value, pos);
            }
            Statement::Expression(expression) => {
                let value = self.evaluate_expression(expression);
                print!("{}\n", value);
            }
        }
    }

    /// Declares and initializes a new variable.
    ///
    /// Creates a new variable in the runtime environment initialized to zero.
    /// The semantic analyzer ensures this is only called for new variables (no duplicates).
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name to declare
    /// * `_var_type` - Variable type (unused, currently all variables are `Real`)
    /// * `_` - Source position (unused, semantic checks handle validation)
    ///
    /// # Note
    ///
    /// Variables are initialized to `0.0` with no uncertainty. They must be assigned
    /// a value before use to hold meaningful data.
    pub fn declare_variable(&mut self, name: &str, _var_type: &VarType, _: &Position) {
        // Semantic analyzer ensures the variable is not previously declared
        self.run_time_vars.insert(
            name.to_string(),
            VarRunTime {
                value: Real::new(0.0)
            }
        );
    }

    /// Assigns a new value to an existing variable.
    ///
    /// Evaluates the expression and stores the result in the named variable.
    /// If the variable doesn't exist (defensive check), records a runtime error
    /// but continues execution.
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name to assign to
    /// * `value` - Expression to evaluate for the new value
    /// * `pos` - Source position (for error reporting)
    ///
    /// # Errors
    ///
    /// Records `UndefinedVariable` if the variable doesn't exist (should be prevented
    /// by semantic analysis).
    fn assign_variable(&mut self, name: &str, value: &Expression, pos: &Position) {
        let value = self.evaluate_expression(value);
        
        if let Some(var) = self.run_time_vars.get_mut(name) {
            var.value = value;
        } else {
            self.errors.push(RunTimeError::UndefinedVariable(name.to_string(), pos.clone()));
        }
    }

    /// Evaluates an expression and returns its computed value.
    ///
    /// Recursively evaluates the expression tree, automatically propagating uncertainty
    /// through all operations. The result is a `Real` number that includes both the
    /// computed value and its uncertainty.
    ///
    /// # Arguments
    ///
    /// * `expression` - The expression to evaluate
    ///
    /// # Returns
    ///
    /// The computed `Real` value with propagated uncertainty
    ///
    /// # Expression Types
    ///
    /// - **Numbers**: Direct conversion to `Real`
    /// - **Variables**: Lookup in runtime environment
    /// - **Binary operations**: Evaluate operands, apply operator with uncertainty propagation
    /// - **Unary operations**: Evaluate operand, apply operator
    /// - **Function calls**: Evaluate arguments, call standard library function
    pub fn evaluate_expression(&mut self, expression: &Expression) -> Real {
        match expression {
            Expression::NumberWithUncertainty { value, error, pos:_ } => Real::with_error(value.clone(), error.clone()),
            Expression::Variable(name) => self.evaluate_variable(name),
            Expression::Binary { left, op, right, pos } => {
                self.evaluate_expression_binary(left, op, right, pos)
            },
            Expression::Unary { op, operand, pos } => {
                self.evaluate_expression_unary(op, operand, pos)
            },
            Expression::FunctionCall { name, args, pos } => {
                self.evaluate_function_call(name, args, pos)
            }
        }
    }

    /// Evaluates a variable reference by looking up its current value.
    ///
    /// Retrieves the variable's value from the runtime environment. If the variable
    /// doesn't exist (defensive check), records an error and returns zero.
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name to look up
    ///
    /// # Returns
    ///
    /// The variable's current value, or `0.0` if not found
    ///
    /// # Errors
    ///
    /// Records `UndefinedVariable` if the variable doesn't exist (should be prevented
    /// by semantic analysis).
    fn evaluate_variable(&mut self, name: &str) -> Real {
        if let Some(var) = self.run_time_vars.get(name) {
            var.value.clone()
        } else {
            self.errors.push(RunTimeError::UndefinedVariable(
                name.to_string(),
                Position { line: 0, column: 0 } // Position not available here
            ));
            Real::new(0.0) // Return default value after logging error
        }
    }

    /// Evaluates a binary operation with uncertainty propagation.
    ///
    /// Evaluates both operands and applies the specified operator. Uncertainty is
    /// automatically propagated according to the rules implemented in the `Real` type.
    ///
    /// # Arguments
    ///
    /// * `left` - Left operand expression
    /// * `op` - Binary operator to apply
    /// * `right` - Right operand expression
    /// * `pos` - Source position (for error reporting)
    ///
    /// # Returns
    ///
    /// The result of the operation with propagated uncertainty
    ///
    /// # Operations
    ///
    /// - **Add/Sub**: Uncertainties combined in quadrature
    /// - **Mul/Div**: Relative uncertainties combined in quadrature
    /// - **Mod**: Conservative uncertainty estimation
    /// - **Power/Root**: Uncertainty propagated via derivatives
    ///
    /// # Errors
    ///
    /// Records `DivisionByZero` if dividing by zero (result is infinity with infinite error).
    fn evaluate_expression_binary(&mut self, left: &Expression, op: &BinaryOp, right: &Expression, pos: &Position) -> Real {
        let left_value = self.evaluate_expression(left);
        let right_value = self.evaluate_expression(right);

        match op {
            BinaryOp::Add => left_value + right_value,
            BinaryOp::Sub => left_value - right_value,
            BinaryOp::Mul => left_value * right_value,
            BinaryOp::Div => {
                if right_value == Real::new(0.0) {
                    self.errors.push(RunTimeError::DivisionByZero(pos.clone()));
                }
                left_value / right_value
            },
            BinaryOp::Mod => left_value % right_value,
            BinaryOp::Power => left_value.power(right_value),
            BinaryOp::Root => left_value.root(right_value),
        }
    }

    /// Evaluates a unary operation.
    ///
    /// Evaluates the operand and applies the specified unary operator.
    /// Uncertainty magnitude is preserved for unary minus, identity for unary plus.
    ///
    /// # Arguments
    ///
    /// * `op` - Unary operator to apply
    /// * `operand` - Operand expression
    /// * `_pos` - Source position (unused currently)
    ///
    /// # Returns
    ///
    /// The result of the operation with propagated uncertainty
    ///
    /// # Operations
    ///
    /// - **Plus**: Identity operation (returns operand unchanged)
    /// - **Minus**: Negates value (preserves uncertainty magnitude)
    fn evaluate_expression_unary(&mut self, op: &UnaryOp, operand: &Expression, _pos: &Position) -> Real {
        let operand_value = self.evaluate_expression(operand);
        match op {
            UnaryOp::Plus => operand_value,
            UnaryOp::Minus => -operand_value,
        }
    }

    /// Evaluates a function call by delegating to the standard library.
    ///
    /// Evaluates all argument expressions and passes the resulting values to the
    /// standard library function implementation. The standard library handles
    /// uncertainty propagation for mathematical functions.
    ///
    /// # Arguments
    ///
    /// * `name` - Function name to call
    /// * `args` - Slice of argument expressions
    /// * `pos` - Source position (passed to standard library for error reporting)
    ///
    /// # Returns
    ///
    /// The result of the function call with propagated uncertainty
    ///
    /// # Panics
    ///
    /// Panics if the function doesn't exist or has incorrect arity. These should
    /// be caught by semantic analysis, so panics here indicate bugs.
    fn evaluate_function_call(&mut self, name: &str, args: &[Expression], pos: &Position) -> Real {
        let vals: Vec<Real> = args.iter().map(|arg| self.evaluate_expression(arg)).collect();
        std_lib_call(name, &vals, pos)
    }

    /// Returns all runtime errors accumulated during execution.
    ///
    /// Errors are collected during execution rather than immediately halting the program.
    /// This allows REPL sessions to recover from errors and continue, and batch execution
    /// to report multiple problems.
    ///
    /// # Returns
    ///
    /// Vector of all runtime errors encountered
    pub fn get_errors(&self) -> Vec<RunTimeError> {
        self.errors.clone()
    }

}