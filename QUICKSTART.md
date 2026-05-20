# Bumerango Quick Start Guide

## 🚀 Installation & Setup

### Prerequisites

- **OS:** Linux, macOS, or Windows (WSL2)
- **Rust:** 1.70+ ([Install](https://rustup.rs/))
- **LLVM:** 15+ (optional, for native compilation)

### Step 1: Clone/Setup

```bash
# Navigate to project
cd bumerango

# View structure
tree src examples

# Directory should look like:
# bumerango/
# ├── src/
# │   ├── main.rs
# │   ├── lib.rs
# │   ├── lexer.rs
# │   ├── parser.rs
# │   ├── ast.rs
# │   └── codegen.rs
# ├── examples/
# │   ├── fibonacci.bume
# │   ├── concurrency.bume
# │   ├── ownership.bume
# │   └── tests.bume
# ├── Cargo.toml
# ├── README.md
# ├── LANGUAGE_SPEC.md
# └── QUICKSTART.md (this file)
```

### Step 2: Build Compiler

```bash
# Build in release mode (optimized)
cargo build --release

# Binary location
ls -la target/release/bumerang
```

### Step 3: Test Installation

```bash
# Show version
./target/release/bumerang version

# Show help
./target/release/bumerang help

# Output should be:
# Bumerango - The Go-like, Rust-safe, Assembly-fast Language
#
# USAGE:
#     bumerang <COMMAND> [OPTIONS]
#
# COMMANDS:
#     compile <file>  - Compile Bumerango source to LLVM IR
#     repl            - Interactive REPL
#     version         - Show version
#     help            - Show this help
```

---

## 📝 Your First Program

### Example 1: Hello, World! (Arithmetic)

**File: `hello.bume`**

```bumerango
pkg main

fn main() {
    let x: i32 = 10;
    let y: i32 = 32;
    let result = x + y;
}
```

**Compile:**

```bash
./target/release/bumerang compile hello.bume
```

**Output:**

```
[*] Compiling: hello.bume
[✓] Success! LLVM IR written to: hello.ll

--- LLVM IR ---
; Bumerango → LLVM IR
; Package: main
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define void @main() {
  %tmp.0 = add i32 10, 32
  ret void
}
```

---

### Example 2: Fibonacci Function

**File: `fib.bume`**

```bumerango
pkg main

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    let result = fib(10);
}
```

**Compile:**

```bash
./target/release/bumerang compile examples/fibonacci.bume
```

**What You Get:**

1. **LLVM IR** in `fibonacci.ll`
2. **Readable intermediate representation** showing function structure
3. Compile further with `llc fibonacci.ll` to get assembly

---

### Example 3: Concurrency

**File: `concurrent.bume`**

```bumerango
pkg main

fn worker(id: i32) -> i32 {
    return id * 2;
}

fn main() {
    co {
        let r1 = worker(1);
        let r2 = worker(2);
        let r3 = worker(3);
    }
}
```

**Compile & Analyze:**

```bash
./target/release/bumerang compile concurrent.bume

# View LLVM IR
cat concurrent.ll
```

---

## 🔧 Using the REPL

**Interactive mode for quick testing:**

```bash
./target/release/bumerang repl
```

**REPL Session:**

```
=== Bumerango REPL v0.1.0 ===
Type 'exit' to quit, 'help' for commands

bumerang> let x = 42;
bumerang> x + 8
LLVM IR:
...

bumerang> fn add(a: i32, b: i32) -> i32 { return a + b; }
LLVM IR:
...

bumerang> exit
Goodbye!
```

---

## 📊 Compilation Pipeline Walkthrough

### Step-by-step what happens:

**Input: `main.bume`**
```bumerango
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
```

**Step 1: Lexer**
```
Tokens: [Fn, Identifier("add"), LeftParen, Identifier("x"), Colon, 
         Identifier("i32"), Comma, ...]
```

**Step 2: Parser**
```
AST: FunctionDef {
    name: "add",
    params: [Parameter { name: "x", type: I32 }, ...],
    body: Block { statements: [Return(Binary(Add, ...))] }
}
```

**Step 3: Codegen**
```llvm
define i32 @add(i32 %x, i32 %y) {
  %tmp.0 = add i32 %x, %y
  ret i32 %tmp.0
}
```

**Step 4: LLVM IR File**
```bash
cat main.ll
```

**Step 5: Compile to Assembly** (with llc)
```bash
llc main.ll -o main.s
cat main.s
```

**Step 6: Link & Execute**
```bash
gcc main.s -o main
./main
```

---

## 🧪 Running Examples

### Test All Examples

```bash
# Fibonacci
./target/release/bumerang compile examples/fibonacci.bume

# Concurrency
./target/release/bumerang compile examples/concurrency.bume

# Ownership
./target/release/bumerang compile examples/ownership.bume

# Tests
./target/release/bumerang compile examples/tests.bume
```

### Verify Output

```bash
# Check LLVM IR
head -20 examples/fibonacci.ll

# See full IR
cat examples/fibonacci.ll

# Count lines
wc -l examples/fibonacci.ll
```

---

## 🐛 Debugging Tips

### Enable Verbose Output

Currently, the compiler outputs:
- Compilation status
- LLVM IR (always printed)
- Success/error messages

### Common Errors

**Error: "Expected identifier after 'pkg'"**
```bumerango
// ❌ WRONG
pkg ;

// ✅ CORRECT
pkg main
```

**Error: "Unexpected token in expression"**
```bumerango
// ❌ WRONG
let x = ;

// ✅ CORRECT
let x = 42;
```

**Error: "Type mismatch"** (coming in v0.2)
```bumerango
// ❌ WRONG
let x: i32 = 3.14;

// ✅ CORRECT
let x: f64 = 3.14;
```

---

## 📚 Language Features Quick Reference

### Variables

```bumerango
let x = 42;              // Immutable, inferred type
let mut y = 10;          // Mutable
let z: i32 = 20;         // Explicit type
const MAX: i32 = 100;    // Constant
```

### Functions

```bumerango
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn no_return() {
    // No return statement
}

pub fn public_func() {
    // Visible outside this module
}
```

### Control Flow

```bumerango
// If-else
if x > 0 {
    println("Positive");
} else if x < 0 {
    println("Negative");
} else {
    println("Zero");
}

// For loop
for let i: i32 = 0; i < 10; i = i + 1 {
    // ...
}

// Infinite loop
loop {
    if condition {
        break;
    }
    continue;
}
```

### Concurrency

```bumerango
// Go-style concurrent block
co {
    task1();
    task2();
    task3();
}

// Channels
chan result = make(i32);

co {
    result <- 42;
}

let value = <- result;
```

### Structs & Methods

```bumerango
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(self) -> i32 {
        return self.x + self.y;
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    let d = p.distance();
}
```

### References & Borrowing

```bumerango
fn read(data: &str) {
    // Immutable borrow
}

fn modify(data: &mut i32) {
    *data = 99;  // Dereference and modify
}

fn main() {
    let mut value = 10;
    modify(&mut value);
}
```

---

## 🔗 Next Steps

1. **Read the spec:** `LANGUAGE_SPEC.md`
2. **Study examples:** `examples/`
3. **Build something:** Create `myprogram.bume`
4. **Join community:** (Coming in v1.0)

---

## 📖 Documentation Map

| Document | Purpose |
|----------|---------|
| `README.md` | Overview, architecture, use cases |
| `LANGUAGE_SPEC.md` | Complete language specification |
| `QUICKSTART.md` | This file - get started quickly |
| `examples/` | Working code samples |
| `src/` | Compiler source code |

---

## 💡 Example: Build a Simple Calculator

**File: `calc.bume`**

```bumerango
pkg main

fn calculate(op: i32, a: i32, b: i32) -> i32 {
    if op == 1 {
        return a + b;  // Add
    } else if op == 2 {
        return a - b;  // Subtract
    } else if op == 3 {
        return a * b;  // Multiply
    } else if op == 4 {
        return a / b;  // Divide
    } else {
        return 0;
    }
}

fn main() {
    let add_result = calculate(1, 10, 5);      // 15
    let sub_result = calculate(2, 10, 5);      // 5
    let mul_result = calculate(3, 10, 5);      // 50
    let div_result = calculate(4, 10, 5);      // 2
}
```

**Compile:**

```bash
./target/release/bumerang compile calc.bume
cat calc.ll
```

---

## 🎓 Learning Path

### Level 1: Basics (30 min)
- [ ] Read this QUICKSTART
- [ ] Compile `fibonacci.bume`
- [ ] Understand lexer → parser → IR pipeline

### Level 2: Language Features (1 hour)
- [ ] Study `LANGUAGE_SPEC.md`
- [ ] Try `concurrency.bume`
- [ ] Try `ownership.bume`

### Level 3: Compiler Internals (2+ hours)
- [ ] Read source code: `src/lexer.rs`, `src/parser.rs`
- [ ] Trace compilation of a simple program
- [ ] Modify a parser rule and recompile

### Level 4: Extensions (Ongoing)
- [ ] Add type checking (v0.2)
- [ ] Implement borrow checker (v0.2)
- [ ] Build standard library (v0.3)

---

## ✅ Verification Checklist

After setup, verify everything works:

- [ ] `cargo build --release` succeeds
- [ ] `bumerang version` outputs "Bumerango v0.1.0"
- [ ] `bumerang compile examples/fibonacci.bume` generates `.ll` file
- [ ] `cat examples/fibonacci.ll` shows valid LLVM IR
- [ ] `bumerang repl` starts interactive shell
- [ ] Examples compile without errors

---

## 🆘 Troubleshooting

### "cargo command not found"
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "llvm-sys not found"
```bash
# Install LLVM development files (Ubuntu/Debian)
sudo apt-get install llvm-15-dev

# macOS
brew install llvm
```

### Compiler crashes with "thread panicked"
- Check Bumerango source file for syntax errors
- Try simpler example first
- File an issue on GitHub

---

## 🎯 Success Criteria

You've successfully set up Bumerango when:

1. ✅ You can compile `.bume` files
2. ✅ LLVM IR is generated correctly
3. ✅ You understand the pipeline: Source → Tokens → AST → IR
4. ✅ You can run examples
5. ✅ You can use the REPL

**Congratulations! You're ready to explore Bumerango! 🚀**

---

**Happy Coding!**
