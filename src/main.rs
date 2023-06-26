mod ast;
mod codegen;
mod lexer;
mod parser;
mod resolver;

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
    let output_file = std::path::Path::new("path/to/output.ll");
    emit_code(program, output_file)?;
    // Now we get llvm ir in `output_file`, then run `clang -o my_program path/to/output.ll -lm`
    // to get executable binary `my_program`. Then you can run it and get `Hello world` in your terminal.
    Ok(())
}
