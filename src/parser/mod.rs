use anyhow::Ok;

use crate::ast::StmtKind::Function;
use crate::ast::{Expr, ExprKind, ExprStmt, StmtKind};
use crate::{
    ast::{FunctionStmt, Identifier, Program, Stmt, Visibility},
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

    pub fn parse(&mut self) -> anyhow::Result<Program> {
        let mut program = Program::default();
        while self.current_token.kind != TokenKind::Eof {
            let stmt = self.parse_stmt()?;
            program.body.push(stmt);
        }
        Ok(program)
    }
    pub fn parse_stmt(&mut self) -> anyhow::Result<Stmt> {
        println!("parse_stmt {:?}", self.current_token);
        match self.current_token.kind {
            TokenKind::Fn => self.parser_function_stmt(None),
            TokenKind::Ident(_) => {
                let expr = self.parse_expr()?;
                // ";"
                self.next_token()?;
                let expr_stmt = ExprStmt { expr };
                Ok(Stmt {
                    kind: StmtKind::Expr(expr_stmt),
                })
            }
            _ => {
                println!("{:?}", self.current_token.kind);
                todo!()
            }
        }
    }

    pub fn parser_function_stmt(&mut self, visibility: Option<Visibility>) -> anyhow::Result<Stmt> {
        match self.current_token.kind {
            TokenKind::Fn => {
                // fn
                self.next_token()?;
                let name = match &self.current_token.kind {
                    TokenKind::Ident(ident) => Identifier {
                        name: ident.clone(),
                    },
                    _ => panic!(),
                };
                // main
                self.next_token()?;
                self.parse_func_arg()?;
                let body = self.parse_body()?;
                let func = FunctionStmt {
                    name,
                    body,
                    visibility,
                };
                Ok(Stmt {
                    kind: Function(func),
                })
            }
            _ => panic!(),
        }
    }

    pub fn parse_body(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let mut body = vec![];
        // '{'
        self.next_token()?;

        while self.current_token.kind != TokenKind::Rbrace {
            let stmt = self.parse_stmt()?;
            body.push(stmt);
        }
        // '}'
        self.next_token()?;

        Ok(body)
    }

    pub fn parse_func_arg(&mut self) -> anyhow::Result<()> {
        // '('
        self.next_token()?;
        // ')'
        self.next_token()?;
        Ok(())
    }

    pub fn parse_expr(&mut self) -> anyhow::Result<Expr> {
        match &self.current_token.kind {
            TokenKind::Ident(ident) => match self.next_token.kind {
                TokenKind::Lparen => self.parse_call_expr(),
                _ => {
                    println!("{:?}", self.current_token);
                    todo!()
                }
            },
            TokenKind::String(s) => {
                let expr = Expr {
                    kind: ExprKind::Str(s.clone()),
                };
                self.next_token();
                Ok(expr)
            }
            _ => {
                println!("{:?}", self.current_token);
                todo!()
            }
        }
    }

    pub fn parse_call_expr(&mut self) -> anyhow::Result<Expr> {
        match &self.current_token.kind {
            TokenKind::Ident(ident) => {
                let func = Expr {
                    kind: ExprKind::Ident(ident.clone()),
                };
                // func name
                self.next_token()?;

                // "("
                self.next_token()?;

                let mut args = vec![];

                while self.current_token.kind != TokenKind::Rparen {
                    let expr = self.parse_expr()?;
                    args.push(expr);
                }

                // ")"
                self.next_token()?;

                Ok(Expr {
                    kind: ExprKind::Call(crate::ast::CallExpr {
                        func: Box::new(func),
                        args,
                    }),
                })
            }
            _ => panic!(),
        }
    }
}
