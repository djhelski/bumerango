# Bumerango Compiler Architecture

## High-Level Overview

```
┌─────────────────────────────────────────────────────────────┐
│                   Bumerango Source Code                     │
│                      (.bume files)                          │
└────────────────────────┬────────────────────────────────────┘
                         │
                    ┌────▼─────┐
                    │  LEXER    │  ← src/lexer.rs (~350 lines)
                    │           │
                    │ Reads chars, produces tokens
                    └────┬──────┘
                         │
                    ┌────▼─────────────────┐
                    │   Token Stream       │
                    │ Vec<Token>           │
                    └────┬─────────────────┘
                         │
                    ┌────▼────────┐
                    │   PARSER    │  ← src/parser.rs (~750 lines)
                    │             │
                    │ Recursive descent parsing
                    │ Produces AST
                    └────┬────────┘
                         │
                    ┌────▼──────────────────────┐
                    │  Abstract Syntax Tree     │
                    │ (AST)                     │
                    │ Program { items: [...] }  │
                    └────┬─────────────────────┘
                         │
          ┌──────────────┼──────────────┐
          │              │              │
          ▼              ▼              ▼
    ┌─────────────┐ ┌──────────┐ ┌──────────────┐
    │   TYPE      │ │  BORROW  │ │ VALIDATION  │
    │  CHECKER    │ │ CHECKER  │ │ (v0.2+)     │
    └─────────────┘ └──────────┘ └──────────────┘
          │              │              │
          └──────────────┼──────────────┘
                         │
                    ┌────▼────────────┐
                    │    CODEGEN      │  ← src/codegen.rs (~400 lines)
                    │                 │
                    │ AST → LLVM IR
                    └────┬───────────┘
                         │
                    ┌────▼────────────────────┐
                    │   LLVM IR (.ll files)   │
                    │                         │
                    │ Optimizable, portable   │
                    └────┬───────────────────┘
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
         ┌────────────┐      ┌─────────────┐
         │  llc tool  │      │  Other LLVM │
         │(external)  │      │  Backends   │
         └────┬───────┘      └─────────────┘
              │
         ┌────▼──────────────┐
         │   Assembly (.s)   │
         │   Object files    │
         │   Native Binary   │
         └───────────────────┘
```

---

## Module Breakdown

### 1. **lexer.rs** — Tokenization

**Purpose:** Convert source code characters into structured tokens.

**Key Components:**

```rust
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    
    // Keywords
    Fn, Let, If, Co, Chan, ...
    
    // Operators
    Plus, Minus, EqualEqual, ...
    
    // Delimiters
    LeftBrace, RightParen, ...
}

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
```

**Algorithm:**

```
for each character:
  skip whitespace
  skip comments
  if digit: read_number()
  if alpha: read_identifier() → check keyword
  if quote: read_string()
  if operator: recognize operator token
  else: emit token or error
```

**Performance:** O(n) where n = source length

**Examples:**

```
Input:  "let x = 42;"
Output: [Let, Identifier("x"), Equal, Integer(42), Semicolon, Eof]
```

---

### 2. **parser.rs** — Syntax Analysis

**Purpose:** Convert token stream into Abstract Syntax Tree (AST).

**Key Components:**

```rust
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    // Recursive descent parsing methods:
    pub fn parse() -> Result<Program, String>
    fn parse_function() -> Result<FunctionDef, String>
    fn parse_expression() -> Result<Expression, String>
    fn parse_statement() -> Result<Statement, String>
    fn parse_block() -> Result<Block, String>
    
    // Helper methods:
    fn current_token() -> &Token
    fn advance() -> Token
    fn expect(TokenType) -> Result<Token, String>
}
```

**Parsing Strategy:** Recursive Descent

```
parse()
├── expect(pkg keyword)
├── parse package name
├── parse imports
└── parse top-level items
    ├── parse_function()
    ├── parse_struct()
    ├── parse_trait()
    └── parse_impl()

parse_expression() [Precedence Climbing]
├── parse_or_expression()     // Lowest precedence
├── parse_and_expression()
├── parse_equality_expression()
├── parse_relational_expression()
├── parse_additive_expression()
├── parse_multiplicative_expression()
├── parse_unary_expression()
└── parse_postfix_expression() // Highest precedence
    ├── function calls
    ├── member access
    └── index access
```

**Complexity:** O(n) where n = token count

**Example:**

```
Input:  [Let, Identifier("x"), Equal, Integer(42), Semicolon]

1. parse_statement() detects Let
2. parse_let_statement()
   - Expect Identifier: "x"
   - Expect Equal
   - parse_expression() → Integer(42)
   - Expect Semicolon

Output: LetStmt { name: "x", value_type: None, value: Integer(42) }
```

---

### 3. **ast.rs** — Abstract Syntax Tree Definitions

**Purpose:** Define all AST node types.

**Key Data Structures:**

