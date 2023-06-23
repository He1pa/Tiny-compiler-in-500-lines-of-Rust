use anyhow::Ok;

use crate::ast::StmtKind::Function;
use crate::{
    ast::{FunctionStmt, Module, Stmt, Visibility, Identifier,},
    lexer::{
        self,
        token::{Token, TokenKind},
        Lexer,
    },
};

#[derive(Clone, Debug)]
pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub next_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> anyhow::Result<Self> {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenKind::Eof),
            next_token: Token::new(TokenKind::Eof),
        };
        parser.next_token()?;
        parser.next_token()?;
        Ok(parser)
    }

    pub fn next_token(&mut self) -> anyhow::Result<()> {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse(&mut self) -> anyhow::Result<Module> {
        let mut module = Module::default();
        while self.current_token.kind != TokenKind::Eof {
            let stmt = self.parse_stmt()?;
            module.body.push(stmt);
        }
        Ok(module)
    }
    pub fn parse_stmt(&mut self) -> anyhow::Result<Stmt> {
        match self.current_token.kind {
            TokenKind::Fn => {
                self.parser_function_stmt(Some(Visibility::Pub))
            }
            _ => {
                println!("{:?}", self.current_token.kind);
                todo!()
            },
        }
    }

    pub fn parser_function_stmt(&mut self, visibility: Option<Visibility>) -> anyhow::Result<Stmt> {
        match self.current_token.kind {
            TokenKind::Fn => {
                // fn
                self.next_token()?;
                let name = match &self.current_token.kind{
                    TokenKind::Ident(ident) => Identifier{name: ident.clone()},
                    _ => panic!()
                };
                // main
                self.next_token()?;
                self.parse_func_arg()?;
                let body = self.parse_body()?;
                let func = FunctionStmt{
                    name,
                    body,
                    visibility,
                };
                Ok(Stmt{kind: Function(func)})
            }
            _ => panic!()
        }
    }

    pub fn parse_body(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let mut body = vec![];
        // '{'
        self.next_token()?;

        while self.current_token.kind != TokenKind::Rbrace{
            let stmt = self.parse_stmt()?;
            body.push(stmt);
        }
        // '}'
        self.next_token()?;

        Ok(body)
    }

    pub fn parse_func_arg(&mut self) -> anyhow::Result<()>{
        // '('
        self.next_token()?;
        // ')'
        self.next_token()?;
        Ok(())
    }
}
