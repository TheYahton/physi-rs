use crate::lexer::Token;
use crate::units::{KELVIN, KILOGRAM};
use crate::{Magic, SIDimension, SIUnit};

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

#[derive(Debug)]
pub enum Expr {
    Number(u32),
    Unit(SIUnit),
    UnknownUnit(Magic),
    Identifier(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

impl Token {
    fn to_operator(&self) -> Result<BinaryOp, String> {
        match self {
            Token::Plus => Ok(BinaryOp::Add),
            Token::Minus => Ok(BinaryOp::Subtract),
            Token::Asterisk => Ok(BinaryOp::Multiply),
            Token::Slash => Ok(BinaryOp::Divide),
            Token::Equal => Ok(BinaryOp::Equal),
            _ => Err("to_operator failed".to_string()),
        }
    }

    fn to_dim(&self) -> Result<SIDimension, String> {
        match self {
            Token::Unit(string) => match string.as_str() {
                "kg" => Ok(KILOGRAM.dim),
                "C" => Ok(KELVIN.dim),
                _ => todo!("{string:?}"),
            },
            _ => Err("to_dim failed".to_string()),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::End)
    }

    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, expected: &Token) -> Result<(), String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?} but found {:?}",
                expected,
                self.peek()
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.equal();
        if !matches!(self.peek(), &Token::End) {
            return Err(format!(
                "ERROR! Unexpected token at the end: {:?}",
                self.peek()
            ));
        }

        expr
    }

    fn equal(&mut self) -> Result<Expr, String> {
        let mut node = self.expr();

        while let Token::Equal = self.peek() {
            let op = self.advance().to_operator();
            let right = self.expr();
            node = Ok(Expr::Binary {
                op: op?,
                left: Box::new(node?),
                right: Box::new(right?),
            })
        }

        node
    }

    fn expr(&mut self) -> Result<Expr, String> {
        let mut node = self.term();

        while let Token::Plus | Token::Minus = self.peek() {
            let op = self.advance().to_operator();
            let right = self.term();
            node = Ok(Expr::Binary {
                op: op?,
                left: Box::new(node?),
                right: Box::new(right?),
            });
        }

        node
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut node = self.unit();

        while let Token::Asterisk | Token::Slash = self.peek() {
            let op = self.advance().to_operator();
            let right = self.unit();
            node = Ok(Expr::Binary {
                op: op?,
                left: Box::new(node?),
                right: Box::new(right?),
            });
        }

        node
    }

    fn unit(&mut self) -> Result<Expr, String> {
        let mut node = self.factor();

        while let Token::Unit(_) = self.peek() {
            let dim = self.advance().to_dim();
            match node? {
                Expr::Number(n) => node = Ok(Expr::Unit(SIUnit::from_dim(n as f64, dim?))),
                Expr::Identifier(name) => {
                    node = Ok(Expr::UnknownUnit(Magic {
                        identifier: name,
                        unit: SIUnit::from_dim(1.0, dim?),
                    }))
                }
                _ => todo!("unit failed"),
            }
        }

        node
    }

    fn factor(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::NumLiteral(value) => {
                let number = *value;
                self.advance();
                Ok(Expr::Number(number))
            }
            Token::Unknown(name) => {
                let id = name.to_string();
                self.advance();
                Ok(Expr::Identifier(id))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expr();
                self.consume(&Token::RightParen)?;
                Ok(Expr::Grouping(Box::new(expr?)))
            }
            other => todo!("{other:?}"),
        }
    }
}
