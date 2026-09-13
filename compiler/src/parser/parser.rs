use crate::lexer::{Token, TokenKind};

use super::ast::*;
use super::error::ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &TokenKind {
        &self.tokens[self.position].kind
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
    }

    fn error<T>(&self, message: impl Into<String>) -> Result<T, ParserError> {
        let token = self.current_token();

        Err(ParserError {
            message: message.into(),
            line: token.line,
            column: token.column,
        })
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParserError> {
        if *self.current() == expected {
            self.advance();
            Ok(())
        } else {
            self.error(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.current()
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut systems = Vec::new();

        while !matches!(self.current(), TokenKind::EOF) {
            systems.push(self.parse_system()?);
        }

        Ok(Program { systems })
    }

    fn parse_system(&mut self) -> Result<System, ParserError> {
        self.expect(TokenKind::System)?;

        let name = match self.current() {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }

            _ => {
                return self.error("Expected system name");
            }
        };

        self.expect(TokenKind::LBrace)?;

        let mut body = Vec::new();

        while !matches!(self.current(), TokenKind::RBrace | TokenKind::EOF) {
            body.push(self.parse_system_item()?);
        }

        self.expect(TokenKind::RBrace)?;

        Ok(System { name, body })
    }

    fn parse_system_item(&mut self) -> Result<SystemItem, ParserError> {
        match self.current() {
            TokenKind::Signal => Ok(SystemItem::Signal(self.parse_signal()?)),

            TokenKind::Channel => Ok(SystemItem::Channel(self.parse_channel()?)),

            TokenKind::Receiver => Ok(SystemItem::Receiver(self.parse_receiver()?)),

            TokenKind::Transmitter => Ok(SystemItem::Transmitter(self.parse_transmitter()?)),

            TokenKind::Connect => Ok(SystemItem::Connect(self.parse_connection()?)),

            TokenKind::Simulate => Ok(SystemItem::Simulate(self.parse_simulation()?)),

            _ => self.error(format!("Unexpected token in system: {:?}", self.current())),
        }
    }

    fn parse_signal(&mut self) -> Result<Signal, ParserError> {
        self.expect(TokenKind::Signal)?;

        let name = self.parse_identifier()?;

        self.expect(TokenKind::LBrace)?;

        let properties = self.parse_properties()?;

        self.expect(TokenKind::RBrace)?;

        Ok(Signal { name, properties })
    }

    fn parse_channel(&mut self) -> Result<Channel, ParserError> {
        self.expect(TokenKind::Channel)?;

        let name = self.parse_identifier()?;

        self.expect(TokenKind::LBrace)?;

        let properties = self.parse_properties()?;

        self.expect(TokenKind::RBrace)?;

        Ok(Channel { name, properties })
    }

    fn parse_receiver(&mut self) -> Result<Receiver, ParserError> {
        self.expect(TokenKind::Receiver)?;

        let name = self.parse_identifier()?;

        self.expect(TokenKind::LBrace)?;

        let properties = self.parse_properties()?;

        self.expect(TokenKind::RBrace)?;

        Ok(Receiver { name, properties })
    }

    fn parse_transmitter(&mut self) -> Result<Transmitter, ParserError> {
        self.expect(TokenKind::Transmitter)?;

        let name = self.parse_identifier()?;

        self.expect(TokenKind::LBrace)?;

        let properties = self.parse_properties()?;

        self.expect(TokenKind::RBrace)?;

        Ok(Transmitter { name, properties })
    }

    fn parse_properties(&mut self) -> Result<Vec<Property>, ParserError> {
        let mut properties = Vec::new();

        while !matches!(self.current(), TokenKind::RBrace | TokenKind::EOF) {
            let name = self.parse_identifier()?;

            self.expect(TokenKind::Assign)?;

            let value = self.parse_value()?;

            properties.push(Property { name, value });
        }

        Ok(properties)
    }

    fn parse_value(&mut self) -> Result<Value, ParserError> {
        let value = match self.current() {
            TokenKind::Integer(value) => {
                let value = *value;
                self.advance();
                Value::Integer(value)
            }

            TokenKind::Float(value) => {
                let value = *value;
                self.advance();
                Value::Float(value)
            }

            TokenKind::String(value) => {
                let value = value.clone();
                self.advance();
                Value::String(value)
            }

            TokenKind::True => {
                self.advance();
                Value::Boolean(true)
            }

            TokenKind::False => {
                self.advance();
                Value::Boolean(false)
            }

            TokenKind::Identifier(value) => {
                let value = value.clone();
                self.advance();
                Value::Identifier(value)
            }

            _ => {
                return self.error(format!("Expected value, found {:?}", self.current()));
            }
        };

        self.parse_optional_unit(value)
    }

    fn parse_optional_unit(&mut self, value: Value) -> Result<Value, ParserError> {
        let unit = match self.current() {
            TokenKind::Hz => Some(Unit::Hz),
            TokenKind::KHz => Some(Unit::KHz),
            TokenKind::MHz => Some(Unit::MHz),
            TokenKind::GHz => Some(Unit::GHz),
            TokenKind::W => Some(Unit::W),
            TokenKind::DB => Some(Unit::DB),
            TokenKind::DBm => Some(Unit::DBm),
            _ => None,
        };

        if let Some(unit) = unit {
            self.advance();

            Ok(Value::Quantity {
                value: Box::new(value),
                unit,
            })
        } else {
            Ok(value)
        }
    }

    fn parse_connection(&mut self) -> Result<Connection, ParserError> {
        self.expect(TokenKind::Connect)?;

        let mut nodes = Vec::new();

        nodes.push(self.parse_identifier()?);

        while matches!(self.current(), TokenKind::Arrow) {
            self.advance();

            nodes.push(self.parse_identifier()?);
        }

        Ok(Connection { nodes })
    }

    fn parse_simulation(&mut self) -> Result<Simulation, ParserError> {
        self.expect(TokenKind::Simulate)?;

        self.expect(TokenKind::LBrace)?;

        let properties = self.parse_properties()?;

        self.expect(TokenKind::RBrace)?;

        Ok(Simulation { properties })
    }

    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        match self.current() {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }

            _ => self.error(format!("Expected identifier, found {:?}", self.current())),
        }
    }
}
