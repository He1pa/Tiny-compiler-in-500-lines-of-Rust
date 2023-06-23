#[derive(Debug, PartialEq, Clone, Default)]
pub struct Module {
    pub modname: String, // module full name which includes this file
    pub body: Vec<Stmt>,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stmt {
    pub kind: StmtKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StmtKind {
    Function(FunctionStmt),
    Expr(ExprStmt),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionStmt {
    pub name: Identifier,
    // pub params: Vec<Parameter>,
    pub body: Vec<Stmt>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier {
    pub name: String,
}

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub enum Visibility {
    Pub,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Call(CallExpr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallExpr {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}