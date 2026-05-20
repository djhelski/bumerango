use std::env;
use std::fs;
use std::io::{self, Read};
use bumerango::compile;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "compile" | "build" => {
            if args.len() < 3 {
                eprintln!("Usage: bumerang compile <file.bume>");
                return;
            }
            compile_file(&args[2]);
        }
        "repl" => {
            repl();
        }
        "version" => {
            println!("Bumerango v0.1.0");
        }
        "help" => {
            print_help();
        }
        file => {
            // Default: treat as file to compile
            compile_file(file);
        }
    }
}

fn compile_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(source) => {
            println!("[*] Compiling: {}", filename);

            match compile(&source) {
                Ok(llvm_ir) => {
                    let output_file = filename.replace(".bume", ".ll");
                    match fs::write(&output_file, &llvm_ir) {
                        Ok(_) => {
                            println!("[✓] Success! LLVM IR written to: {}", output_file);
                            println!("\n--- LLVM IR ---\n{}", llvm_ir);

                            // Try to compile with llc
                            compile_with_llc(&output_file);
                        }
                        Err(e) => {
                            eprintln!("[✗] Error writing output file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[✗] Compilation error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("[✗] Error reading file: {}", e);
        }
    }
}

fn compile_with_llc(ll_file: &str) {
    println!("\n[*] Attempting to compile LLVM IR with 'llc'...");

    let output = std::process::Command::new("llc")
        .arg(ll_file)
        .arg("-o")
        .arg(ll_file.replace(".ll", ".s"))
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("[✓] Assembly generated: {}", ll_file.replace(".ll", ".s"));
            } else {
                println!("[!] llc not found or failed - install LLVM tools for assembly generation");
            }
        }
        Err(_) => {
            println!("[!] llc not found - install LLVM tools for assembly generation");
        }
    }
}

fn repl() {
    println!("=== Bumerango REPL v0.1.0 ===");
    println!("Type 'exit' to quit, 'help' for commands\n");

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!("bumerang> ");
        use std::io::Write;
        io::stdout().flush().unwrap();

        buffer.clear();
        if stdin.read_line(&mut buffer).is_err() {
            break;
        }

        let line = buffer.trim();

        match line {
            "exit" | "quit" => break,
            "help" => {
                println!("Commands:");
                println!("  exit          - Exit REPL");
                println!("  help          - Show this help");
                println!("  clear         - Clear screen");
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
            }
            _ if !line.is_empty() => {
                match compile(line) {
                    Ok(ir) => {
                        println!("LLVM IR:\n{}", ir);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            _ => {}
        }
    }

    println!("Goodbye!");
}

fn print_help() {
    println!("Bumerango - The Go-like, Rust-safe, Assembly-fast Language\n");
    println!("USAGE:");
    println!("    bumerang <COMMAND> [OPTIONS]\n");
    println!("COMMANDS:");
    println!("    compile <file>  - Compile Bumerango source to LLVM IR");
    println!("    repl            - Interactive REPL");
    println!("    version         - Show version");
    println!("    help            - Show this help\n");
    println!("EXAMPLES:");
    println!("    bumerang compile main.bume");
    println!("    bumerang repl");
}
