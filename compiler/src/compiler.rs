use crate::ir::{lower_program, Module};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::parser::{Parser, Program};
use crate::semantic::SemanticAnalyzer;

#[derive(Debug)]
pub struct CompilationResult {
    pub ast: Program,
    pub ir: Module,
}

pub fn compile(source: &str) -> Result<CompilationResult, String> {
    let mut lexer = Lexer::new(source);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let token = lexer
            .next_token()
            .map_err(|error| format!("Lexer error: {}", error))?;

        let is_eof = matches!(token.kind, TokenKind::EOF);

        tokens.push(token);

        if is_eof {
            break;
        }
    }

    let mut parser = Parser::new(tokens);

    let ast = parser
        .parse()
        .map_err(|error| format!("Parser error: {}", error))?;

    let mut analyzer = SemanticAnalyzer::new();

    analyzer
        .analyze(&ast)
        .map_err(|error| format!("{}", error))?;

    let ir = lower_program(&ast);

    Ok(CompilationResult { ast, ir })
}
