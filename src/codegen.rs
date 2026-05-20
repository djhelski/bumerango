use crate::ast::*;
use std::collections::HashMap;

pub struct Codegen {
    functions: String,
    strings: Vec<String>,
    string_count: usize,
    variables: HashMap<String, String>, // var_name -> llvm_name
    var_counter: usize,
    temp_counter: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            functions: String::new(),
            strings: Vec::new(),
            string_count: 0,
            variables: HashMap::new(),
            var_counter: 0,
            temp_counter: 0,
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        let mut output = String::new();

        // LLVM module header
        output.push_str("; Bumerango → LLVM IR\n");
        output.push_str(&format!("; Package: {}\n", program.package));
        output.push_str("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        output.push_str("target triple = \"x86_64-unknown-linux-gnu\"\n\n");

        // String constants
        for item in &program.items {
            if let TopLevel::Function(func) = item {
                self.extract_strings(&func.body);
            }
        }

        // Declare string constants
        for (i, s) in self.strings.iter().enumerate() {
            output.push_str(&format!(
                "@.str.{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n",
                i,
                s.len() + 1,
                escape_string(s)
            ));
        }

        if !self.strings.is_empty() {
            output.push_str("\n");
        }

        // Forward declarations
        for item in &program.items {
            if let TopLevel::Function(func) = item {
                output.push_str(&self.declare_function(func));
            }
        }

        output.push_str("\n");

        // Function definitions
        for item in &program.items {
            if let TopLevel::Function(func) = item {
                self.generate_function(func, &mut output);
            }
        }

