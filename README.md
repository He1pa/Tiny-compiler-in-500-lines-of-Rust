# Tiny compiler in 500 lines of Rust

Write a tiny compiler in 500 lines of rust to run `hello world` in cargo init template.

```Rust
fn main() {
    println!("Hello, world!");
}
```

It's very simple but still includes a hand-coded lexer, parser (LL1) and LLVM IR codegen. I didn't implement the resolver part because the example code is simple enough that I assume it will always be right.

## requirement
+ Clang
+ LLVM 12
