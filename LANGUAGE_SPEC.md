# BUMERANGO Language Specification v0.1

## Table of Contents

1. [Lexical Structure](#lexical-structure)
2. [Grammar](#grammar)
3. [Type System](#type-system)
4. [Memory Model](#memory-model)
5. [Concurrency Model](#concurrency-model)
6. [Built-in Functions](#built-in-functions)
7. [Standard Library](#standard-library)

---

## Lexical Structure

### Comments

```bumerango
// Single-line comment
// Bumerango ignores text after //

/* Multi-line comments are not yet supported */
```

### Identifiers

```
identifier = letter { letter | digit | "_" }
letter = "a" ... "z" | "A" ... "Z" | "_"
digit = "0" ... "9"
```

**Valid:** `myVar`, `_private`, `value123`, `MAX_VALUE`
**Invalid:** `123start`, `my-var`, `my var`

### Literals

#### Integer Literals
```bumerango
let decimal = 42;
let hex = 0xFF;
let binary = 0b1010;
let octal = 0o77;
```

#### Float Literals
```bumerango
let pi = 3.14;
let scientific = 1.23e-4;
let negative = -99.99;
```

#### String Literals
```bumerango
let hello = "Hello, World!";
let escape = "Line 1\nLine 2\tTabbed";
let quote = "She said \"Hi\"";
```

#### Boolean Literals
```bumerango
let yes = true;
let no = false;
```

### Keywords

```
pkg      import    use       pub       priv
fn       return    defer     let       const       mut
struct   trait     impl      enum      union
if       else      for       loop      break       continue
match    co        chan      make      <- (recv)   -> (send)
true     false     panic     ref       unsafe
```

### Operators

#### Arithmetic
```
+       // Addition
-       // Subtraction
*       // Multiplication
/       // Division
%       // Modulo
```

#### Comparison
```
==      // Equal
!=      // Not equal
<       // Less than
<=      // Less than or equal
>       // Greater than
>=      // Greater than or equal
```

#### Logical
```
&&      // AND
||      // OR
!       // NOT
```

#### Bitwise
```
&       // AND
|       // OR
^       // XOR
<<      // Left shift
>>      // Right shift
```

#### Assignment
```
=       // Assign
+=      // Add-assign
-=      // Subtract-assign
*=      // Multiply-assign
/=      // Divide-assign
```

#### Memory
```
&       // Address-of (reference)
*       // Dereference / Pointer
<-      // Channel receive
->      // Channel send / Function return
```

---

## Grammar

### Program Structure

```
program         = package_decl { import_stmt } { top_level_item }
package_decl    = "pkg" identifier
import_stmt     = "import" string_literal
top_level_item  = function_def | struct_def | trait_def | impl_def | enum_def | const_def
```

### Function Definition

```
function_def    = [ "pub" ] "fn" identifier "(" [ parameters ] ")" [ "->" type ] block
parameters      = parameter { "," parameter }
parameter       = [ "mut" ] identifier ":" type
block           = "{" { statement } "}"
```

**Example:**
```bumerango
pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
```

### Variable Declaration

```
let_stmt        = "let" [ "mut" ] identifier [ ":" type ] "=" expression ";"
const_stmt      = "const" identifier ":" type "=" expression ";"
```

**Examples:**
```bumerango
let x = 10;                  // Type inferred
let y: i32 = 20;             // Explicit type
let mut z: i32 = 30;         // Mutable
const MAX: i32 = 100;        // Constant
```

### Statements

```
statement       = let_stmt
                | const_stmt
                | if_stmt
                | for_stmt
                | loop_stmt
                | match_stmt
                | return_stmt
                | break_stmt
                | continue_stmt
                | defer_stmt
                | co_stmt
                | panic_stmt
                | expression_stmt

expression_stmt = expression ";"
```

### Control Flow

#### If Statement
```
if_stmt         = "if" expression block { "else" "if" expression block } [ "else" block ]
```

```bumerango
if x > 0 {
    println("Positive");
} else if x < 0 {
    println("Negative");
} else {
    println("Zero");
}
```

#### For Loop
```
for_stmt        = "for" [ init ] ";" [ condition ] ";" [ update ] block
```

```bumerango
for let i: i32 = 0; i < 10; i = i + 1 {
    println("i = {}", i);
}
```

#### Loop (Infinite)
```
loop_stmt       = "loop" block
```

```bumerango
loop {
    if condition {
        break;
    }
}
```

#### Match (Pattern Matching)
```
match_stmt      = "match" expression "{" match_arm { match_arm } "}"
match_arm       = pattern "=>" expression ","
```

```bumerango
match status {
    0 => { println("Success"); },
    1 => { println("Error"); },
    _ => { println("Unknown"); },
}
```

#### Co (Concurrent Block)
```
co_stmt         = "co" block
```

```bumerango
co {
    task1();
    task2();
    task3();
}
// Wait for all tasks to complete
```

### Types

#### Primitive Types
```
i8, i16, i32, i64, i128        // Signed integers
u8, u16, u32, u64, u128        // Unsigned integers
f32, f64                        // Floating point
bool                            // Boolean
str                             // String
```

#### Composite Types
```
type            = primitive_type
                | array_type
                | reference_type
                | pointer_type
                | channel_type
                | struct_type
                | trait_type
                | function_type

array_type      = "[" "]" type
reference_type  = "&" [ "mut" ] type
pointer_type    = "*" type
channel_type    = "chan" type
```

#### Type Aliases
```bumerango
// Custom types via struct
struct UserId {
    id: i32,
}

// Or through trait definitions
trait Comparable {
    fn compare(other: &Comparable) -> i32;
}
```

### Expressions

```
expression      = or_expr

or_expr         = and_expr { "||" and_expr }
and_expr        = equality_expr { "&&" equality_expr }
equality_expr   = relational_expr { ( "==" | "!=" ) relational_expr }
relational_expr = additive_expr { ( "<" | "<=" | ">" | ">=" ) additive_expr }
additive_expr   = multiplicative_expr { ( "+" | "-" ) multiplicative_expr }
multiplicative_expr = unary_expr { ( "*" | "/" | "%" ) unary_expr }

unary_expr      = [ ( "!" | "-" | "&" | "*" ) ] postfix_expr

postfix_expr    = primary_expr
                  { ( call | member_access | index_access ) }

call            = "(" [ arguments ] ")"
member_access   = "." identifier
index_access    = "[" expression "]"

primary_expr    = identifier
                | literal
                | "(" expression ")"
                | array_literal
                | struct_literal
                | if_expr
                | match_expr
                | block_expr
```

### Type Annotations

```
identifier : type          // Explicit type
```

**Examples:**
```bumerango
let x: i32 = 10;
fn foo(a: i32, b: str) -> bool { ... }
```

---

## Type System

### Type Inference

Bumerango uses **bidirectional type inference**:

```bumerango
let x = 42;              // Inferred: i32
let y = 3.14;            // Inferred: f64
let z: i32 = 10;         // Explicit: i32
```

### Type Coercion

Automatic coercion is **limited** (Rust-like):

```bumerango
let a: i32 = 42;
let b: i64 = a;          // ERROR: no implicit coercion

// Explicit casting
let b: i64 = a as i64;   // OK (planned for v0.2)
```

### Generic Types (Planned for v0.3)

```bumerango
struct Vector<T> {
    elements: []T,
    capacity: i32,
}

fn push<T>(v: &mut Vector<T>, item: T) {
    // ...
}
```

---

## Memory Model

### Ownership

Every value has **exactly one owner**:

```bumerango
let data = create_data();    // data owns the value
let other = data;            // Ownership moves to other
// data is no longer valid
println(data);               // ERROR
```

### Borrowing

Temporary access without taking ownership:

```bumerango
let data = create_data();
use(&data);                  // Immutable borrow
use(&data);                  // Another immutable borrow (OK)
use(&mut data);              // ERROR: can't mut borrow while immutable borrows exist
```

### Stack vs Heap

- **Stack:** Primitives (`i32`, `bool`, etc.), small fixed-size types
- **Heap:** Strings, arrays, custom structs, channels

### Memory Safety Guarantees

1. **No null pointer dereferences** (Option type in v0.2)
2. **No data races** (borrow checker enforces exclusive mutable access)
3. **No use-after-free** (ownership system)
4. **No double-free** (RAII with ownership)

---

## Concurrency Model

### Go-style Concurrency

#### Co Blocks (Goroutines)

```bumerango
co {
    do_work_1();
    do_work_2();
    do_work_3();
}
// Blocks until all tasks complete (structured concurrency)
```

#### Channels

```bumerango
// Create channel
chan messages = make(str);

// Send (in concurrent block)
co {
    messages <- "Hello";
    messages <- "World";
}

// Receive
let msg1 = <- messages;
let msg2 = <- messages;
```

#### Synchronization

```bumerango
// Channels provide synchronization
chan result = make(i32);

co {
    // Worker computes
    result <- expensive_computation();
}

// Main thread waits for result
let value = <- result;
```

### Channel Operations

```
chan T              // Declare channel
make(T)             // Create channel
value <- chan       // Receive
chan <- value       // Send
```

**Example:**

```bumerango
fn worker(id: i32, tasks: chan i32, results: chan i32) {
    loop {
        match <- tasks {
            -1 => { break; },  // Exit signal
            task => {
                let result = task * 2;
                results <- result;
            }
        }
    }
}

fn main() {
    chan tasks = make(i32);
    chan results = make(i32);

    // Spawn workers
    co {
        worker(1, tasks, results);
        worker(2, tasks, results);
    }

    // Send tasks
    for let i: i32 = 1; i <= 10; i = i + 1 {
        tasks <- i;
    }

    // Collect results
    for let i: i32 = 1; i <= 10; i = i + 1 {
        let result = <- results;
        println("Result: {}", result);
    }
}
```

---

## Built-in Functions

### I/O

```bumerango
fn println(format: str, args: ...) -> void;
fn print(format: str, args: ...) -> void;
fn input() -> str;
```

**Example:**
```bumerango
println("Value: {}", 42);
println("Name: {}, Age: {}", "Alice", 30);
```

### Memory

```bumerango
fn make<T>(capacity: i32) -> chan T;  // Create channel
fn len<T>(arr: []T) -> i32;           // Array length
fn cap<T>(arr: []T) -> i32;           // Capacity
```

### Type Operations

```bumerango
fn sizeof<T>() -> i32;                // Size in bytes
```

---

## Standard Library

### Planned Modules (v0.3+)

```
std.io      - Input/output
std.fs      - Filesystem
std.net     - Networking
std.time    - Time operations
std.math    - Mathematical functions
std.sync    - Synchronization primitives
std.collections - Data structures (Vec, HashMap, etc.)
std.concurrent - Advanced concurrency utilities
```

**Example (Planned):**

```bumerango
import "std.io"
import "std.collections"

fn main() {
    let mut numbers = io.read_numbers();
    let sum = collections.sum(numbers);
    io.println("Sum: {}", sum);
}
```

---

## Error Handling (v0.2+)

### Result Type

```bumerango
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: i32, b: i32) -> Result<i32, str> {
    if b == 0 {
        return Result::Err("Division by zero");
    }
    return Result::Ok(a / b);
}

fn main() {
    match divide(10, 2) {
        Result::Ok(value) => println("Result: {}", value),
        Result::Err(msg) => println("Error: {}", msg),
    }
}
```

### Option Type

```bumerango
enum Option<T> {
    Some(T),
    None,
}

fn get_user(id: i32) -> Option<str> {
    if id > 0 {
        return Option::Some("User found");
    }
    return Option::None;
}
```

---

## Operator Precedence (Highest to Lowest)

| Precedence | Operators |
|-----------|-----------|
| 1 | `()`, `[]`, `.` (call, index, member access) |
| 2 | `!`, `-`, `&`, `*` (unary) |
| 3 | `*`, `/`, `%` (multiplicative) |
| 4 | `+`, `-` (additive) |
| 5 | `<<`, `>>` (shift) |
| 6 | `&` (bitwise AND) |
| 7 | `^` (bitwise XOR) |
| 8 | `\|` (bitwise OR) |
| 9 | `<`, `<=`, `>`, `>=` (relational) |
| 10 | `==`, `!=` (equality) |
| 11 | `&&` (logical AND) |
| 12 | `\|\|` (logical OR) |
| 13 | `=`, `+=`, `-=`, etc. (assignment) |

---

## Planned Features (v0.2+)

### Generics

```bumerango
struct Pair<T, U> {
    first: T,
    second: U,
}

fn swap<T>(a: T, b: T) -> (T, T) {
    return (b, a);
}
```

### Lifetimes (Rust-style)

```bumerango
fn first<'a>(s: &'a str) -> &'a str {
    return s;
}
```

### Advanced Pattern Matching

```bumerango
match value {
    1 | 2 | 3 => { /* handle 1, 2, 3 */ },
    4..=10 => { /* range */ },
    User { name: "Alice", .. } => { /* destructure */ },
    _ => { /* default */ },
}
```

### Macro System

```bumerango
macro assert(condition) {
    if !condition {
        panic("Assertion failed");
    }
}
```

---

## Philosophy

> **Bumerango combines three philosophies:**
>
> 1. **Go's Simplicity:** Easy to read, write, and reason about
> 2. **Rust's Safety:** Strong guarantees at compile-time
> 3. **C's Performance:** Minimal overhead, direct hardware access

---

## Examples by Feature

### Fibonacci (Recursion)
```bumerango
fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
```

### Factorial (Iteration)
```bumerango
fn factorial(n: i32) -> i32 {
    let mut result: i32 = 1;
    for let i: i32 = 2; i <= n; i = i + 1 {
        result = result * i;
    }
    return result;
}
```

### Concurrent Computation
```bumerango
fn compute(id: i32, results: chan i32) {
    results <- id * id;
}

fn main() {
    chan results = make(i32);
    
    co {
        compute(1, results);
        compute(2, results);
        compute(3, results);
    }
    
    for let i: i32 = 1; i <= 3; i = i + 1 {
        println("Result: {}", <- results);
    }
}
```

### Ownership Transfer
```bumerango
fn take_ownership(s: str) {
    // s is owned here
    println(s);
}
// s is dropped after function ends

fn main() {
    let message = "Hello";
    take_ownership(message);
    // message is no longer valid here
}
```

---

## Version History

- **v0.1 (Current):** Lexer, Parser, Basic Codegen, Examples
- **v0.2 (Q2 2026):** Type checker, Borrow checker, Better error messages
- **v0.3 (Q3 2026):** Standard library, Generics, Traits
- **v1.0 (Q4 2026):** Production-ready, Package manager, Full stdlib

---

## License

MIT License - Open source and free to use

---

**Status:** Alpha - Active Development  
**Last Updated:** May 2026
