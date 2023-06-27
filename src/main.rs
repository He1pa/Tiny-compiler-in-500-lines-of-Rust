mod ast;
mod codegen;
mod lexer;
mod parser;
mod resolver;

use std::env;
use std::process::Command;
use codegen::emit_code;
use lexer::Lexer;
use resolver::Resolver;

use crate::parser::Parser;

fn main() -> anyhow::Result<()> {
    let input = r#"fn main() {
        println!("Hello world");
    }
    "#;
    let lexer = Lexer::new(input)?;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse()?;
    let resolver = Resolver::new(&program);
    resolver.resolve()?;
    let path = env::current_dir().unwrap();
    let llvm_ir_path = path.join("output.ll");
    let output_path = path.join("my_program");
    emit_code(program, &llvm_ir_path)?;
    let mut cmd = Command::new("clang");
    cmd.arg("-o")
        .arg(&output_path)
        .arg(&llvm_ir_path)
        .arg("-lc")
        .status()
        .expect("Failed to execute clang");
    Ok(())
}
