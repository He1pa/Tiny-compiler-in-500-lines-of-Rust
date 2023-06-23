pub mod ast;
pub mod parser;
pub mod lexer;

use lexer::Lexer;

use crate::parser::Parser;

fn main() -> anyhow::Result<()>{
    let input = r#"fn main() {

    }
    "#;
    let lexer = Lexer::new(input)?;
    let mut parser = Parser::new(lexer)?;
    let module = parser.parse()?;
    println!("{:?}", module);
    Ok(())

}
