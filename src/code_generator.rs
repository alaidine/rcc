use crate::NodeType;

#[derive(Debug, Clone)]
pub struct CodeGenerator;

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
                    NodeType::Constant(expression) => expression.value,
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
