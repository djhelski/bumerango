#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),

    // Keywords
    Fn,
    Let,
    Const,
    If,
    Else,
    For,
    Loop,
    Break,
    Continue,
    Return,
    Match,
    Co,           // concurrency
    Chan,         // channel
    Make,
    Pkg,
    Import,
    Use,
    Pub,
    Priv,
    Struct,
    Trait,
    Impl,
    Enum,
    Union,
    Mut,
    Ref,
    Unsafe,
    Defer,
    Panic,
    True,
    False,

    // Operators
    Plus,
    Minus,
    Star,         // * or &
    Slash,
    Percent,
    Equal,        // =
    EqualEqual,   // ==
    NotEqual,     // !=
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,          // &&
    Or,           // ||
    Not,          // !
    Ampersand,    // &
    Pipe,         // |
    Caret,        // ^
    LeftArrow,    // <-
    RightArrow,   // ->
    FatArrow,     // =>
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    DoubleColon,
    Comma,
    Dot,
    DotDot,
    DotDotEqual,
    Question,

    // Special
    Eof,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        if self.position + offset < self.input.len() {
            Some(self.input[self.position + offset])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.current_char() == Some('/') && self.peek_char(1) == Some('/') {
            self.advance();
            self.advance();
            while let Some(ch) = self.current_char() {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> TokenType {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char() {
            if ch.is_numeric() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && self.peek_char(1).map_or(false, |c| c.is_numeric()) {
                is_float = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            TokenType::Float(num_str.parse().unwrap_or(0.0))
        } else {
            TokenType::Integer(num_str.parse().unwrap_or(0))
        }
    }

    fn read_string(&mut self) -> String {
        self.advance(); // Skip opening quote
        let mut string = String::new();

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => string.push(escaped),
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }
        string
    }

    fn keyword_or_identifier(&self, ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Fn,
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "loop" => TokenType::Loop,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            "match" => TokenType::Match,
            "co" => TokenType::Co,
            "chan" => TokenType::Chan,
            "make" => TokenType::Make,
            "pkg" => TokenType::Pkg,
            "import" => TokenType::Import,
            "use" => TokenType::Use,
            "pub" => TokenType::Pub,
            "priv" => TokenType::Priv,
            "struct" => TokenType::Struct,
            "trait" => TokenType::Trait,
            "impl" => TokenType::Impl,
            "enum" => TokenType::Enum,
            "union" => TokenType::Union,
            "mut" => TokenType::Mut,
            "ref" => TokenType::Ref,
            "unsafe" => TokenType::Unsafe,
            "defer" => TokenType::Defer,
            "panic" => TokenType::Panic,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Identifier(ident.to_string()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            if self.current_char() == Some('/') && self.peek_char(1) == Some('/') {
                self.skip_comment();
                continue;
            }
            break;
        }

        let line = self.line;
        let column = self.column;

        let token_type = match self.current_char() {
            None => TokenType::Eof,
            Some('\n') => {
                self.advance();
                TokenType::Newline
            }
            Some('(') => {
                self.advance();
                TokenType::LeftParen
            }
            Some(')') => {
                self.advance();
                TokenType::RightParen
            }
            Some('{') => {
                self.advance();
                TokenType::LeftBrace
            }
            Some('}') => {
                self.advance();
                TokenType::RightBrace
            }
            Some('[') => {
                self.advance();
                TokenType::LeftBracket
            }
            Some(']') => {
                self.advance();
                TokenType::RightBracket
            }
            Some(';') => {
                self.advance();
                TokenType::Semicolon
            }
            Some(',') => {
                self.advance();
                TokenType::Comma
            }
            Some(':') => {
                self.advance();
                if self.current_char() == Some(':') {
                    self.advance();
                    TokenType::DoubleColon
                } else {
                    TokenType::Colon
                }
            }
            Some('.') => {
                self.advance();
                match self.current_char() {
                    Some('.') => {
                        self.advance();
                        if self.current_char() == Some('=') {
                            self.advance();
                            TokenType::DotDotEqual
                        } else {
                            TokenType::DotDot
                        }
                    }
                    _ => TokenType::Dot,
                }
            }
            Some('?') => {
                self.advance();
                TokenType::Question
            }
            Some('+') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::PlusEqual
                } else {
                    TokenType::Plus
                }
            }
            Some('-') => {
                self.advance();
                if self.current_char() == Some('>') {
                    self.advance();
                    TokenType::RightArrow
                } else if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::MinusEqual
                } else {
                    TokenType::Minus
                }
            }
            Some('*') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::StarEqual
                } else {
                    TokenType::Star
                }
            }
            Some('/') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::SlashEqual
                } else {
                    TokenType::Slash
                }
            }
            Some('%') => {
                self.advance();
                TokenType::Percent
            }
            Some('=') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::EqualEqual
                } else if self.current_char() == Some('>') {
                    self.advance();
                    TokenType::FatArrow
                } else {
                    TokenType::Equal
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::NotEqual
                } else {
                    TokenType::Not
                }
            }
            Some('<') => {
                self.advance();
                if self.current_char() == Some('-') {
                    self.advance();
                    TokenType::LeftArrow
                } else if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            Some('&') => {
                self.advance();
                if self.current_char() == Some('&') {
                    self.advance();
                    TokenType::And
                } else {
                    TokenType::Ampersand
                }
            }
            Some('|') => {
                self.advance();
                if self.current_char() == Some('|') {
                    self.advance();
                    TokenType::Or
                } else {
                    TokenType::Pipe
                }
            }
            Some('^') => {
                self.advance();
                TokenType::Caret
            }
            Some('"') => TokenType::String(self.read_string()),
            Some(ch) if ch.is_numeric() => self.read_number(),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                self.keyword_or_identifier(&ident)
            }
            Some(ch) => {
                self.advance();
                TokenType::Identifier(ch.to_string())
            }
        };

        let lexeme = format!("{:?}", token_type);
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = matches!(token.token_type, TokenType::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 6); // let x = 42 ; eof
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("fn main() { return 10; }");
        let tokens = lexer.tokenize();
        // Verify keyword recognition
        assert!(matches!(tokens[0].token_type, TokenType::Fn));
    }
}
