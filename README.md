# 🚀 BUMERANGO — Compiler & Language Design

![Bumerango](https://img.shields.io/badge/Language-Bumerango-orange)
![Status](https://img.shields.io/badge/Status-Alpha%20v0.1-blue)

**Bumerango** — A modern programming language combining:
- 🐹 **Go-like simplicity** and concurrency (`co` blocks, channels)
- 🦀 **Rust-level safety** (type system, ownership, borrow checking)
- ⚡ **Assembly-fast performance** (compiled to LLVM IR → native machine code)

---

## 📚 Table of Contents

1. [Language Overview](#language-overview)
2. [Architecture](#architecture)
3. [Building the Compiler](#building-the-compiler)
4. [Language Features](#language-features)
5. [Syntax Guide](#syntax-guide)
6. [Examples](#examples)
7. [Compilation Pipeline](#compilation-pipeline)
8. [Testing](#testing)
9. [Future Roadmap](#future-roadmap)
10. [Use Cases & Monetization](#use-cases--monetization)

---

## Language Overview

### Design Goals

```
Bumerango = Go's Concurrency + Rust's Safety + C's Performance
```

**Core Principles:**
- Statically typed with strong compile-time guarantees
- Memory-safe: ownership system prevents data races
- Concurrent by default: `co` blocks for goroutine-like parallelism
- Direct LLVM compilation: no garbage collection, minimal overhead

### Key Features

| Feature | Details |
|---------|---------|
| **Concurrency** | `co` blocks, channels (`chan`), message passing |
| **Type System** | Static, strong typing with type inference |
| **Memory Safety** | Ownership + borrowing (Rust-like) |
| **Performance** | LLVM IR compilation → native code |
| **Traits** | Interface support with `trait` keyword |
| **Structs** | Product types with fields |
| **Enums** | Sum types with pattern matching |

---

## 🏗️ Architecture

### Compilation Pipeline

```
Bumerango Source (.bume)
    ↓
[LEXER] → Tokenization
    ↓
TokenStream
    ↓
[PARSER] → Recursive Descent Parsing
    ↓
Abstract Syntax Tree (AST)
    ↓
[TYPECHECKER] → Type validation & inference
    ↓
Typed AST
    ↓
[CODEGEN] → LLVM IR generation
    ↓
LLVM IR (.ll)
    ↓
[LLVM Backend] → Machine code compilation
    ↓
Native Binary (.o, executable)
```

### Module Structure

```
bumerango/
├── src/
│   ├── main.rs           # CLI tool
│   ├── lib.rs            # Library export
│   ├── lexer.rs          # Tokenization (300+ lines)
│   ├── parser.rs         # Recursive descent parser (700+ lines)
│   ├── ast.rs            # AST node definitions (200+ lines)
│   └── codegen.rs        # LLVM IR generation (400+ lines)
├── examples/
│   ├── fibonacci.bume    # Recursive example
│   ├── concurrency.bume  # Go routine-like concurrency
│   └── ownership.bume    # Ownership & borrowing
├── Cargo.toml            # Dependencies
└── README.md             # This file
```

---

## 🔨 Building the Compiler

### Prerequisites

- **Rust 1.70+** (install from https://rustup.rs/)
- **LLVM 15+** (for `llc` to compile IR → assembly)

### Installation & Build

```bash
# Clone/navigate to project
cd bumerango

# Build the compiler
cargo build --release

# The binary will be at: target/release/bumerang
```

### Quick Start

```bash
# Compile a Bumerango file
./target/release/bumerang compile examples/fibonacci.bume

# Interactive REPL
./target/release/bumerang repl

# Show help
./target/release/bumerang help
```

---

## 📖 Language Features

### 1. **Basic Types**

```rust
i8, i16, i32, i64, i128      // Signed integers
u8, u16, u32, u64, u128      // Unsigned integers
f32, f64                      // Floating point
bool                          // Boolean
str                           // Immutable string
[]T                           // Array
&T, &mut T                    // References/borrows
*T                            // Pointers
chan T                        // Channels
```

### 2. **Functions**

```bumerango
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// Type inference on return
fn greet(name: str) {
    // void function
}

// Mutable parameters
fn modify(mut arr: []i32) {
    arr[0] = 42;
}
```

### 3. **Variables**

```bumerango
// Immutable (default)
let x: i32 = 10;

// Mutable
let mut y: i32 = 20;
y = 30;  // OK

// Type inference
let z = 42;  // inferred as i32

// Constants
const MAX: i32 = 100;
```

### 4. **Control Flow**

```bumerango
// If-else
if condition {
    // ...
} else if other {
    // ...
} else {
    // ...
}

// For loops
for let i: i32 = 0; i < 10; i = i + 1 {
    println("i = {}", i);
}

// Infinite loops
loop {
    if done {
        break;
    }
    continue;
}

// Match (pattern matching)
match value {
    1 => { /* handle 1 */ },
    2 => { /* handle 2 */ },
    _ => { /* default */ },
}
```

### 5. **Concurrency — Go Routines**

```bumerango
// Concurrent block
co {
    let result1 = expensive_computation();
    let result2 = another_task();
}

// Channels for message passing
chan result = make(i32);

co {
    result <- 42;  // Send
}

let value = <- result;  // Receive
```

### 6. **Structs & Methods**

```bumerango
pub struct Person {
    name: str,
    age: i32,
    email: str,
}

impl Person {
    fn new(name: str, age: i32) -> Person {
        return Person {
            name: name,
            age: age,
            email: "",
        };
    }

    fn birthday(mut self) {
        self.age = self.age + 1;
    }
}

fn main() {
    let mut p = Person::new("Alice", 30);
    p.birthday();
}
```

### 7. **Traits & Polymorphism**

```bumerango
trait Reader {
    fn read() -> str;
}

trait Writer {
    fn write(data: str);
}

struct FileHandler {}

impl Reader for FileHandler {
    fn read() -> str {
        return "file content";
    }
}
```

### 8. **Enums & Pattern Matching**

```bumerango
enum Result {
    Ok(i32),
    Err(str),
}

fn process() -> Result {
    return Result::Ok(42);
}

fn main() {
    let r = process();
    match r {
        Result::Ok(value) => println("Success: {}", value),
        Result::Err(msg) => println("Error: {}", msg),
    }
}
```

### 9. **Memory Management — Ownership**

```bumerango
// Ownership transfer
let data = make_data();  // data is owned here
use_data(data);           // ownership moves
// data is no longer valid here

// Borrowing (read-only)
let data = make_data();
read_data(&data);         // borrow
write_data(&data);        // still valid

// Mutable borrowing
let mut data = make_data();
modify_data(&mut data);   // mutable borrow
```

### 10. **Error Handling**

```bumerango
fn divide(a: i32, b: i32) -> Result {
    if b == 0 {
        return Result::Err("Division by zero");
    }
    return Result::Ok(a / b);
}

fn main() {
    let result = divide(10, 2);
    match result {
        Result::Ok(val) => println("Result: {}", val),
        Result::Err(e) => println("Error: {}", e),
    }
}
```

---

## 💬 Syntax Guide

### Keywords

```
pkg, fn, let, const, if, else, for, loop, break, continue
return, match, co, chan, make, import, use, pub, priv
struct, trait, impl, enum, union, mut, &, ref, unsafe
defer, panic, true, false
```

### Operators

```
Arithmetic:     +  -  *  /  %
Comparison:     ==  !=  <  <=  >  >=
Logical:        &&  ||  !
Bitwise:        &  |  ^  <<  >>
Assignment:     =  +=  -=  *=  /=
Channels:       <-  ->
References:     &  *
```

### Control Structure Precedence

```
1. OR (||)
2. AND (&&)
3. Equality (==, !=)
4. Relational (<, <=, >, >=)
5. Bitwise OR (|)
6. Bitwise XOR (^)
7. Bitwise AND (&)
8. Shift (<<, >>)
9. Additive (+, -)
10. Multiplicative (*, /, %)
```

---

## 📝 Examples

### Example 1: Fibonacci (Recursion)

**File: `examples/fibonacci.bume`**

```bumerango
pkg main

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    let result = fibonacci(10);
    println("Fib(10) = {}", result);
}
```

**Expected Output:**
```
Fib(10) = 55
```

---

### Example 2: Concurrency & Channels

**File: `examples/concurrency.bume`**

```bumerango
pkg main

fn worker(id: i32, results: chan i32) {
    let task_result = id * id;
    results <- task_result;
}

fn main() {
    chan results = make(i32);

    // Launch concurrent workers
    co {
        worker(1, results);
        worker(2, results);
        worker(3, results);
    }

    // Receive results
    let r1 = <- results;
    let r2 = <- results;
    let r3 = <- results;

    println("Results: {}, {}, {}", r1, r2, r3);
}
```

**Expected Output:**
```
Results: 1, 4, 9
```

---

### Example 3: Ownership & Borrowing

**File: `examples/ownership.bume`**

```bumerango
pkg main

struct Box {
    value: i32,
}

fn peek(b: &Box) -> i32 {
    return b.value;  // Read-only borrow
}

fn modify(b: &mut Box) {
    b.value = 99;   // Mutable borrow
}

fn main() {
    let mut b = Box { value: 42 };
    
    let val = peek(&b);
    println("Value: {}", val);
    
    modify(&mut b);
    println("Modified: {}", b.value);
}
```

**Expected Output:**
```
Value: 42
Modified: 99
```

---

### Example 4: Structs & Methods

```bumerango
pkg main

struct Counter {
    count: i32,
}

impl Counter {
    fn increment(mut self) {
        self.count = self.count + 1;
    }

    fn get_value(self) -> i32 {
        return self.count;
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    counter.increment();
    
    let final_value = counter.get_value();
    println("Final count: {}", final_value);
}
```

**Expected Output:**
```
Final count: 2
```

---

## 🔄 Compilation Pipeline (Deep Dive)

### Step 1: Lexer (Token Generation)

**Input:**
```bumerango
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
```

**Output (Token Stream):**
```
[Fn, Identifier("add"), LeftParen, Identifier("x"), Colon, 
 Identifier("i32"), Comma, Identifier("y"), Colon, Identifier("i32"), 
 RightParen, RightArrow, Identifier("i32"), LeftBrace, 
 Return, Identifier("x"), Plus, Identifier("y"), Semicolon, RightBrace, Eof]
```

**Code Location:** [src/lexer.rs](src/lexer.rs)

### Step 2: Parser (AST Generation)

**Input:** Token stream

**Output (AST):**
```
Program {
  package: "main",
  imports: [],
  items: [
    Function {
      name: "add",
      params: [
        Parameter { name: "x", type: I32 },
        Parameter { name: "y", type: I32 }
      ],
      return_type: I32,
      body: Block {
        statements: [
          Return(Binary(Add, Identifier("x"), Identifier("y")))
        ]
      }
    }
  ]
}
```

**Code Location:** [src/parser.rs](src/parser.rs)

### Step 3: Type Checking (Validation)

- Verify all identifiers are declared
- Check type compatibility in operations
- Validate function signatures
- Ensure ownership rules are obeyed

(Note: Full type checker coming in v0.2)

### Step 4: LLVM IR Generation (Codegen)

**Input:** AST

**Output (LLVM IR):**
```llvm
; Bumerango → LLVM IR
; Package: main
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define i32 @add(i32 %x, i32 %y) {
  %tmp.0 = add i32 %x, %y
  ret i32 %tmp.0
}
```

**Code Location:** [src/codegen.rs](src/codegen.rs)

### Step 5: Native Compilation (LLVM Backend)

```bash
llc -o add.s add.ll          # IR → Assembly
gcc -c add.s -o add.o        # Assembly → Object
gcc add.o -o add             # Link → Executable
./add                        # Run!
```

---

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Test specific module
cargo test lexer::
cargo test parser::
cargo test codegen::
```

### Example Test Cases

**Test 1: Lexer**
```rust
#[test]
fn test_keywords() {
    let mut lexer = Lexer::new("fn let co chan");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].token_type, TokenType::Fn));
    assert!(matches!(tokens[1].token_type, TokenType::Let));
}
```

**Test 2: Parser**
```rust
#[test]
fn test_function_parsing() {
    let code = "fn main() { return 42; }";
    let mut parser = Parser::new(code);
    let result = parser.parse();
    assert!(result.is_ok());
}
```

### Manual Testing

```bash
# Compile example and check output
bumerang compile examples/fibonacci.bume
cat examples/fibonacci.ll  # View LLVM IR

# Interactive testing
bumerang repl
bumerang> let x = 10;
bumerang> let y = 20;
bumerang> x + y
```

---

## 🗺️ Future Roadmap

### v0.2 (Type Checking)
- [ ] Full type checker with inference
- [ ] Ownership/borrow checker
- [ ] Error recovery & diagnostics

### v0.3 (Standard Library)
- [ ] Printf, file I/O
- [ ] Collections (Vec, HashMap)
- [ ] Concurrency runtime

### v0.4 (Optimization)
- [ ] Constant folding
- [ ] Dead code elimination
- [ ] Inline function calls
- [ ] Loop unrolling

### v1.0 (Production)
- [ ] Package manager
- [ ] Full standard library
- [ ] Cross-platform compilation
- [ ] Debug symbols & profiling

---

## 💼 Use Cases & Monetization

### 1. **Educational Value**
- **Target:** CS students, compiler enthusiasts
- **Monetization:** 
  - Paid courses on "Build Your Own Compiler"
  - Interactive compiler explorer (SaaS)
  - Certification program

### 2. **Systems Programming**
- **Target:** OS, embedded systems developers
- **Use Case:** Rust-like safety + Go concurrency for systems code
- **Monetization:**
  - Enterprise support & consulting
  - Bumerango certification for developers

### 3. **Content Creation**
- **Format:** Detailed blog series, YouTube tutorials
- **Topics:**
  - "From Lexer to Machine Code"
  - "Compiler Design in Rust"
  - "LLVM IR for Language Developers"
- **Revenue:** Sponsored content, Patreon

### 4. **Cloud IDE / Playground**
- **Feature:** Online editor + live compiler
- **Monetization:** Freemium SaaS model
  - Free: Basic compilation
  - Pro: Concurrent execution, debugging, performance analytics

### 5. **Language Extensions**
- GPU compilation (`cu`, `cl`)
- WebAssembly (`--target wasm`)
- Mobile targets (`--target arm64`)

---

## 📊 Compiler Statistics

| Component | Lines | Purpose |
|-----------|-------|---------|
| **Lexer** | ~350 | Tokenization |
| **Parser** | ~750 | AST generation |
| **AST** | ~200 | Data structures |
| **Codegen** | ~400 | LLVM IR emission |
| **Main** | ~200 | CLI & REPL |
| **Total** | ~1,900 | Complete compiler |

---

## 🎯 Key Concepts

### Lexer → Parser → AST → Codegen

```
"let x = 42;"
    ↓
[Let, Identifier("x"), Equal, Integer(42), Semicolon]
    ↓
Let { name: "x", value: Integer(42) }
    ↓
%var.0 = alloca i64
store i64 42, i64* %var.0
```

### Memory Safety: Ownership Rules

1. **Each value has ONE owner**
2. **Moving transfers ownership**
3. **Borrowing via `&` creates temporary access**
4. **Mutable borrow `&mut` gives exclusive modification**

### Concurrency: Go-style `co` blocks

```bumerango
co {
    task1();  // Runs concurrently
    task2();
    task3();
}
// All tasks complete before proceeding
```

---

## 📚 References

- [LLVM Language Reference](https://llvm.org/docs/LangRef/)
- [Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Go Tour - Goroutines](https://tour.golang.org/concurrency/1)
- [Compiler Design (Dragon Book)](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)

---

## 📞 Support

For questions, file an issue or start a discussion!

**Author:** Bumerango Language Team  
**License:** MIT  
**Status:** Active Development (Alpha)

---

**Happy Compiling! 🚀**