        output
    }

    fn extract_strings(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.extract_strings_stmt(stmt);
        }
    }

    fn extract_strings_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr) => self.extract_strings_expr(expr),
            Statement::If(if_stmt) => {
                self.extract_strings(&if_stmt.then_block);
                if let Some(else_block) = &if_stmt.else_simple {
                    self.extract_strings(else_block);
                }
            }
            Statement::For(for_stmt) => self.extract_strings(&for_stmt.body),
            Statement::Loop(block) => self.extract_strings(block),
            Statement::Co(block) => self.extract_strings(block),
            _ => {}
        }
    }

    fn extract_strings_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::String(s) => {
                if !self.strings.contains(s) {
                    self.strings.push(s.clone());
                }
            }
            Expression::Call(func, args) => {
                self.extract_strings_expr(func);
                for arg in args {
                    self.extract_strings_expr(arg);
                }
            }
            Expression::Binary(_, left, right) => {
                self.extract_strings_expr(left);
                self.extract_strings_expr(right);
            }
            Expression::Unary(_, expr) => self.extract_strings_expr(expr),
            Expression::MemberAccess(expr, _) => self.extract_strings_expr(expr),
            Expression::Index(expr, idx) => {
                self.extract_strings_expr(expr);
                self.extract_strings_expr(idx);
            }
            _ => {}
        }
    }

    fn declare_function(&self, func: &FunctionDef) -> String {
        let ret_type = self.type_to_llvm(&func.return_type);
        let params = func
            .params
            .iter()
            .map(|p| format!("{} %{}", self.type_to_llvm(&p.param_type), p.name))
            .collect::<Vec<_>>()
            .join(", ");

        format!("declare {} @{}({})\n", ret_type, func.name, params)
    }

    fn generate_function(&mut self, func: &FunctionDef, output: &mut String) {
        let ret_type = self.type_to_llvm(&func.return_type);
        let params = func
            .params
            .iter()
            .map(|p| format!("{} %{}", self.type_to_llvm(&p.param_type), p.name))
            .collect::<Vec<_>>()
            .join(", ");

        output.push_str(&format!("define {} @{}({}) {{\n", ret_type, func.name, params));

        // Function body
        self.variables.clear();
        self.var_counter = 0;
        self.temp_counter = 0;

        for param in &func.params {
            self.variables
                .insert(param.name.clone(), format!("%{}", param.name));
        }

        let mut body_ir = String::new();
        for stmt in &func.body.statements {
            self.generate_statement(stmt, &mut body_ir);
        }

        output.push_str(&body_ir);

        // Default return if needed
        match &func.return_type {
            Type::Any => output.push_str("  ret void\n"),
            _ => {}
        }

        output.push_str("}\n\n");
    }

    fn generate_statement(&mut self, stmt: &Statement, output: &mut String) {
        match stmt {
            Statement::Let(let_stmt) => {
                let value_ir = self.generate_expression(&let_stmt.value, output);
                let llvm_type = if let Some(t) = &let_stmt.value_type {
                    self.type_to_llvm(t)
                } else {
                    "i64".to_string()
                };

                let var_name = format!("%var.{}", self.var_counter);
                self.var_counter += 1;
                self.variables.insert(let_stmt.name.clone(), var_name.clone());

                output.push_str(&format!("  {} = alloca {}\n", var_name, llvm_type));
                output.push_str(&format!("  store {} {}, {} {}\n", llvm_type, value_ir, llvm_type, var_name));
            }
            Statement::Return(Some(expr)) => {
                let value_ir = self.generate_expression(expr, output);
                let ret_type = match expr {
                    Expression::Integer(_) => "i64",
                    Expression::Float(_) => "f64",
                    Expression::Bool(_) => "i1",
                    Expression::String(_) => "i8*",
                    _ => "i64",
                };
                output.push_str(&format!("  ret {} {}\n", ret_type, value_ir));
            }
            Statement::Return(None) => {
                output.push_str("  ret void\n");
            }
            Statement::Expression(expr) => {
                self.generate_expression(expr, output);
            }
            Statement::If(if_stmt) => {
                self.generate_if_statement(if_stmt, output);
            }
            Statement::For(for_stmt) => {
                self.generate_for_statement(for_stmt, output);
            }
            Statement::Co(_) => {
                // Simplified: co blocks are treated like regular blocks for now
                output.push_str("  ; co block (concurrent)\n");
            }
            Statement::Loop(block) => {
                let loop_id = self.temp_counter;
                self.temp_counter += 1;

                output.push_str(&format!("  br label %loop.{}\n", loop_id));
                output.push_str(&format!("loop.{}:\n", loop_id));

                for stmt in &block.statements {
                    self.generate_statement(stmt, output);
                }

                output.push_str(&format!("  br label %loop.{}\n", loop_id));
            }
            Statement::Break => {
                output.push_str("  br label %break\n");
            }
            Statement::Continue => {
                output.push_str("  br label %continue\n");
            }
            Statement::Panic(msg) => {
                output.push_str("  call void @panic()\n");
            }
            _ => {}
        }
    }

    fn generate_if_statement(&mut self, if_stmt: &IfStmt, output: &mut String) {
        let cond_ir = self.generate_expression(&if_stmt.condition, output);
        let if_id = self.temp_counter;
        self.temp_counter += 1;

        output.push_str(&format!("  br i1 {}, label %then.{}, label %else.{}\n", cond_ir, if_id, if_id));

        output.push_str(&format!("then.{}:\n", if_id));
        for stmt in &if_stmt.then_block.statements {
            self.generate_statement(stmt, output);
        }

        if if_stmt.else_simple.is_some() {
            output.push_str(&format!("  br label %else.end.{}\n", if_id));
            output.push_str(&format!("else.{}:\n", if_id));
            if let Some(else_block) = &if_stmt.else_simple {
                for stmt in &else_block.statements {
                    self.generate_statement(stmt, output);
                }
            }
            output.push_str(&format!("else.end.{}:\n", if_id));
        } else {
            output.push_str(&format!("else.{}:\n", if_id));
        }
    }

    fn generate_for_statement(&mut self, for_stmt: &ForStmt, output: &mut String) {
        if let Some(init) = &for_stmt.init {
            self.generate_statement(init, output);
        }

        let loop_id = self.temp_counter;
        self.temp_counter += 1;

        output.push_str(&format!("  br label %for.check.{}\n", loop_id));
        output.push_str(&format!("for.check.{}:\n", loop_id));

        if let Some(cond) = &for_stmt.condition {
            let cond_ir = self.generate_expression(cond, output);
            output.push_str(&format!(
                "  br i1 {}, label %for.body.{}, label %for.end.{}\n",
                cond_ir, loop_id, loop_id
            ));
        }

        output.push_str(&format!("for.body.{}:\n", loop_id));
        for stmt in &for_stmt.body.statements {
            self.generate_statement(stmt, output);
        }

        if let Some(update) = &for_stmt.update {
            self.generate_expression(update, output);
        }

        output.push_str(&format!("  br label %for.check.{}\n", loop_id));
        output.push_str(&format!("for.end.{}:\n", loop_id));
    }

    fn generate_expression(&mut self, expr: &Expression, output: &mut String) -> String {
        match expr {
            Expression::Integer(n) => format!("{}", n),
            Expression::Float(f) => format!("{}", f),
            Expression::Bool(b) => format!("{}", if *b { 1 } else { 0 }),
            Expression::String(s) => {
                let idx = self.strings.iter().position(|x| x == s).unwrap();
                format!("@.str.{}", idx)
            }
            Expression::Identifier(name) => {
                self.variables
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| format!("%{}", name))
            }
            Expression::Binary(op, left, right) => {
                self.generate_binary_op(*op, left, right, output)
            }
            Expression::Call(func, args) => self.generate_call(func, args, output),
            Expression::Unary(op, expr) => self.generate_unary_op(*op, expr, output),
            _ => "%undef".to_string(),
        }
    }

    fn generate_binary_op(
        &mut self,
        op: BinaryOp,
        left: &Expression,
        right: &Expression,
        output: &mut String,
    ) -> String {
        let left_ir = self.generate_expression(left, output);
        let right_ir = self.generate_expression(right, output);

        let temp_var = format!("%tmp.{}", self.temp_counter);
        self.temp_counter += 1;

        let instr = match op {
            BinaryOp::Add => format!("add i64 {}, {}", left_ir, right_ir),
            BinaryOp::Subtract => format!("sub i64 {}, {}", left_ir, right_ir),
            BinaryOp::Multiply => format!("mul i64 {}, {}", left_ir, right_ir),
            BinaryOp::Divide => format!("sdiv i64 {}, {}", left_ir, right_ir),
            BinaryOp::Equal => format!("icmp eq i64 {}, {}", left_ir, right_ir),
            BinaryOp::NotEqual => format!("icmp ne i64 {}, {}", left_ir, right_ir),
            BinaryOp::Less => format!("icmp slt i64 {}, {}", left_ir, right_ir),
            BinaryOp::Greater => format!("icmp sgt i64 {}, {}", left_ir, right_ir),
            BinaryOp::And => format!("and i1 {}, {}", left_ir, right_ir),
            BinaryOp::Or => format!("or i1 {}, {}", left_ir, right_ir),
            _ => "undef".to_string(),
        };

        output.push_str(&format!("  {} = {}\n", temp_var, instr));
        temp_var
    }

    fn generate_unary_op(
        &mut self,
        op: UnaryOp,
        expr: &Expression,
        output: &mut String,
    ) -> String {
        let expr_ir = self.generate_expression(expr, output);
        let temp_var = format!("%tmp.{}", self.temp_counter);
        self.temp_counter += 1;

        let instr = match op {
            UnaryOp::Negate => format!("sub i64 0, {}", expr_ir),
            UnaryOp::Not => format!("xor i1 {}, 1", expr_ir),
            _ => "undef".to_string(),
        };

        output.push_str(&format!("  {} = {}\n", temp_var, instr));
        temp_var
    }

    fn generate_call(
        &mut self,
        func: &Expression,
        args: &[Expression],
        output: &mut String,
    ) -> String {
        let func_name = match func {
            Expression::Identifier(name) => name.clone(),
            _ => "unknown".to_string(),
        };

        let mut arg_strs = Vec::new();
        for arg in args {
            let arg_ir = self.generate_expression(arg, output);
            arg_strs.push(format!("i64 {}", arg_ir));
        }

        let call_ir = if func_name == "println" {
            format!(
                "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ({} x i8], [3 x i8]* @\".str\", i32 0, i32 0), {})",
                3,
                arg_strs.join(", ")
            )
        } else {
            format!("call i64 @{}({})", func_name, arg_strs.join(", "))
        };

        let temp_var = format!("%tmp.{}", self.temp_counter);
        self.temp_counter += 1;

        output.push_str(&format!("  {} = {}\n", temp_var, call_ir));
        temp_var
    }

    fn type_to_llvm(&self, t: &Type) -> String {
        match t {
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::I128 => "i128".to_string(),
            Type::U8 => "i8".to_string(),
            Type::U16 => "i16".to_string(),
            Type::U32 => "i32".to_string(),
            Type::U64 => "i64".to_string(),
            Type::U128 => "i128".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Bool => "i1".to_string(),
            Type::String => "i8*".to_string(),
            Type::Array(inner) => format!("{{{{}}}} x {}}}", self.type_to_llvm(inner)),
            Type::Reference(inner, _) => format!("{}*", self.type_to_llvm(inner)),
            Type::Pointer(inner) => format!("{}*", self.type_to_llvm(inner)),
            Type::Channel(_) => "i8*".to_string(), // Simplified
            Type::Any => "void".to_string(),
            _ => "i64".to_string(),
        }
    }
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\n", "\\0A")
        .replace("\t", "\\09")
        .replace("\"", "\\22")
}
