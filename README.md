# 🦀 R-Lox: A Tree-Walk Interpreter in Rust

![Lox Language](https://img.shields.io/badge/Language-Lox-orange)
![Rust](https://img.shields.io/badge/Implemented%20in-Rust-red)
![Status](https://img.shields.io/badge/Status-Complete-green)

A high-performance, memory-safe **Lox** interpreter implemented in Rust. This project is a complete implementation of the tree-walk interpreter from Robert Nystrom's *Crafting Interpreters*, adapted to leverage Rust's unique ownership model.

## 🚀 Features

* **Complete Lexical Pipeline**: Hand-written scanner and recursive-descent parser.
* **First-Class Functions**: Full support for closures and lexical scoping.
* **Object-Oriented Programming**:
    * Classes with method inheritance.
    * Instance property access (`get` and `set`).
    * Constructors (`init`) with correct `this` binding.
    * Superclass method access via the `super` keyword.
* **Static Resolution**: A dedicated resolution pass ensuring variables are bound to their correct lexical scopes before execution.
* **Robust Error Handling**: Precise runtime and parse-time error reporting with line and token context.

---

## 🛠 Architecture

The interpreter follows a multi-stage pipeline:

1.  **Scanner**: Tokenizes raw source code into structured `Token` objects.
2.  **Parser**: Transforms tokens into an Abstract Syntax Tree (AST) using recursive descent.
3.  **Resolver**: Walks the AST to resolve variable bindings and calculate "scope distances" for the interpreter.
4.  **Interpreter**: Executes the AST by traversing nodes and managing state within nested environments.



---

## 📋 Example Code

```lox
class Doughnut {
  cook() {
    print "Fry until golden brown.";
  }
}

class BostonCream : Doughnut {
  cook() {
    super.cook();
    print "Pipe full of custard and coat with chocolate.";
  }
}

var myDoughnut = BostonCream();
myDoughnut.cook();
