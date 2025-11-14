#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identifiers and literals
    Identifier(String),
    Number(String),  // Store as string for now, will parse later
    
    // Operators
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    
    // Assignment
    Assign,      // =
    
    // Parentheses
    LParen,      // (
    RParen,      // )
    
    // End of line/statement
    Newline,
    
    // End of file
    EOF,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Identifier(name) => write!(f, "Identifier({})", name),
            Token::Number(value) => write!(f, "Number({})", value),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Assign => write!(f, "="),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Newline => write!(f, "newline"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}