//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

use crate::significance::tokenizer::{Token, TokenWithPos, Tokenizer};

const FLOAT_EPSILON: f64 = 1e-10;

fn assert_tokens(actual: &[TokenWithPos], expected: &[Token]) {

    // check tokens until divergence occurs, if it occurs
    let min_len = actual.len().min(expected.len());
    
    for i in 0..min_len {
        let actual_token = &actual[i].token;
        let expected_token = &expected[i];
        
        let tokens_match = match (actual_token, expected_token) {
            (Token::Number(actual_num), Token::Number(expected_num)) => {
                (actual_num - expected_num).abs() < FLOAT_EPSILON
            }
            _ => actual_token == expected_token
        };
        
        if !tokens_match {
            panic!("Token mismatch at index {}:\n  Expected: {:?}\n  Actual:   {:?}\n  Length: expected={}, actual={}",
                    i, expected_token, actual_token, expected.len(), actual.len());
        }
    }
    
    if actual.len() != expected.len() {
        if actual.len() > expected.len() {
            println!("Extra actual tokens:");
            for (i, token) in actual[expected.len()..].iter().enumerate() {
                println!("  [{}]: {:?}", expected.len() + i, token.token);
            }
            panic!("Expected {} tokens, got {} tokens. Extra actual tokens shown above.",
                    expected.len(), actual.len());
        } else {
            println!("Missing expected tokens:");
            for (i, token) in expected[actual.len()..].iter().enumerate() {
                println!("  [{}]: {:?}", actual.len() + i, token);
            }
            panic!("Expected {} tokens, got {} tokens. Missing expected tokens shown above.",
                    expected.len(), actual.len());
        }
    }
}

#[test]
fn test_tokenize_basic_assignment() {
    let mut tokenizer = Tokenizer::new("x := 12.3 +/- 0.5");
    let tokens = tokenizer.tokenize().unwrap();
    
    assert_tokens(&tokens, &[
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(12.3),
        Token::PlusMinus,
        Token::Number(0.5),
        Token::EOF,
    ]);
}

#[test]
fn test_tokenize_variable_declaration() {
    let mut tokenizer = Tokenizer::new("{x : real}");
    let tokens = tokenizer.tokenize().unwrap();
    
    assert_tokens(&tokens, &[
        Token::LeftBrace,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Real,
        Token::RightBrace,
        Token::EOF,
    ]);
}

#[test]
fn test_tokenize_full_example_program() {
    let input = r#"#This is an example program in significance

{x : real} # this is the `x` variable
{y : real} # this represents a change in `x`
{z : real} # z is the next iteration of `x`

x := 12.3 +/- 0.5
y := 2.6 +/- 0.2
z := x + y
z
w := x*x + y**2
w"#;
    
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize().unwrap();
    
    assert_tokens(&tokens, &[
        Token::Comment("This is an example program in significance".to_string()),
        Token::Newline,
        Token::Newline,
        Token::LeftBrace, Token::Identifier("x".to_string()), Token::Colon, Token::Real, Token::RightBrace,
        Token::Comment(" this is the `x` variable".to_string()),
        Token::Newline,
        Token::LeftBrace, Token::Identifier("y".to_string()), Token::Colon, Token::Real, Token::RightBrace,
        Token::Comment(" this represents a change in `x`".to_string()),
        Token::Newline,
        Token::LeftBrace, Token::Identifier("z".to_string()), Token::Colon, Token::Real, Token::RightBrace,
        Token::Comment(" z is the next iteration of `x`".to_string()),
        Token::Newline,
        Token::Newline,
        Token::Identifier("x".to_string()), Token::Assign, Token::Number(12.3), Token::PlusMinus, Token::Number(0.5),
        Token::Newline,
        Token::Identifier("y".to_string()), Token::Assign, Token::Number(2.6), Token::PlusMinus, Token::Number(0.2),
        Token::Newline,
        Token::Identifier("z".to_string()), Token::Assign, Token::Identifier("x".to_string()), Token::Plus, Token::Identifier("y".to_string()),
        Token::Newline,
        Token::Identifier("z".to_string()),
        Token::Newline,
        Token::Identifier("w".to_string()), Token::Assign, Token::Identifier("x".to_string()), Token::Multiply, Token::Identifier("x".to_string()), 
        Token::Plus, Token::Identifier("y".to_string()), Token::Power, Token::Number(2.0),
        Token::Newline,
        Token::Identifier("w".to_string()),
        Token::EOF,
    ]);
}
