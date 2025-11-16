use crate::token::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character: '{0}' at position {1}")]
    UnexpectedCharacter(char, usize),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() && self.ch != '\n' {
            self.read_char();
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' || self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }
    
    fn read_number(&mut self) -> String {
        let position = self.position;
        
        // Read integer part and first decimal/separator
        while self.ch.is_ascii_digit() || self.ch == '-' {
            self.read_char();
        }
        
        // Check for decimal point (base-10) or semicolon (sexagesimal)
        if self.ch == '.' || self.ch == ';' || self.ch == ',' {
            self.read_char(); // consume the separator
            
            // Read fractional part
            while self.ch.is_ascii_digit() {
                self.read_char();
            }
        }
        
        self.input[position..self.position].iter().collect()
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        
        while self.ch != '\0' {
            match self.ch {
                // Skip whitespace (except newlines)
                ' ' | '\t' | '\r' => {
                    self.skip_whitespace();
                    continue;
                }
                
                // Newline
                '\n' => {
                    tokens.push(Token::Newline);
                    self.read_char();
                }
                
                // Operators
                '+' => {
                    tokens.push(Token::Plus);
                    self.read_char();
                }
                '-' => {
                    // Check if this is a negative number or subtraction
                    if self.peek_char().is_ascii_digit() && 
                       (tokens.is_empty() || 
                        matches!(tokens.last(), Some(Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Assign | Token::LParen))) {
                        // It's a negative number, let read_number handle it
                        let num = self.read_number();
                        tokens.push(Token::Number(num));
                    } else {
                        tokens.push(Token::Minus);
                        self.read_char();
                    }
                }
                '*' => {
                    tokens.push(Token::Asterisk);
                    self.read_char();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.read_char();
                }
                '=' => {
                    tokens.push(Token::Assign);
                    self.read_char();
                }
                
                // Parentheses
                '(' => {
                    tokens.push(Token::LParen);
                    self.read_char();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.read_char();
                }
                
                // Number separators (handled in read_number)
                '.' | ';' | ',' => {
                    // These should be consumed as part of number reading
                    // If we encounter them here, it's an error
                    return Err(LexerError::UnexpectedCharacter(self.ch, self.position));
                }
                
                // Identifiers (start with letter or underscore)
                ch if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.read_identifier();
                    tokens.push(Token::Identifier(ident));
                }
                
                // Numbers (including negative and with separators)
                ch if ch.is_ascii_digit() => {
                    let num = self.read_number();
                    tokens.push(Token::Number(num));
                }
                
                // Unexpected character
                _ => {
                    return Err(LexerError::UnexpectedCharacter(
                        self.ch, 
                        self.position
                    ));
                }
            }
        }
        
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_sexagesimal_notation() {
        let input = "1;30 + 2;45";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::Number("1;30".to_string()),
            Token::Plus,
            Token::Number("2;45".to_string()),
            Token::EOF,
        ]);
    }
    
    #[test]
    fn test_comma_notation() {
        let input = "1,30 * 2,15";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::Number("1,30".to_string()),
            Token::Asterisk,
            Token::Number("2,15".to_string()),
            Token::EOF,
        ]);
    }
    
    #[test]
    fn test_negative_numbers() {
        let input = "-5 + -3.14";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::Number("-5".to_string()),
            Token::Plus,
            Token::Number("-3.14".to_string()),
            Token::EOF,
        ]);
    }
    
    #[test]
    fn test_mixed_formats() {
        let input = "x = 10 + 2;30 - 5.5";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Number("10".to_string()),
            Token::Plus,
            Token::Number("2;30".to_string()),
            Token::Minus,
            Token::Number("5.5".to_string()),
            Token::EOF,
        ]);
    }
}