use crate::lexer::{LexerError, Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    // ---------------------------------------------------------
    // Look at the current character without consuming it
    // ---------------------------------------------------------
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    // ---------------------------------------------------------
    // Look at the next character
    // ---------------------------------------------------------
    fn peek_next(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    // ---------------------------------------------------------
    // Consume one character
    // ---------------------------------------------------------
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();

        if let Some(c) = ch {
            self.position += 1;

            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        ch
    }

    // ---------------------------------------------------------
    // Skip whitespace
    // ---------------------------------------------------------
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // ---------------------------------------------------------
    // Skip comments
    //
    // Supports:
    //
    // // single-line comment
    //
    // /* multi-line comment */
    // ---------------------------------------------------------
    fn skip_comments(&mut self) -> Result<(), LexerError> {
        loop {
            // Single-line comment
            if self.peek() == Some('/') && self.peek_next() == Some('/') {
                self.advance();
                self.advance();

                while let Some(c) = self.peek() {
                    self.advance();

                    if c == '\n' {
                        break;
                    }
                }

                continue;
            }

            // Multi-line comment
            if self.peek() == Some('/') && self.peek_next() == Some('*') {
                let start_line = self.line;
                let start_column = self.column;

                self.advance();
                self.advance();

                loop {
                    match self.peek() {
                        Some('*') if self.peek_next() == Some('/') => {
                            self.advance();
                            self.advance();
                            break;
                        }

                        Some(_) => {
                            self.advance();
                        }

                        None => {
                            return Err(LexerError {
                                message: "Unterminated multi-line comment".to_string(),
                                line: start_line,
                                column: start_column,
                            });
                        }
                    }
                }

                continue;
            }

            break;
        }

        Ok(())
    }

    // ---------------------------------------------------------
    // Read an identifier or keyword
    // ---------------------------------------------------------
    fn identifier(&mut self) -> String {
        let mut text = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                text.push(c);
                self.advance();
            } else {
                break;
            }
        }

        text
    }

    // ---------------------------------------------------------
    // Read a number
    //
    // Supports:
    //
    // 10
    // 3.14
    // 1e6
    // 2.5e-3
    // ---------------------------------------------------------
    fn number(&mut self) -> Result<TokenKind, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let mut text = String::new();

        // Integer part
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                text.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part
        if self.peek() == Some('.') {
            text.push('.');
            self.advance();

            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    text.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Scientific notation
        if matches!(self.peek(), Some('e') | Some('E')) {
            text.push(self.advance().unwrap());

            // Optional + or -
            if matches!(self.peek(), Some('+') | Some('-')) {
                text.push(self.advance().unwrap());
            }

            // Scientific notation must contain digits
            let mut exponent_digits = 0;

            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    text.push(c);
                    exponent_digits += 1;
                    self.advance();
                } else {
                    break;
                }
            }

            if exponent_digits == 0 {
                return Err(LexerError {
                    message: format!("Invalid scientific notation '{}'", text),
                    line: start_line,
                    column: start_column,
                });
            }
        }

        // Determine integer vs floating-point number
        if text.contains('.') || text.contains('e') || text.contains('E') {
            match text.parse::<f64>() {
                Ok(value) => Ok(TokenKind::Float(value)),

                Err(_) => Err(LexerError {
                    message: format!("Invalid floating-point number '{}'", text),
                    line: start_line,
                    column: start_column,
                }),
            }
        } else {
            match text.parse::<i64>() {
                Ok(value) => Ok(TokenKind::Integer(value)),

                Err(_) => Err(LexerError {
                    message: format!("Invalid integer '{}'", text),
                    line: start_line,
                    column: start_column,
                }),
            }
        }
    }

    // ---------------------------------------------------------
    // Read a string literal
    // ---------------------------------------------------------
    fn string(&mut self) -> Result<String, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        // Consume opening "
        self.advance();

        let mut result = String::new();

        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    self.advance();
                    return Ok(result);
                }

                '\\' => {
                    self.advance();

                    match self.peek() {
                        Some('n') => {
                            result.push('\n');
                            self.advance();
                        }

                        Some('t') => {
                            result.push('\t');
                            self.advance();
                        }

                        Some('r') => {
                            result.push('\r');
                            self.advance();
                        }

                        Some('\\') => {
                            result.push('\\');
                            self.advance();
                        }

                        Some('"') => {
                            result.push('"');
                            self.advance();
                        }

                        Some(other) => {
                            return Err(LexerError {
                                message: format!("Unknown escape sequence '\\{}'", other),
                                line: self.line,
                                column: self.column,
                            });
                        }

                        None => {
                            return Err(LexerError {
                                message: "Unterminated string literal".to_string(),
                                line: start_line,
                                column: start_column,
                            });
                        }
                    }
                }

                '\n' => {
                    return Err(LexerError {
                        message: "Unterminated string literal".to_string(),
                        line: start_line,
                        column: start_column,
                    });
                }

                _ => {
                    result.push(c);
                    self.advance();
                }
            }
        }

        Err(LexerError {
            message: "Unterminated string literal".to_string(),
            line: start_line,
            column: start_column,
        })
    }

    // ---------------------------------------------------------
    // Convert identifier names into keywords when appropriate
    // ---------------------------------------------------------
    fn keyword(name: String) -> TokenKind {
        match name.as_str() {
            "system" => TokenKind::System,
            "signal" => TokenKind::Signal,
            "channel" => TokenKind::Channel,
            "receiver" => TokenKind::Receiver,
            "transmitter" => TokenKind::Transmitter,
            "connect" => TokenKind::Connect,
            "simulate" => TokenKind::Simulate,
            "library" => TokenKind::Library,
            "function" => TokenKind::Function,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "const" => TokenKind::Const,
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,

            // Units
            "Hz" => TokenKind::Hz,
            "kHz" => TokenKind::KHz,
            "MHz" => TokenKind::MHz,
            "GHz" => TokenKind::GHz,

            "W" => TokenKind::W,
            "dB" => TokenKind::DB,
            "dBm" => TokenKind::DBm,

            _ => TokenKind::Identifier(name),
        }
    }

    // ---------------------------------------------------------
    // Get the next token
    // ---------------------------------------------------------
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        loop {
            self.skip_whitespace();

            self.skip_comments()?;

            let before_comments = self.position;

            self.skip_whitespace();

            // Nothing was consumed by comment handling
            // and we're at the same position.
            if self.position == before_comments {
                break;
            }
        }

        let line = self.line;
        let column = self.column;

        let ch = match self.peek() {
            Some(c) => c,
            None => {
                return Ok(Token {
                    kind: TokenKind::EOF,
                    line,
                    column,
                });
            }
        };

        // -----------------------------------------------------
        // Identifiers / keywords
        // -----------------------------------------------------
        if ch.is_alphabetic() || ch == '_' {
            let name = self.identifier();

            return Ok(Token {
                kind: Self::keyword(name),
                line,
                column,
            });
        }

        // -----------------------------------------------------
        // Numbers
        // -----------------------------------------------------
        if ch.is_ascii_digit() {
            let kind = self.number()?;

            return Ok(Token { kind, line, column });
        }

        // -----------------------------------------------------
        // String
        // -----------------------------------------------------
        if ch == '"' {
            let value = self.string()?;

            return Ok(Token {
                kind: TokenKind::String(value),
                line,
                column,
            });
        }

        // -----------------------------------------------------
        // Symbols / operators
        // -----------------------------------------------------
        let kind = match ch {
            '{' => {
                self.advance();
                TokenKind::LBrace
            }

            '}' => {
                self.advance();
                TokenKind::RBrace
            }

            '(' => {
                self.advance();
                TokenKind::LParen
            }

            ')' => {
                self.advance();
                TokenKind::RParen
            }

            ':' => {
                self.advance();
                TokenKind::Colon
            }

            ';' => {
                self.advance();
                TokenKind::Semicolon
            }

            ',' => {
                self.advance();
                TokenKind::Comma
            }

            '=' => {
                self.advance();

                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }

            '!' => {
                self.advance();

                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::NotEqual
                } else {
                    return Err(LexerError {
                        message: "Unexpected '!'".to_string(),
                        line,
                        column,
                    });
                }
            }

            '<' => {
                self.advance();

                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }

            '>' => {
                self.advance();

                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            '+' => {
                self.advance();
                TokenKind::Plus
            }

            '-' => {
                self.advance();

                if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }

            '*' => {
                self.advance();
                TokenKind::Multiply
            }

            '/' => {
                self.advance();
                TokenKind::Divide
            }

            _ => {
                return Err(LexerError {
                    message: format!("Unexpected character '{}'", ch),
                    line,
                    column,
                });
            }
        };

        Ok(Token { kind, line, column })
    }
}
