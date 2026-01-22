# Rust Mastery Learning Plan 🦀

This plan is designed to take you from a Rust beginner to a confident developer, focusing on the unique concepts that make Rust powerful.

## Phase 1: The Fundamentals (Foundations)

### Day 1: Hello Rust & Cargo
- [ ] Install Rust (`rustup`)
- [ ] Understand `cargo` (new, build, run, check)
- [ ] Basic project structure
- [ ] `println!` macro and basic formatting
- **Practice:** Create a "Hello, World" project.

### Day 2: Variables & Data Types
- [ ] Mutability (`mut`)
- [ ] Variable Shadowing
- [ ] Scalar types (integers, floats, bool, char)
- [ ] Compound types (tuples, arrays)
- **Practice:** Build a simple calculator for temperatures (Celsius/Fahrenheit).

### Day 3: Functions & Control Flow
- [ ] Function signatures and return types
- [ ] Expressions vs Statements
- [ ] `if` / `else` (as an expression)
- [ ] Loops (`loop`, `while`, `for`)
- **Practice:** Implement a Fibonacci sequence generator.

### Day 4: Ownership (The Heart of Rust)
- [ ] Memory safety without a GC
- [ ] Scope and the `Drop` trait
- [ ] Moving vs Cloning
- [ ] The Stack vs the Heap
- **Practice:** Small exercises to move and clone data.

### Day 5: Borrowing & References
- [ ] Immutable references (`&T`)
- [ ] Mutable references (`&mut T`)
- [ ] The Rules of Borrowing (no data races)
- [ ] Dangling references
- **Practice:** Write functions that take references and modify data.

### Day 6: Slices & Strings
- [ ] String vs `&str`
- [ ] Array slices
- [ ] Memory layout of slices
- **Practice:** Implementation of a "First Word" finder in a string.

### Day 7: Structs & Methods
- [ ] Defining and instantiating structs
- [ ] Tuple structs
- [ ] `impl` blocks and methods
- [ ] Associated functions (constructors)
- **Practice:** Create a `Rectangle` struct with area and perimeter methods.

---

## Phase 2: Power Tools (Real-World Rust)

### Day 8: Enums & Pattern Matching
- [ ] Defining Enums
- [ ] The `Option<T>` enum
- [ ] `match` control flow
- [ ] `if let` syntax
- **Practice:** Create a state machine or a "Coin" calculator.

### Day 9: Modules & Packages
- [ ] `mod` and `pub`
- [ ] `use` keyword
- [ ] Crate organization
- **Practice:** Refactor a project into multiple files.

### Day 10: Common Collections
- [ ] Vectors (`Vec<T>`)
- [ ] Strings
- [ ] Hash Maps
- **Practice:** A simple "Todo List" using a Vector or a "Word Counter" using a HashMap.

### Day 11: Error Handling
- [ ] `panic!` for unrecoverable errors
- [ ] `Result<T, E>` for recoverable errors
- [ ] The `?` operator (early returns)
- **Practice:** File reading with error handling.

### Day 12: Generics & Traits
- [ ] Using generics in functions and structs
- [ ] Defining and implementing Traits
- [ ] Trait Bounds
- **Practice:** Implement a `Summary` trait for different objects.

### Day 13: Lifetimes
- [ ] The Borrow Checker
- [ ] Lifetime annotations (`'a`)
- [ ] Lifetime elision rules
- **Practice:** Functions that return the longest of two string slices.

### Day 14: Automated Testing
- [ ] Unit tests (`#[test]`)
- [ ] Integration tests
- [ ] Documentation tests
- **Practice:** Write a full test suite for a math library.

---

## Phase 3: Advanced Concepts (Mastery)

### Day 15: Functional Rust
- [ ] Closures
- [ ] Iterators (`map`, `filter`, `fold`)
- **Practice:** Processing a list of numbers using functional style.

### Day 16: Smart Pointers
- [ ] `Box<T>` for heap allocation
- [ ] `Rc<T>` for reference counting
- [ ] `RefCell<T>` for interior mutability
- **Practice:** Implement a recursive data structure (like a Linked List).

### Day 17: Fearless Concurrency
- [ ] Spawn threads
- [ ] Message passing (`mpsc`)
- [ ] Shared state (`Arc`, `Mutex`)
- **Practice:** A multi-threaded web crawler or a parallel sum calculator.

### Day 18: Unsafe Rust & Macros
- [ ] When to use `unsafe`
- [ ] Basic declarative macros (`macro_rules!`)
- **Practice:** Create a simple HTML-builder macro.

---

## Project Ideas for Mastery
1. **CLI Tool:** A grep-like tool (search for text in files).
2. **Web Server:** A basic HTTP server from scratch using `std::net`.
3. **Rust + Tauri:** Build a desktop app (since we are in DockPilot!).
4. **Interpreter:** A simple calculator or Lisp-like language.

---
*Ready to start? Let's begin with **Day 1: Hello Rust**!*
