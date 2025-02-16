use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::process::Command;

mod lexer;

use lexer::Lexer;
use lexer::Token;
use lexer::TokenType;

mod parser;

use parser::NodeType;
use parser::Parser;

mod code_generator;

use code_generator::CodeGenerator;

#[derive(Debug, Clone)]
struct CompiledFile {
    filepath: String,
    asm: String,
}

impl CompiledFile {
    fn new(filepath: String, asm: String) -> Self {
        Self { filepath, asm }
    }
}

#[derive(Debug, Clone)]
struct Compiler;

impl Compiler {
    pub fn compile(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Vec<CompiledFile>, &'static str> {
        args.next();

        let mut files: Vec<CompiledFile> = Vec::<CompiledFile>::new();

        for filepath in args {
            let contents =
                fs::read_to_string(&filepath).expect("Should have been able to read the file");

            /* Lexical analysis */
            let mut binding = Lexer::new(&contents);
            let lex = binding.lex();

            /* Parsing */
            let mut parser = Parser::new(lex.unwrap().tokens.clone());
            let ast = match parser.parse() {
                Ok(ast) => ast,
                Err(error) => return Err(error),
            };

            /* Code generation */
            let code = match CodeGenerator::generate(&ast) {
                Ok(code) => code,
                Err(error) => return Err(error),
            };

            files.push(CompiledFile::new(filepath, code));
        }

        Ok(files)
    }
}

fn main() -> std::io::Result<()> {
    if env::args().count() == 1 {
        eprintln!("rcc: fatal error: no input files");
        process::exit(1);
    }

    if env::args().count() > 2 {
        eprintln!("rcc can only handle one file");
        process::exit(1);
    }

    let compiled_files_struct: Vec<CompiledFile> =
        Compiler::compile(env::args()).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    for compiled_file_struct in compiled_files_struct {
        let mut asm_path = String::new();
        let mut exec_path = String::new();

        let path_parts = compiled_file_struct.filepath.split("/");
        let path_collection: Vec<&str> = path_parts.collect();

        for i in 0..path_collection.len() {
            if i != 0 {
                asm_path.push('/');
                exec_path.push('/');
            }

            if i == path_collection.len() - 1 {
                let filename_parts = path_collection[i].split(".");
                let filename_collection: Vec<&str> = filename_parts.collect();

                asm_path.push_str(filename_collection[0]);
                asm_path.push_str(".s");

                exec_path.push_str(filename_collection[0]);
            } else {
                asm_path.push_str(path_collection[i]);
                exec_path.push_str(path_collection[i]);
            }
        }

        let mut file = File::create(&asm_path)?;
        let _ = file.write_all(compiled_file_struct.asm.as_bytes());

        Command::new("gcc")
            .args(["-o", &exec_path, &asm_path])
            .output()
            .expect("failed to execute process");

        fs::remove_file(asm_path)?;
    }

    Ok(())
}
