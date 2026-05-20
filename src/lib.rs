pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;

use parser::Parser;
use codegen::Codegen;

pub fn compile(source: &str) -> Result<String, String> {
    // Parse
    let mut parser = Parser::new(source);
    let program = parser.parse()?;

    // Generate LLVM IR
    let mut codegen = Codegen::new();
    let llvm_ir = codegen.generate(&program);

    Ok(llvm_ir)
}
