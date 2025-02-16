use crate::Token;
use crate::TokenType;

#[derive(Debug, Clone)]
pub enum NodeType {
    Program(ProgramNode),
    Function(FunctionNode),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub children: Vec<NodeType>,
}

impl ProgramNode {
    pub fn new(children: Vec<NodeType>) -> Self {
        Self { children }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionNode {
    pub id: String,
    pub children: Vec<NodeType>,
}

impl FunctionNode {
    pub fn new(id: String, children: Vec<NodeType>) -> Self {
        Self { id, children }
    }
}

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub children: Vec<NodeType>,
}

impl StatementNode {
    pub fn new(children: Vec<NodeType>) -> Self {
        Self { children }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionNode {
    pub value: u32,
}

impl ExpressionNode {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
    pub current: u32,
}

impl Parser<'_> {
    pub fn new<'a>(tokens: Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            tokens,
            current: 0 as u32,
        }
    }

    pub fn parse(&mut self) -> Result<NodeType, &'static str> {
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