```rust
pub struct Program {
    pub package: String,
    pub imports: Vec<String>,
    pub items: Vec<TopLevel>,
}

pub enum TopLevel {
    Function(FunctionDef),
    Struct(StructDef),
    Trait(TraitDef),
    Impl(ImplBlock),
    Enum(EnumDef),
    Const(ConstDef),
}

pub enum Statement {
    Let(LetStmt),
    Const(ConstDef),
    Expression(Expression),
    If(IfStmt),
    For(ForStmt),
    Loop(Block),
    Co(Block),           // Concurrency
    ChannelSend(Box<Expression>, Box<Expression>),
    Return(Option<Box<Expression>>),
    Break, Continue,
    Panic(Option<Box<Expression>>),
}

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
    // ... more variants
}

pub enum Type {
    I32, I64, F64, Bool, String,
    Array(Box<Type>),
    Reference(Box<Type>, bool),  // is_mutable
    Pointer(Box<Type>),
    Channel(Box<Type>),
    Function(Vec<Type>, Box<Type>),  // (params, return)
    Struct(String),
    // ... more variants
}
```

**Design Pattern:** Visitor pattern (used by codegen)

```rust
// In codegen:
match expression {
    Expression::Integer(n) => format!("{}", n),
    Expression::Binary(op, left, right) => { ... },
    // Process each variant
}
```

---

### 4. **codegen.rs** — Code Generation

**Purpose:** Convert AST to LLVM Intermediate Representation (IR).

**Key Components:**

```rust
pub struct Codegen {
    functions: String,        // Accumulated IR code
    strings: Vec<String>,     // String constants
    variables: HashMap<String, String>,  // Variable mappings
    var_counter: usize,
    temp_counter: usize,
}

impl Codegen {
    pub fn generate(program: &Program) -> String
    fn generate_function(&mut self, func: &FunctionDef, output: &mut String)
    fn generate_statement(&mut self, stmt: &Statement, output: &mut String)
    fn generate_expression(&mut self, expr: &Expression) -> String
    fn generate_binary_op(&mut self, op: BinaryOp, ...) -> String
    fn type_to_llvm(&self, t: &Type) -> String
}
```

**LLVM IR Generation Process:**

```
1. Emit module header
   - target datalayout
   - target triple

2. Collect string constants
   - Find all strings in program
   - Emit as global constants

3. Declare external functions
   - printf, scanf, etc.

4. For each function:
   - Emit signature
   - Generate IR for body
   - Track variables with SSA form
   - Emit return

5. Finalize module
```

**Example Transformation:**

```bumerango
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
```

↓

```llvm
define i32 @add(i32 %x, i32 %y) {
  %tmp.0 = add i32 %x, %y
  ret i32 %tmp.0
}
```

**Key Features:**

- **SSA Form:** Each value assigned once (`%tmp.0`, `%tmp.1`, ...)
- **Temp Management:** Automatic temporary variable generation
- **Type Mapping:**
  - `i32` → `i32`
  - `str` → `i8*` (pointer to chars)
  - `[]T` → array type
  - References → pointers

---

### 5. **lib.rs** — Public API

**Purpose:** Export compiler functions for library use.

```rust
pub fn compile(source: &str) -> Result<String, String> {
    let mut parser = Parser::new(source);
    let program = parser.parse()?;
    
    let mut codegen = Codegen::new();
    let llvm_ir = codegen.generate(&program);
    
    Ok(llvm_ir)
}
```

**Usage:**

```rust
// As a library
use bumerango::compile;

let source = "fn main() { let x = 42; }";
match compile(source) {
    Ok(ir) => println!("{}", ir),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

### 6. **main.rs** — CLI Tool

**Purpose:** Command-line interface for the compiler.

**Commands:**

```
bumerang compile <file>     # Compile to LLVM IR
bumerang repl               # Interactive shell
bumerang version            # Show version
bumerang help               # Show help
```

**Workflow:**

```
1. Parse arguments
2. Load source file
3. Call compile() function
4. Write LLVM IR to .ll file
5. Optionally invoke llc for further compilation
```

---

## Data Flow Example

### Complete Compilation of: `fn main() { let x = 42; }`

#### Stage 1: Lexer Output

```
Token { type: Fn, line: 1, col: 1 }
Token { type: Identifier("main"), line: 1, col: 4 }
Token { type: LeftParen, line: 1, col: 8 }
Token { type: RightParen, line: 1, col: 9 }
Token { type: LeftBrace, line: 1, col: 11 }
Token { type: Let, line: 1, col: 13 }
Token { type: Identifier("x"), line: 1, col: 17 }
Token { type: Equal, line: 1, col: 19 }
Token { type: Integer(42), line: 1, col: 21 }
Token { type: Semicolon, line: 1, col: 23 }
Token { type: RightBrace, line: 1, col: 24 }
Token { type: Eof, line: 1, col: 25 }
```

#### Stage 2: Parser Output (AST)

```
Program {
  package: "main",
  imports: [],
  items: [
    Function {
      name: "main",
      params: [],
      return_type: Any,
      body: Block {
        statements: [
          Let(
            LetStmt {
              name: "x",
              value_type: None,
              value: Integer(42),
              is_mutable: false,
            }
          )
        ]
      },
      is_public: false,
    }
  ]
}
```

#### Stage 3: Codegen Output (LLVM IR)

```llvm
; Bumerango → LLVM IR
; Package: main
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define void @main() {
  %var.0 = alloca i64
  store i64 42, i64* %var.0
  ret void
}
```

#### Stage 4: Machine Code (via llc)

```asm
_main:
  push rbp
  mov rbp, rsp
  mov rax, 42
  mov qword ptr [rbp - 8], rax
  xor eax, eax
  pop rbp
  ret
