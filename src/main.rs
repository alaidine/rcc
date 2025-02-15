use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Keyword,
    Identifier,
    Integer,
    CloseParen,
    OpenParen,
    CloseBrace,
    OpenBrace,
    Semicolon,
}

#[derive(Debug, Clone)]
enum NodeType {
    Program(ProgramNode),
    Function(FunctionNode),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

#[derive(Debug, Clone)]
struct Token<'a> {
    r#type: TokenType,
    value: &'a str,
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    source: &'a String,
    tokens: Vec<Token<'a>>,
}

#[derive(Debug, Clone)]
struct ProgramNode {
    children: Vec<NodeType>,
}

impl ProgramNode {
    fn new(children: Vec<NodeType>) -> Self {
        Self { children }
    }
}

#[derive(Debug, Clone)]
struct FunctionNode {
    id: String,
    children: Vec<NodeType>,
}

impl FunctionNode {
    fn new(id: String, children: Vec<NodeType>) -> Self {
        Self { id, children }
    }
}
#[derive(Debug, Clone)]
struct StatementNode {
    children: Vec<NodeType>,
}

impl StatementNode {
    fn new(children: Vec<NodeType>) -> Self {
        Self { children }
    }
}

#[derive(Debug, Clone)]
struct ExpressionNode {
    value: u32,
}

impl ExpressionNode {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: u32,
}

impl Parser<'_> {
    fn new<'a>(tokens: Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            tokens,
            current: 0 as u32,
        }
    }

    fn parse(&mut self) -> Result<NodeType, &'static str> {
        match Self::parse_function(self) {
            Ok(function) => Ok(NodeType::Program(ProgramNode::new(Vec::from([function])))),
            Err(err) => return Err(err),
        }
    }

    fn parse_function(&mut self) -> Result<NodeType, &'static str> {
        let id;

        if self.tokens[self.current as usize].value != "int" {
            return Err("Error in parse_function");
        }

        self.current += 1;

        if self.tokens[self.current as usize].r#type != TokenType::Identifier {
            return Err("Error in parse_function");
        }

        id = String::from(self.tokens[self.current as usize].value);

        self.current += 1;

        if self.tokens[self.current as usize].value != "(" {
            return Err("Error in parse_function");
        }

        self.current += 1;

        if self.tokens[self.current as usize].value != "void" {
            return Err("Error in parse_function");
        }

        self.current += 1;

        if self.tokens[self.current as usize].value != ")" {
            return Err("Error in parse_function");
        }

        self.current += 1;

        if self.tokens[self.current as usize].value != "{" {
            return Err("Error in parse_function");
        }

        self.current += 1;

        let function = match Self::parse_statement(self) {
            Err(_) => Err("Error in parse_function"),
            Ok(statement) => Ok(NodeType::Function(FunctionNode::new(
                String::from(id),
                Vec::from([statement]),
            ))),
        };

        if self.tokens[self.current as usize].value != "}" {
            return Err("Error in parse_function");
        }

        Ok(function?)
    }

    fn parse_statement(&mut self) -> Result<NodeType, &'static str> {
        if self.tokens[self.current as usize].value != "return" {
            return Err("Error in parse_statement");
        }

        self.current += 1;

        let statement = match Self::parse_expression(self) {
            Err(_) => Err("Error in parse_statement"),
            Ok(expression) => Ok(NodeType::Statement(StatementNode::new(Vec::from([
                expression,
            ])))),
        };

        if self.tokens[self.current as usize].value != ";" {
            return Err("Error in parse_statement");
        }

        self.current += 1;

        Ok(statement?)
    }

    fn parse_expression(&mut self) -> Result<NodeType, &'static str> {
        if self.tokens[self.current as usize].r#type != TokenType::Integer {
            return Err("Error in parse_expression");
        }

        let expression = NodeType::Expression(ExpressionNode::new(
            self.tokens[self.current as usize]
                .value
                .parse::<u32>()
                .unwrap(),
        ));

        self.current += 1;

        Ok(expression)
    }
}

impl Lexer<'_> {
    fn new(src: &String) -> Lexer {
        Lexer {
            source: src,
            tokens: Vec::<Token>::new(),
        }
    }

    fn lex(&mut self) -> Result<&Lexer, &'static str> {
        let len = self.source.len();
        let keywords = HashSet::from(["int", "return", "void"]);

        let mut i: usize = 0;

        while i < len.try_into().unwrap() {
            let ch = self.source.as_bytes()[i] as char;

            if ch.is_ascii_alphabetic() {
                let mut y: usize = i;

                while (self.source.as_bytes()[y] as char).is_ascii_alphabetic() {
                    y += 1;
                }

                if keywords.contains(&self.source[i..y]) {
                    self.tokens.push(Token {
                        r#type: TokenType::Keyword,
                        value: &self.source[i..y],
                    });
                } else {
                    self.tokens.push(Token {
                        r#type: TokenType::Identifier,
                        value: &self.source[i..y],
                    });
                }

                i = y;

                continue;
            }

            if ch.is_digit(10) {
                let mut y: usize = i;

                while (self.source.as_bytes()[y] as char).is_digit(10) {
                    y += 1;
                }

                self.tokens.push(Token {
                    r#type: TokenType::Integer,
                    value: &self.source[i..y],
                });

                i = y;

                continue;
            }

            if ch == ';' {
                self.tokens.push(Token {
                    r#type: TokenType::Semicolon,
                    value: ";",
                });

                i += 1;

                continue;
            }

            if ch == '{' {
                self.tokens.push(Token {
                    r#type: TokenType::OpenBrace,
                    value: "{",
                });

                i += 1;

                continue;
            }

            if ch == '}' {
                self.tokens.push(Token {
                    r#type: TokenType::CloseBrace,
                    value: "}",
                });

                i += 1;

                continue;
            }

            if ch == '(' {
                self.tokens.push(Token {
                    r#type: TokenType::OpenParen,
                    value: "(",
                });

                i += 1;

                continue;
            }

            if ch == ')' {
                self.tokens.push(Token {
                    r#type: TokenType::CloseParen,
                    value: ")",
                });

                i += 1;

                continue;
            }

            i += 1;
        }

        Ok(self)
    }
}

#[derive(Debug, Clone)]
struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(ast: &NodeType) -> Result<String, &'static str> {
        let mut asm: String = String::new();

        match &ast {
            NodeType::Program(program) => {
                for function in &program.children {
                    let code = match CodeGenerator::generate(&function) {
                        Ok(code) => code,
                        Err(error) => return Err(error),
                    };

                    asm.push_str(&code);
                }
            }
            NodeType::Function(function) => {
                for statement in &function.children {
                    let function_declaration =
                        format!("\t.globl {}\n{}:\n", function.id, function.id);

                    asm.push_str(&function_declaration);

                    let code = match CodeGenerator::generate(&statement) {
                        Ok(code) => code,
                        Err(error) => return Err(error),
                    };

                    asm.push_str(&code);
                }
            }
            NodeType::Statement(statement) => {
                let expression = match &statement.children[0] {
                    NodeType::Expression(expression) => expression.value,
                    _ => return Err("error in return statement"),
                };

                let code = format!("\tmov\t${expression}, %rax\n\tret\n");

                asm.push_str(&code);
            }
            _ => {}
        }

        Ok(asm)
    }
}

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
