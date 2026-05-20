use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens
            .get(self.position)
            .unwrap_or_else(|| self.tokens.last().unwrap())
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    fn advance(&mut self) -> Token {
        let token = self.current_token().clone();
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        token
    }

    fn expect(&mut self, expected: TokenType) -> Result<Token, String> {
        if std::mem::discriminant(&self.current_token().token_type)
            == std::mem::discriminant(&expected)
        {
            Ok(self.advance())
        } else {
            Err(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token().token_type
            ))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current_token().token_type, TokenType::Newline) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        self.skip_newlines();

        // Parse package declaration
        let package = if matches!(self.current_token().token_type, TokenType::Pkg) {
            self.advance();
            let name = match &self.current_token().token_type {
                TokenType::Identifier(n) => n.clone(),
                _ => return Err("Expected identifier after 'pkg'".to_string()),
            };
            self.advance();
            self.skip_newlines();
            name
        } else {
            "main".to_string()
        };

        // Parse imports
        let mut imports = Vec::new();
        while matches!(self.current_token().token_type, TokenType::Import) {
            self.advance();
            if let TokenType::String(path) = &self.current_token().token_type {
                imports.push(path.clone());
                self.advance();
            }
            self.skip_newlines();
        }

        // Parse top-level items
        let mut items = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::Eof) {
            self.skip_newlines();
            if matches!(self.current_token().token_type, TokenType::Eof) {
                break;
            }

            let is_pub = matches!(self.current_token().token_type, TokenType::Pub);
            if is_pub {
                self.advance();
            }

            match &self.current_token().token_type {
                TokenType::Fn => {
                    items.push(TopLevel::Function(self.parse_function(is_pub)?));
                }
                TokenType::Struct => {
                    items.push(TopLevel::Struct(self.parse_struct(is_pub)?));
                }
                TokenType::Trait => {
                    items.push(TopLevel::Trait(self.parse_trait(is_pub)?));
                }
                TokenType::Impl => {
                    items.push(TopLevel::Impl(self.parse_impl()?));
                }
                TokenType::Enum => {
                    items.push(TopLevel::Enum(self.parse_enum(is_pub)?));
                }
                TokenType::Const => {
                    items.push(TopLevel::Const(self.parse_const()?));
                }
                _ => {
                    self.advance();
                }
            }
            self.skip_newlines();
        }

        Ok(Program {
            package,
            imports,
            items,
        })
    }

    fn parse_function(&mut self, is_public: bool) -> Result<FunctionDef, String> {
        self.expect(TokenType::Fn)?;

        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenType::RightParen)?;

        let return_type = if matches!(self.current_token().token_type, TokenType::RightArrow) {
            self.advance();
            self.parse_type()?
        } else {
            Type::Any
        };

        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
            is_public,
        })
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, String> {
        let mut params = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::RightParen) {
            let is_mutable = if matches!(self.current_token().token_type, TokenType::Mut) {
                self.advance();
                true
            } else {
                false
            };

            let name = match &self.current_token().token_type {
                TokenType::Identifier(n) => n.clone(),
                _ => return Err("Expected parameter name".to_string()),
            };
            self.advance();

            self.expect(TokenType::Colon)?;
            let param_type = self.parse_type()?;

            params.push(Parameter {
                name,
                param_type,
                is_mutable,
            });

            if matches!(self.current_token().token_type, TokenType::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match &self.current_token().token_type {
            TokenType::Identifier(name) => {
                let type_name = name.clone();
                self.advance();
                Ok(match type_name.as_str() {
                    "i8" => Type::I8,
                    "i16" => Type::I16,
                    "i32" => Type::I32,
                    "i64" => Type::I64,
                    "i128" => Type::I128,
                    "u8" => Type::U8,
                    "u16" => Type::U16,
                    "u32" => Type::U32,
                    "u64" => Type::U64,
                    "u128" => Type::U128,
                    "f32" => Type::F32,
                    "f64" => Type::F64,
                    "bool" => Type::Bool,
                    "str" => Type::String,
                    _ => Type::Struct(type_name),
                })
            }
            TokenType::Ampersand => {
                self.advance();
                let is_mutable = matches!(self.current_token().token_type, TokenType::Mut);
                if is_mutable {
                    self.advance();
                }
                let inner = self.parse_type()?;
                Ok(Type::Reference(Box::new(inner), is_mutable))
            }
            TokenType::Star => {
                self.advance();
                let inner = self.parse_type()?;
                Ok(Type::Pointer(Box::new(inner)))
            }
            TokenType::Chan => {
                self.advance();
                let inner = self.parse_type()?;
                Ok(Type::Channel(Box::new(inner)))
            }
            TokenType::LeftBracket => {
                self.advance();
                self.expect(TokenType::RightBracket)?;
                let inner = self.parse_type()?;
                Ok(Type::Array(Box::new(inner)))
            }
            _ => Err(format!("Unexpected type: {:?}", self.current_token().token_type)),
        }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(TokenType::LeftBrace)?;
        self.skip_newlines();

        let mut statements = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::RightBrace) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(TokenType::RightBrace)?;

        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.current_token().token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Const => {
                let const_def = self.parse_const()?;
                Ok(Statement::Const(const_def))
            }
            TokenType::If => {
                let if_stmt = self.parse_if_statement()?;
                Ok(Statement::If(if_stmt))
            }
            TokenType::For => {
                let for_stmt = self.parse_for_statement()?;
                Ok(Statement::For(for_stmt))
            }
            TokenType::Loop => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::Loop(body))
            }
            TokenType::Break => {
                self.advance();
                self.consume_semicolon();
                Ok(Statement::Break)
            }
            TokenType::Continue => {
                self.advance();
                self.consume_semicolon();
                Ok(Statement::Continue)
            }
            TokenType::Return => {
                self.advance();
                let expr = if matches!(self.current_token().token_type, TokenType::Semicolon)
                    || matches!(self.current_token().token_type, TokenType::RightBrace)
                {
                    None
                } else {
                    Some(Box::new(self.parse_expression()?))
                };
                self.consume_semicolon();
                Ok(Statement::Return(expr))
            }
            TokenType::Defer => {
                self.advance();
                let stmt = self.parse_statement()?;
                Ok(Statement::Defer(Box::new(stmt)))
            }
            TokenType::Co => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::Co(body))
            }
            TokenType::Panic => {
                self.advance();
                let expr = if matches!(self.current_token().token_type, TokenType::LeftParen) {
                    self.advance();
                    let e = self.parse_expression();
                    self.expect(TokenType::RightParen)?;
                    e.ok().map(Box::new)
                } else {
                    None
                };
                self.consume_semicolon();
                Ok(Statement::Panic(expr))
            }
            TokenType::Semicolon => {
                self.advance();
                Ok(Statement::Empty)
            }
            _ => {
                let expr = self.parse_expression()?;
                self.consume_semicolon();
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::Let)?;

        let is_mutable = matches!(self.current_token().token_type, TokenType::Mut);
        if is_mutable {
            self.advance();
        }

        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected identifier in let statement".to_string()),
        };
        self.advance();

        let value_type = if matches!(self.current_token().token_type, TokenType::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenType::Equal)?;
        let value = self.parse_expression()?;
        self.consume_semicolon();

        Ok(Statement::Let(LetStmt {
            name,
            value_type,
            value,
            is_mutable,
        }))
    }

    fn parse_const(&mut self) -> Result<ConstDef, String> {
        self.expect(TokenType::Const)?;

        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected identifier in const".to_string()),
        };
        self.advance();

        self.expect(TokenType::Colon)?;
        let const_type = self.parse_type()?;

        self.expect(TokenType::Equal)?;
        let value = self.parse_expression()?;
        self.consume_semicolon();

        Ok(ConstDef {
            name,
            const_type,
            value,
        })
    }

    fn parse_if_statement(&mut self) -> Result<IfStmt, String> {
        self.expect(TokenType::If)?;

        let condition = self.parse_expression()?;
        self.skip_newlines();

        let then_block = self.parse_block()?;
        self.skip_newlines();

        let (else_block, else_simple) = if matches!(self.current_token().token_type, TokenType::Else)
        {
            self.advance();
            self.skip_newlines();

            if matches!(self.current_token().token_type, TokenType::If) {
                let nested_if = self.parse_if_statement()?;
                (Some(Box::new(nested_if)), None)
            } else {
                let else_body = self.parse_block()?;
                (None, Some(Box::new(else_body)))
            }
        } else {
            (None, None)
        };

        Ok(IfStmt {
            condition,
            then_block,
            else_block,
            else_simple,
        })
    }

    fn parse_for_statement(&mut self) -> Result<ForStmt, String> {
        self.expect(TokenType::For)?;

        let init = if matches!(self.current_token().token_type, TokenType::LeftBrace) {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };

        let condition = if matches!(self.current_token().token_type, TokenType::LeftBrace) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        let update = if matches!(self.current_token().token_type, TokenType::LeftBrace) {
            None
        } else {
            let expr = self.parse_expression();
            expr.ok()
        };

        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(ForStmt {
            init,
            condition,
            update,
            body,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_or_expression()
    }

    fn parse_or_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_and_expression()?;

        while matches!(self.current_token().token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_and_expression()?;
            expr = Expression::Binary(BinaryOp::Or, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_and_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality_expression()?;

        while matches!(self.current_token().token_type, TokenType::And) {
            self.advance();
            let right = self.parse_equality_expression()?;
            expr = Expression::Binary(BinaryOp::And, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_equality_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_relational_expression()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };

            self.advance();
            let right = self.parse_relational_expression()?;
            expr = Expression::Binary(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_relational_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_additive_expression()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };

            self.advance();
            let right = self.parse_additive_expression()?;
            expr = Expression::Binary(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_additive_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_multiplicative_expression()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.advance();
            let right = self.parse_multiplicative_expression()?;
            expr = Expression::Binary(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_multiplicative_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary_expression()?;

        loop {
            let op = match &self.current_token().token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Percent => BinaryOp::Modulo,
                _ => break,
            };

            self.advance();
            let right = self.parse_unary_expression()?;
            expr = Expression::Binary(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, String> {
        match &self.current_token().token_type {
            TokenType::Not => {
                self.advance();
                let expr = self.parse_unary_expression()?;
                Ok(Expression::Unary(UnaryOp::Not, Box::new(expr)))
            }
            TokenType::Minus => {
                self.advance();
                let expr = self.parse_unary_expression()?;
                Ok(Expression::Unary(UnaryOp::Negate, Box::new(expr)))
            }
            TokenType::Ampersand => {
                self.advance();
                let is_mutable = matches!(self.current_token().token_type, TokenType::Mut);
                if is_mutable {
                    self.advance();
                }
                let expr = self.parse_unary_expression()?;
                Ok(Expression::Reference(Box::new(expr), is_mutable))
            }
            TokenType::Star => {
                self.advance();
                let expr = self.parse_unary_expression()?;
                Ok(Expression::Dereference(Box::new(expr)))
            }
            TokenType::LeftArrow => {
                self.advance();
                let expr = self.parse_primary_expression()?;
                Ok(Expression::ChannelReceive(Box::new(expr)))
            }
            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary_expression()?;

        loop {
            match &self.current_token().token_type {
                TokenType::LeftParen => {
                    self.advance();
                    let args = self.parse_arguments()?;
                    self.expect(TokenType::RightParen)?;
                    expr = Expression::Call(Box::new(expr), args);
                }
                TokenType::Dot => {
                    self.advance();
                    if let TokenType::Identifier(member) = &self.current_token().token_type {
                        let member = member.clone();
                        self.advance();
                        expr = Expression::MemberAccess(Box::new(expr), member);
                    } else {
                        return Err("Expected member name after '.'".to_string());
                    }
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenType::RightBracket)?;
                    expr = Expression::Index(Box::new(expr), Box::new(index));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, String> {
        match &self.current_token().token_type.clone() {
            TokenType::Integer(n) => {
                let val = *n;
                self.advance();
                Ok(Expression::Integer(val))
            }
            TokenType::Float(f) => {
                let val = *f;
                self.advance();
                Ok(Expression::Float(val))
            }
            TokenType::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expression::String(val))
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::Bool(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expression::Bool(false))
            }
            TokenType::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Ok(Expression::Identifier(id))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                while !matches!(self.current_token().token_type, TokenType::RightBracket) {
                    elements.push(self.parse_expression()?);
                    if matches!(self.current_token().token_type, TokenType::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(TokenType::RightBracket)?;
                Ok(Expression::Array(elements))
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?}",
                self.current_token().token_type
            )),
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, String> {
        let mut args = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::RightParen) {
            args.push(self.parse_expression()?);
            if matches!(self.current_token().token_type, TokenType::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn parse_struct(&mut self, is_public: bool) -> Result<StructDef, String> {
        self.expect(TokenType::Struct)?;

        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected struct name".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftBrace)?;
        let mut fields = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::RightBrace) {
            if let TokenType::Identifier(field_name) = &self.current_token().token_type {
                let field_name = field_name.clone();
                self.advance();
                self.expect(TokenType::Colon)?;
                let field_type = self.parse_type()?;

                fields.push(StructField {
                    name: field_name,
                    field_type,
                });

                if matches!(self.current_token().token_type, TokenType::Comma) {
                    self.advance();
                }
            }
            self.skip_newlines();
        }

        self.expect(TokenType::RightBrace)?;

        Ok(StructDef {
            name,
            fields,
            is_public,
        })
    }

    fn parse_trait(&mut self, is_public: bool) -> Result<TraitDef, String> {
        self.expect(TokenType::Trait)?;

        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected trait name".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftBrace)?;
        let mut methods = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::RightBrace) {
            self.skip_newlines();
            if matches!(self.current_token().token_type, TokenType::Fn) {
                methods.push(self.parse_function(false)?);
            }
            self.skip_newlines();
        }

        self.expect(TokenType::RightBrace)?;

        Ok(TraitDef {
            name,
            methods,
            is_public,
        })
    }

    fn parse_impl(&mut self) -> Result<ImplBlock, String> {
        self.expect(TokenType::Impl)?;

        let for_type = match &self.current_token().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected type name in impl".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftBrace)?;
        let mut methods = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::RightBrace) {
            self.skip_newlines();
            if matches!(self.current_token().token_type, TokenType::Fn) {
                methods.push(self.parse_function(false)?);
            }
            self.skip_newlines();
        }

        self.expect(TokenType::RightBrace)?;

        Ok(ImplBlock { for_type, methods })
    }

    fn parse_enum(&mut self, is_public: bool) -> Result<EnumDef, String> {
        self.expect(TokenType::Enum)?;

        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected enum name".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftBrace)?;
        let mut variants = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::RightBrace) {
            self.skip_newlines();
            if let TokenType::Identifier(var_name) = &self.current_token().token_type {
                let var_name = var_name.clone();
                self.advance();

                let data = if matches!(self.current_token().token_type, TokenType::LeftParen) {
                    self.advance();
                    let t = self.parse_type()?;
                    self.expect(TokenType::RightParen)?;
                    Some(t)
                } else {
                    None
                };

                variants.push(EnumVariant {
                    name: var_name,
                    data,
                });

                if matches!(self.current_token().token_type, TokenType::Comma) {
                    self.advance();
                }
            }
            self.skip_newlines();
        }

        self.expect(TokenType::RightBrace)?;

        Ok(EnumDef {
            name,
            variants,
            is_public,
        })
    }

    fn consume_semicolon(&mut self) {
        if matches!(self.current_token().token_type, TokenType::Semicolon) {
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function() {
        let code = "fn add(x: i32, y: i32) -> i32 { return x + y; }";
        let mut parser = Parser::new(code);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_let() {
        let code = "fn main() { let x: i32 = 42; }";
        let mut parser = Parser::new(code);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
