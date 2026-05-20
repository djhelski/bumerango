use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Bool,
    String,
    Array(Box<Type>),
    Reference(Box<Type>, bool), // (inner, is_mutable)
    Pointer(Box<Type>),
    Channel(Box<Type>),
    Function(Vec<Type>, Box<Type>), // (params, return)
    Struct(String),
    Trait(String),
    Enum(String),
    Union(String),
    Any,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::String => "str".to_string(),
            Type::Array(t) => format!("[{}]", t.to_string()),
            Type::Reference(t, is_mut) => {
                if *is_mut {
                    format!("&mut {}", t.to_string())
                } else {
                    format!("&{}", t.to_string())
                }
            }
            Type::Pointer(t) => format!("*{}", t.to_string()),
            Type::Channel(t) => format!("chan {}", t.to_string()),
            Type::Function(params, ret) => {
                let p_str = params
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) -> {}", p_str, ret.to_string())
            }
            Type::Struct(name) => name.clone(),
            Type::Trait(name) => format!("trait {}", name),
            Type::Enum(name) => name.clone(),
            Type::Union(name) => name.clone(),
            Type::Any => "any".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub package: String,
    pub imports: Vec<String>,
    pub items: Vec<TopLevel>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Function(FunctionDef),
    Struct(StructDef),
    Trait(TraitDef),
    Impl(ImplBlock),
    Enum(EnumDef),
    Const(ConstDef),
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub is_mutable: bool,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStmt),
    Const(ConstDef),
    Expression(Expression),
    If(IfStmt),
    For(ForStmt),
    Loop(Block),
    Break,
    Continue,
    Return(Option<Box<Expression>>),
    Defer(Box<Statement>),
    Co(Block),            // concurrency block
    ChannelSend(Box<Expression>, Box<Expression>), // chan <- value
    Panic(Option<Box<Expression>>),
    Empty,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    pub value_type: Option<Type>,
    pub value: Expression,
    pub is_mutable: bool,
}

#[derive(Debug, Clone)]
pub struct ConstDef {
    pub name: String,
    pub const_type: Type,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Box<IfStmt>>, // for else-if chain
    pub else_simple: Option<Box<Block>>, // for plain else
}

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub init: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub methods: Vec<FunctionDef>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub for_type: String,
    pub methods: Vec<FunctionDef>,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub data: Option<Type>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
    Unary(UnaryOp, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    MemberAccess(Box<Expression>, String),
    Index(Box<Expression>, Box<Expression>),
    Array(Vec<Expression>),
    Struct(String, Vec<(String, Expression)>),
    If(Box<IfStmt>),
    Block(Block),
    Cast(Box<Expression>, Type),
    Reference(Box<Expression>, bool), // (expr, is_mutable)
    Dereference(Box<Expression>),
    ChannelReceive(Box<Expression>), // <- chan
    Match(Box<Expression>, Vec<MatchArm>),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: String,
    pub body: Expression,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Not,
    Negate,
    BitwiseNot,
    Reference,
    Dereference,
}

impl BinaryOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::Equal | BinaryOp::NotEqual => 3,
            BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual => 4,
            BinaryOp::BitwiseOr => 5,
            BinaryOp::BitwiseXor => 6,
            BinaryOp::BitwiseAnd => 7,
            BinaryOp::LeftShift | BinaryOp::RightShift => 8,
            BinaryOp::Add | BinaryOp::Subtract => 9,
            BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => 10,
        }
    }
}
