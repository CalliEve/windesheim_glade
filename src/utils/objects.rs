use super::{regex::*, weights::*};
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum LangObject {
    Zolang(Zolang),
    Als(Als),
    Action(Steps),
    Assignment(Assignment),
    Print(Print),
}

#[derive(Clone, Debug)]
pub enum ExpressionVar {
    Variable(String),
    Int(i32),
    KleurOog,
    ZwOog,
    Kompas,
    Expression(Box<IntExpression>),
}

impl ExpressionVar {
    pub fn parse(text: String, line: usize, ctx: &Context) -> ExpressionVar {
        if let Ok(i) = i32::from_str_radix(text.trim(), 10) {
            ExpressionVar::Int(i)
        } else if text.trim().chars().count() == 1 {
            ExpressionVar::Variable(text.trim().to_owned())
        } else if INT_EXPRESSION.is_match(&text) {
            ExpressionVar::Expression(Box::new(IntExpression::parse(text, line, ctx)))
        } else {
            panic!("invalid expression to assign on line {}", line)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Comparer {
    Equal,
    NotEqual,
    GreaterThan,
    SmallerThan,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Product,
    Divide,
    Remainder,
}

#[derive(Clone, Debug)]
pub enum Steps {
    StepForwards,
    StepBackwards,
    TurnLeft,
    TurnRight,
}

#[derive(Clone, Debug)]
pub enum Hardware {
    Kompas,
    ZwOog,
    KleurOog,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub file_text: String,
    pub code: CodeBlock,
    pub points: i32,
    pub variables: HashMap<String, Option<i32>>,
    pub useable: Vec<Hardware>,
}

impl Context {
    pub fn new(file_text: String) -> Context {
        let mut ctx = Context {
            file_text: file_text.clone(),
            code: CodeBlock {
                text: file_text.clone(),
                line: 0,
                objects: Vec::new(),
            },
            points: 0,
            variables: HashMap::new(),
            useable: Vec::new(),
        };

        ctx.parse_variables();
        ctx.code = CodeBlock::parse(file_text, 0, &mut ctx);
        ctx
    }

    fn parse_variables(&mut self) {
        let lines: Vec<&str> = Regex::new("\n").unwrap().split(&self.file_text).collect();

        for (i, line) in lines.iter().enumerate() {
            if INSTANTIATOR.is_match(line) {
                let c = INSTANTIATOR.captures(line).unwrap();
                let name = c.get(1).unwrap().as_str().to_owned();

                match name.as_ref() {
                    "kompas" => {
                        self.useable.push(Hardware::Kompas);
                        self.points += KOMPAS_HARDWARE;
                        continue;
                    },
                    "zwOog" => {
                        self.useable.push(Hardware::ZwOog);
                        self.points += ZWOOG_HARDWARE;
                        continue;
                    },
                    "kleurOog" => {
                        self.useable.push(Hardware::KleurOog);
                        self.points += KLEUROOG_HARDWARE;
                        continue;
                    },
                    _ => {},
                }

                self.variables.insert(name, None);
                self.points += VAR_HARDWARE;
            }
        }
    }

    pub fn execute(&mut self) {
        let code = self.code.clone();
        code.execute(self);
        self.code = code;
    }

    pub fn get_var(&self, name: &str) -> i32 {
        if let Some(v) = self.variables.get(name) {
            if let Some(i) = v {
                return *i;
            }
        }
        panic!("no variable named {} defined", name)
    }
}

#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub objects: Vec<LangObject>,
    pub text: String,
    pub line: usize,
}

impl CodeBlock {
    pub fn parse(text: String, line_nr: usize, ctx: &mut Context) -> Self {
        let lines: Vec<&str> = Regex::new("\n").unwrap().split(&text).collect();
        let mut objects: Vec<LangObject> = Vec::new();
        let mut in_block = false;

        for (i, line) in lines.iter().enumerate() {
            if in_block {
                if END_BLOCK.is_match(line) {
                    in_block = false
                }
                continue;
            } else if ASSIGNMENT.is_match(line) {
                objects.push(Assignment::parse(String::from(*line), line_nr + i + 1, ctx))
            } else if PRINT.is_match(line) {
                objects.push(Print::parse(String::from(*line), line_nr + i + 1, ctx))
            } else if String::from(*line).trim().is_empty() {
                continue;
            } else if ZOLANG_ID.is_match(line) {
                let mut full: String = String::new();
                lines.iter().for_each(|x| {
                    full.push_str(*x);
                    full.push_str("\n");
                });
                objects.push(Zolang::parse(full, line_nr + i + 1, ctx));
                in_block = true;
            } else if ALS_ID.is_match(line) {
                let mut full: String = String::new();
                lines.iter().for_each(|x| {
                    full.push_str(*x);
                    full.push_str("\n");
                });
                objects.push(Als::parse(full, line_nr + i + 1, ctx));
                in_block = true;
            }
        }

        Self {
            objects,
            text,
            line: line_nr,
        }
    }

    pub fn execute(&self, ctx: &mut Context) {
        for obj in &self.objects {
            match obj {
                LangObject::Zolang(v) => v.run_loop(ctx),
                LangObject::Assignment(v) => v.calc(ctx),
                LangObject::Als(v) => v.run_if(ctx),
                LangObject::Print(v) => v.print(ctx),
                _ => panic!(
                    "found a non-valid statement in the code block starting at {}",
                    self.line
                ),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoolExpression {
    pub left: ExpressionVar,
    pub comparer: Comparer,
    pub right: ExpressionVar,
    pub line: usize,
}

impl BoolExpression {
    pub fn parse(text: String, line: usize, ctx: &Context) -> BoolExpression {
        let c = BOOL_EXPRESSION.captures(&text).unwrap();
        let left_str = c.get(1).unwrap().as_str().to_owned();
        let right_str = c.get(3).unwrap().as_str().to_owned();

        let comparer = match c.get(2).unwrap().as_str() {
            "==" => Comparer::Equal,
            "!=" => Comparer::NotEqual,
            ">" => Comparer::GreaterThan,
            "<" => Comparer::SmallerThan,
            _ => panic!("unknown comparer"),
        };

        BoolExpression {
            left: ExpressionVar::parse(left_str, line, ctx),
            right: ExpressionVar::parse(right_str, line, ctx),
            line,
            comparer,
        }
    }

    pub fn calc(&self, ctx: &Context) -> bool {
        println!("checking");
        let left = match &self.left {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            _ => panic!(
                "these values are not currently supported for an bool expression, expression at line: {}",
                self.line
            ), // TODO: add the special vars
        };

        let right = match &self.right {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            _ => panic!(
                "these values are not currently supported for an bool expression, expression at line: {}",
                self.line
            ), // TODO: add the special vars
        };

        // println!("left: {}, op: {:?}, right: {}", left, &self.comparer, right);

        match &self.comparer {
            Comparer::Equal => left == right,
            Comparer::NotEqual => left != right,
            Comparer::GreaterThan => left > right,
            Comparer::SmallerThan => left < right,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntExpression {
    pub left: ExpressionVar,
    pub operator: Operator,
    pub right: ExpressionVar,
    pub line: usize,
}

impl IntExpression {
    pub fn parse(text: String, line: usize, ctx: &Context) -> Self {
        let c = INT_EXPRESSION.captures(&text).unwrap();
        let left_str = c.get(1).unwrap().as_str().to_owned();
        let right_str = c.get(3).unwrap().as_str().to_owned();

        let operator = match c.get(2).unwrap().as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "/" => Operator::Divide,
            "*" => Operator::Product,
            "%" => Operator::Remainder,
            _ => panic!("unknown operator"),
        };

        // println!(
        //     "left: {}, op: {:?}, right: {}",
        //     left_str, &operator, right_str
        // );

        Self {
            left: ExpressionVar::parse(left_str, line, ctx),
            right: ExpressionVar::parse(right_str, line, ctx),
            line,
            operator,
        }
    }

    pub fn calc(&self, ctx: &Context) -> i32 {
        let left = match &self.left {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            _ => panic!(
                "these values are not currently supported for an int expression, expression at line: {}",
                self.line
            ), // TODO: add the special vars
        };

        let right = match &self.right {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            _ => panic!(
                "these values are not currently supported for an int expression, expression at line: {}",
                self.line
            ), // TODO: add the special vars
        };

        // println!("left: {}, op: {:?}, right: {}", left, &self.operator, right);

        match &self.operator {
            Operator::Plus => left + right,
            Operator::Minus => left - right,
            Operator::Product => left * right,
            Operator::Divide => left / right,
            Operator::Remainder => left % right,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Zolang {
    pub expression: BoolExpression,
    pub block: CodeBlock,
    pub line: usize,
}

impl Zolang {
    pub fn parse(text: String, line: usize, ctx: &mut Context) -> LangObject {
        ctx.points += ZOLANG_SOFTWARE;
        let c = ZOLANG.captures(&text).unwrap();
        let expr_str = c.get(1).unwrap().as_str().to_owned();
        let codeblock_str = c.get(2).unwrap().as_str().to_owned();

        LangObject::Zolang(Self {
            expression: BoolExpression::parse(expr_str, line, ctx),
            block: CodeBlock::parse(codeblock_str, line + 1, ctx),
            line,
        })
    }

    pub fn run_loop(&self, ctx: &mut Context) {
        while self.expression.calc(ctx) {
            self.block.execute(ctx)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Als {
    pub expression: BoolExpression,
    pub if_block: CodeBlock,
    pub else_block: Option<CodeBlock>,
    pub line: usize,
}

impl Als {
    pub fn parse(text: String, line: usize, ctx: &mut Context) -> LangObject {
        ctx.points += ALS_SOFTWARE;
        let c = ALS.captures(&text).unwrap();
        let expr_str = c.get(1).unwrap().as_str().to_owned();
        let if_codeblock_str = c.get(2).unwrap().as_str().to_owned();

        let else_block = if c.get(4).is_some() {
            let else_codeblock_str = c.get(4).unwrap().as_str().to_owned();
            Some(CodeBlock::parse(else_codeblock_str, line + 1, ctx))
        } else {
            None
        };

        LangObject::Als(Self {
            expression: BoolExpression::parse(expr_str, line, ctx),
            if_block: CodeBlock::parse(if_codeblock_str, line + 1, ctx),
            else_block,
            line,
        })
    }

    pub fn run_if(&self, ctx: &mut Context) {
        if self.expression.calc(ctx) {
            self.if_block.execute(ctx)
        } else if self.else_block.is_some() {
            self.else_block.as_ref().unwrap().execute(ctx)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub var: String,
    pub expression: ExpressionVar,
    pub line: usize,
}

impl Assignment {
    pub fn parse(text: String, line: usize, ctx: &mut Context) -> LangObject {
        ctx.points += ASSIGNMENT_SOFTWARE;
        let c = ASSIGNMENT.captures(&text).unwrap();
        let name = c.get(1).unwrap().as_str();
        let to_assign = c.get(2).unwrap().as_str().to_owned();

        let expression = ExpressionVar::parse(to_assign, line, ctx);

        LangObject::Assignment(Self {
            var: name.to_owned(),
            line,
            expression,
        })
    }

    pub fn calc(&self, ctx: &mut Context) {
        let value = match &self.expression {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Expression(exp) => exp.calc(ctx),
            ExpressionVar::Int(i) => *i,
            _ => panic!(
                "these values are not currently supported for an assignment, assignment at line: {}",
                self.line
            ),
        };

        if ctx.variables.get(&self.var).is_some() {
            ctx.variables.insert(self.var.clone(), Some(value));
            return;
        }

        panic!(
            "tried to assign to a non-existant variable at line {}",
            self.line
        )
    }
}

#[derive(Clone, Debug)]
pub struct Print {
    pub expression: ExpressionVar,
    pub line: usize,
}

impl Print {
    pub fn parse(text: String, line: usize, ctx: &Context) -> LangObject {
        let c = PRINT.captures(&text).unwrap();
        let to_print = c.get(1).unwrap().as_str().to_owned();

        let expression = ExpressionVar::parse(to_print, line, ctx);

        LangObject::Print(Self {
            expression,
            line,
        })
    }

    pub fn print(&self, ctx: &mut Context) {
        let value = match &self.expression {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Expression(exp) => exp.calc(ctx),
            ExpressionVar::Int(i) => *i,
            _ => panic!(
                "these values are not currently supported for an assignment, assignment at line: {}",
                self.line
            ),
        };

        println!("at line {} print: {}", self.line, value)
    }
}
