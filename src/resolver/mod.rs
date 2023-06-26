use crate::ast::Program;

pub struct Resolver {
    pub program: Program,
}

impl Resolver {
    pub fn new(program: &Program) -> Self {
        Resolver {
            program: program.clone(),
        }
    }

    pub fn resolve(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
