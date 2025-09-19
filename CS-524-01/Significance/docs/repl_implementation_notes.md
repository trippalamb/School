# REPL Implementation Plan

## Design Pattern: Validate-Then-Commit

For REPL error handling, use transactional behavior where each statement is "all or nothing":
- **Success**: Both symbol table AND variable values get updated
- **Failure**: No state changes, just print error and continue

## Architecture Overview

### Core Components

```rust
pub struct SemanticAnalyzer {
    symbol_table: HashMap<String, VarInfo>,
    errors: Vec<SemanticError>,
}

pub struct Executor {
    variable_values: HashMap<String, Real>, // actual runtime values
}

struct ReplSession {
    analyzer: SemanticAnalyzer,
    executor: Executor,
}
```

### Key Methods Needed

#### SemanticAnalyzer Changes
```rust
impl SemanticAnalyzer {
    // Read-only validation (doesn't modify state)
    pub fn validate_statement(&self, statement: &Statement) -> Vec<SemanticError> {
        // Check against current symbol table, but don't modify it
    }
    
    // State-modifying operation (only call after validation passes)
    pub fn commit_statement(&mut self, statement: &Statement) {
        // Actually update the symbol table
    }
    
    // Keep existing methods for file mode
    pub fn analyze_program(&mut self, program: &Program) { ... }
    pub fn analyze_statement(&mut self, statement: &Statement) { ... }
}
```

#### Executor Design
```rust
impl Executor {
    pub fn execute_statement(&mut self, statement: &Statement, symbol_table: &HashMap<String, VarInfo>) -> Result<Option<Real>, ExecutionError> {
        // Returns Some(value) for expressions, None for declarations/assignments
    }
}
```

### REPL Session Implementation

```rust
impl ReplSession {
    fn execute_line(&mut self, input: &str) -> Result<Option<Real>, Error> {
        // Phase 1: Parse and validate (read-only)
        let statement = parse_statement(input)?;
        let temp_errors = self.analyzer.validate_statement(&statement);
        
        if !temp_errors.is_empty() {
            return Err(/* errors */); // No state was modified
        }
        
        // Phase 2: Commit changes (we know it's valid)
        self.analyzer.commit_statement(&statement);
        let result = self.executor.execute_statement(&statement, &self.analyzer.symbol_table)?;
        
        Ok(result)
    }
}
```

## Required Parser Changes

1. **Make `Parser::parse_statement()` public** for single-statement parsing
2. **Handle incomplete input** - detect when statement needs more lines
3. **EOF handling** - single statements won't have EOF tokens

## State Management

### File Mode Flow
1. Parse entire file → Complete `Program` AST
2. `analyzer.analyze_program(program)` → All errors at once
3. `executor.execute_program(program, symbol_table)` → All results

### REPL Mode Flow
1. Parse individual statements → One `Statement` at a time
2. `analyzer.validate_statement()` → Check for errors (read-only)
3. If valid: `analyzer.commit_statement()` → Update symbol table
4. `executor.execute_statement()` → Update variable values and get result
5. Maintain persistent state across statements

## Error Recovery Strategy

- **Syntax Errors**: Show error, continue session
- **Semantic Errors**: Show error, no state changes, continue session  
- **Runtime Errors**: Show error, partial state rollback may be needed
- **Incomplete Input**: Prompt with `...` for continuation

## Dual Mode Support

The same codebase supports both:
- **File Mode**: Create components → analyze entire program → execute entire program
- **REPL Mode**: Create persistent session → validate/commit/execute per statement

This design allows both use cases without major architectural changes.