```

---

## Compilation Phases (Future)

### v0.2: Type Checking Phase

```
AST (untyped)
    ↓
Type Inference
    ↓
Type Checking
    ↓
Typed AST
    ↓
Borrow Checking
    ↓
Validated AST
    ↓
Codegen
```

### v0.3: Optimization Phase

```
LLVM IR (unoptimized)
    ↓
Constant Folding
    ↓
Dead Code Elimination
    ↓
Inline Functions
    ↓
Loop Optimizations
    ↓
LLVM IR (optimized)
```

---

## Error Handling Strategy

### Current (v0.1)

- **Lexer:** Report line/column of unexpected character
- **Parser:** Report expected vs. actual token
- **Codegen:** Panic on unsupported constructs

**Example Error:**

```
Error: Expected ":", got "i32" at line 1, column 15
```

### Future (v0.2+)

- **Error Recovery:** Continue parsing after errors
- **Error Messages:** Show code context + suggestions
- **Severity Levels:** Warning, Error, Fatal

```
error[E001]: Type mismatch
  ┌─ main.bume:5:8
  │
5 │ let x: i32 = 3.14;
  │         ^^^ expected i32, found f64
  │
  = help: use type annotation: let x: f64 = 3.14;
```

---

## Performance Characteristics

| Component | Complexity | Input Size | Time |
|-----------|-----------|-----------|------|
| Lexer | O(n) | Source chars | ~1ms per KB |
| Parser | O(n) | Tokens | ~5ms per KB |
| Codegen | O(n) | AST nodes | ~10ms per KB |
| **Total** | **O(n)** | **Source** | **~20ms per KB** |

**Example:** 10KB source → ~200ms compile time (single-threaded)

---

## Memory Layout

### During Compilation

```
Stack (LLVM Managed)
├── Parser position pointers
├── Current token
└── Recursive call stack

Heap (Allocated)
├── Source code string
├── Tokens vector (~100 bytes each)
├── AST nodes (variable size)
└── String constant pool
```

### Output

```
LLVM Module in Memory
├── Global string constants
├── Function signatures
├── Function bodies (SSA form)
└── Module metadata
```

---

## Testing Architecture

### Unit Tests

```rust
#[test]
fn test_lexer_keywords() { ... }

#[test]
fn test_parser_function() { ... }

#[test]
fn test_codegen_arithmetic() { ... }
```

### Integration Tests

```bash
./test.sh  # Compile examples and verify output
```

---

## Extensibility Points

### Adding New Keywords

1. Add to `TokenType` enum in `lexer.rs`
2. Add to keyword recognition in `Lexer::keyword_or_identifier()`
3. Add parsing logic in `parser.rs`
4. Add AST variant in `ast.rs`
5. Add codegen in `codegen.rs`

### Adding New Operators

1. Add to `TokenType` enum
2. Add tokenization in `Lexer::next_token()`
3. Add to operator precedence table in `Parser`
4. Add to `BinaryOp` enum in `ast.rs`
5. Add code generation in `Codegen`

### Adding New Statements

1. Define `Statement::NewVariant(...)` in `ast.rs`
2. Add parsing in `parser.rs`
3. Add codegen in `codegen.rs`

---

## Dependency Graph

```
main.rs
├── lib.rs
│   ├── parser.rs
│   │   ├── ast.rs
│   │   └── lexer.rs
│   │       └── (no internal deps)
│   ├── codegen.rs
│   │   └── ast.rs
│   └── ast.rs
└── external crates (none required for core compiler)

Test dependencies:
├── lexer tests
├── parser tests
└── integration tests
```

---

## Build Optimization

### Debug Build (`cargo build`)
- Full debug symbols
- Minimal optimization
- Compile time: ~5 seconds

### Release Build (`cargo build --release`)
- Strip symbols, optimize
- Full optimizations
- Compile time: ~15 seconds
- Binary size: ~2-5 MB (with LLVM linkage)

---

**Architecture Document Version:** 0.1  
**Last Updated:** May 2026  
**Status:** Complete for v0.1, ready for v0.2 additions
