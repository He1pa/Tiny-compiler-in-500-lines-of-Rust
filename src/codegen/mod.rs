use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use std::path::Path;

use crate::ast::{CallExpr, Expr, ExprStmt, Program, Stmt};

pub struct CodegenContext<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub program: Program,
}

pub type CompileResult<'a> = anyhow::Result<BasicValueEnum<'a>>;
// Result<BasicValueEnum<'a>, String>;

impl<'ctx> CodegenContext<'ctx> {
    pub fn ok_result(&self) -> CompileResult<'ctx> {
        let i32_type = self.context.i32_type();
        Ok(i32_type.const_int(0u64, false).into())
    }

    pub fn new(context: &'ctx Context, module: Module<'ctx>, program: Program) -> Self {
        CodegenContext {
            context,
            module,
            program,
            builder: context.create_builder(),
        }
    }

    pub fn codegen(self: &CodegenContext<'ctx>) -> CompileResult {
        let main_func_stmt = self
            .program
            .body
            .iter()
            .find(|stmt| match &stmt.kind {
                crate::ast::StmtKind::Function(func) => func.name.name == "main",
                _ => false,
            })
            .unwrap()
            .into_func_stmt();

        let main_type = self.context.i32_type().fn_type(&[], false);

        let main_func = self.module.add_function("main", main_type, None);

        let entry = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(entry);

        for stmt in main_func_stmt.body {
            self.walk_stmt(&stmt)?;
        }
        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));
        self.ok_result()
    }

    pub fn walk_stmt(&self, stmt: &Stmt) -> CompileResult {
        match &stmt.kind {
            crate::ast::StmtKind::Expr(expr_stmt) => self.walk_expr_stmt(expr_stmt),
            _ => todo!(),
        }
    }

    pub fn walk_expr_stmt(&self, expr_stmt: &ExprStmt) -> CompileResult {
        self.walk_expr(&expr_stmt.expr)
    }

    pub fn walk_expr(&self, expr: &Expr) -> CompileResult {
        match &expr.kind {
            crate::ast::ExprKind::Call(call_expr) => self.walk_call_expr(call_expr),
            crate::ast::ExprKind::Ident(_) => todo!(),
            crate::ast::ExprKind::Str(_) => todo!(),
        }
    }

    pub fn walk_call_expr(&self, call_expr: &CallExpr) -> CompileResult {
        match &call_expr.func.kind {
            crate::ast::ExprKind::Ident(id) => match id.as_str() {
                "println!" => {
                    let printf_type = self.context.void_type().fn_type(
                        &[self
                            .context
                            .i8_type()
                            .ptr_type(AddressSpace::default())
                            .into()],
                        false,
                    );
                    let printf_func = self.module.add_function("printf", printf_type, None);
                    match &call_expr.args[0].kind {
                        crate::ast::ExprKind::Str(s) => {
                            // Define the string to be printed
                            let print_s = s.clone() + "\n";
                            let str_data = self.context.const_string(print_s.as_bytes(), false);
                            let str_global =
                                self.module.add_global(str_data.get_type(), None, "string");
                            str_global.set_initializer(&str_data);
                            // Get a pointer to the string
                            let str_ptr = self.builder.build_pointer_cast(
                                str_global.as_pointer_value(),
                                self.context.i8_type().ptr_type(AddressSpace::default()),
                                "ptr",
                            );
                            self.builder
                                .build_call(printf_func, &[str_ptr.into()], "call");
                        }
                        _ => todo!(),
                    }
                    self.ok_result()
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

pub fn emit_code(program: Program, output_file: &Path) -> anyhow::Result<()> {
    let context = Context::create();
    let module = context.create_module("main_module");
    let ctx = CodegenContext::new(&context, module, program);
    CodegenContext::codegen(&ctx)?;
    ctx.module
        .print_to_file(output_file)
        .expect("Failed to write LLVM IR to file.");
    Ok(())
}
