pub mod ast;
pub mod parser;
pub mod lexer;
pub mod resolver;

use lexer::Lexer;
use resolver::Resolver;

use crate::parser::Parser;

fn main() -> anyhow::Result<()>{
    let input = r#"fn main() {
        println!("Hello, world!");
    }
    "#;
    let lexer = Lexer::new(input)?;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse()?;
    let resolver = Resolver::new(&program);
    resolver.resolve()?;
    for stmt in program.body{
        println!("{:?}", stmt);
    }
    Ok(())

}
