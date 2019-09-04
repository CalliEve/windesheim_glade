use super::{map::*, regex::*, weights::*};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum LangObject {
    Zolang(Zolang),
    Als(Als),
    Assignment(Assignment),
    Print(Print),
    StepForwards,
    StepBackwards,
    TurnLeft,
    TurnRight,
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
    pub fn parse(text: &str, line: usize, ctx: &Context) -> Self {
        if let Ok(i) = i32::from_str_radix(text.trim(), 10) {
            Self::Int(i)
        } else if text.trim().chars().count() == 1 {
            if ctx.variables.get(text.trim()).is_none() {
                panic!("variable {} is not defined at line {}", text.trim(), line)
            }
            Self::Variable(text.trim().to_owned())
        } else if text.trim() == "kompas" && ctx.useable.contains(&Hardware::Kompas) {
            Self::Kompas
        } else if text.trim() == "zwOog" && ctx.useable.contains(&Hardware::ZwOog) {
            Self::ZwOog
        } else if text.trim() == "kleurOog" && ctx.useable.contains(&Hardware::KleurOog) {
            Self::KleurOog
        } else if INT_EXPRESSION.is_match(&text) {
            Self::Expression(Box::new(IntExpression::parse(&text, line, ctx)))
        } else {
            panic!("invalid expression on line {}", line)
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

#[derive(Clone, Debug, PartialEq)]
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
    pub glade: Glade,
}

impl Context {
    pub fn new(file_text: &str, glade: Glade) -> Self {
        Self {
            file_text: file_text.to_owned(),
            code: CodeBlock {
                text: file_text.to_owned(),
                line: 0,
                objects: Vec::new(),
            },
            points: 0,
            variables: HashMap::new(),
            useable: Vec::new(),
            glade,
        }
    }

    pub fn parse(&mut self) {
        self.parse_variables();
        self.code = CodeBlock::parse(self.file_text.clone(), 0, self);
    }

    fn parse_variables(&mut self) {
        let text = self.file_text.clone();
        let lines: Vec<&str> = text.split('\n').collect();

        for line in &lines {
            if INSTANTIATOR.is_match(line) {
                let c = INSTANTIATOR.captures(line).unwrap();
                let name = c.get(1).unwrap().as_str().to_owned();

                match name.as_ref() {
                    "kompas" => {
                        self.useable.push(Hardware::Kompas);
                        self.add_points(KOMPAS_HARDWARE);
                        continue;
                    },
                    "zwOog" => {
                        self.useable.push(Hardware::ZwOog);
                        self.add_points(ZWOOG_HARDWARE);
                        continue;
                    },
                    "kleurOog" => {
                        self.useable.push(Hardware::KleurOog);
                        self.add_points(KLEUROOG_HARDWARE);
                        continue;
                    },
                    _ => {},
                }

                self.variables.insert(name, None);
                self.add_points(VAR_HARDWARE);
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

    pub fn add_points(&mut self, p: i32) {
        self.points += p;
        // println!("points: {}", self.points);
        if self.points > 2020 {
            panic!(
                "Used up too much of your money! your total expenses are: {}",
                self.points
            )
        }
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
        let lines: Vec<&str> = text.split('\n').collect();
        let mut objects: Vec<LangObject> = Vec::new();
        let mut open_brackets = 0;

        for (i, line) in lines.iter().enumerate() {
            if open_brackets > 0 {
                if line.contains('}') {
                    open_brackets -= 1;
                }
                if line.contains('{') {
                    open_brackets += 1;
                }
            } else if FORBIDDEN_END_BLOCK.is_match(&line) {
                panic!(
                    "a closing }} has to be on a newline! it isn't at line {}",
                    line_nr + i + 1
                )
            } else if HANGING_EXPRESSION.is_match(&line) {
                panic!(
                    "an operator or comparer needs something to operate with or compare to! it doesn't at line {}",
                    line_nr + i + 1
                )
            } else if line.trim() == "draaiLinks" {
                ctx.add_points(ACTION_SOFTWARE);
                objects.push(LangObject::TurnLeft)
            } else if line.trim() == "draaiRechts" {
                ctx.add_points(ACTION_SOFTWARE);
                objects.push(LangObject::TurnRight)
            } else if line.trim() == "stapVooruit" {
                ctx.add_points(ACTION_SOFTWARE);
                objects.push(LangObject::StepForwards)
            } else if line.trim() == "stapAchteruit" {
                ctx.add_points(ACTION_SOFTWARE);
                objects.push(LangObject::StepBackwards)
            } else if ASSIGNMENT.is_match(line) {
                objects.push(Assignment::parse(*line, line_nr + i + 1, ctx))
            } else if PRINT.is_match(line) {
                objects.push(Print::parse(*line, line_nr + i + 1, ctx))
            } else if String::from(*line).trim().is_empty() {
                continue;
            } else if ZOLANG_ID.is_match(line) {
                let mut full: String = String::new();
                lines.iter().enumerate().for_each(|(k, x)| {
                    if k >= i {
                        full.push_str(*x);
                        full.push_str("\n");
                    }
                });
                open_brackets += 1;
                println!("parsing zolang");
                objects.push(Zolang::parse(&full, line_nr + i + 1, ctx));
            } else if ALS_ID.is_match(line) {
                let mut full: String = String::new();
                lines.iter().enumerate().for_each(|(k, x)| {
                    if k >= i {
                        full.push_str(*x);
                        full.push_str("\n");
                    }
                });
                open_brackets += 1;
                println!("parsing als");
                objects.push(Als::parse(&full, line_nr + i + 1, ctx));
            } else if INSTANTIATOR.is_match(line) {
                continue;
            } else {
                panic!("\ninvalid text on line {}: {}\n", line_nr + i + 1, line)
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
                LangObject::TurnLeft => {
                    ctx.glade.turn_left(false);
                    ctx.add_points(TURNLEFT_USAGE);
                },
                LangObject::TurnRight => {
                    ctx.glade.turn_right(false);
                    ctx.add_points(TURNRIGHT_USAGE);
                },
                LangObject::StepForwards => {
                    // println!("step forward");
                    match ctx.glade.forward() {
                        Result::Ok(a) => ctx.points -= a,
                        Result::Err(_) => {
                            println!(
                                "WARNING: collided against obstacle! location: {}, {}, direction: {:?}",
                                ctx.glade.griever.x + 1, ctx.glade.griever.y + 1, ctx.glade.griever.direction
                            );
                            ctx.add_points(PUSH_OBSTACLE)
                        },
                    };
                    if ctx.glade.success() {
                        println!("\nSUCCESS!\ncosts: {}", 2020 - ctx.points);
                        std::process::exit(0)
                    };
                },
                LangObject::StepBackwards => {
                    // println!("step backwards");
                    match ctx.glade.backward() {
                        Result::Ok(a) => ctx.points -= a,
                        Result::Err(_) => {
                            println!(
                                "WARNING: collided against obstacle! location: {}, {}, direction: {:?}",
                                ctx.glade.griever.x + 1, ctx.glade.griever.y + 1, ctx.glade.griever.direction
                            );
                            ctx.add_points(PUSH_OBSTACLE)
                        },
                    };
                    if ctx.glade.success() {
                        println!("\nSUCCESS!\ncosts: {}", 2020 - ctx.points);
                        std::process::exit(0)
                    };
                },
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
    pub fn parse(text: &str, line: usize, ctx: &Context) -> Self {
        let c = BOOL_EXPRESSION
            .captures(&text)
            .unwrap_or_else(|| panic!("there is an error in the bool expression at line {}", line));
        let left_str = c.get(1).unwrap().as_str();
        let right_str = c.get(3).unwrap().as_str();

        let comparer = match c.get(2).unwrap().as_str() {
            "==" => Comparer::Equal,
            "!=" => Comparer::NotEqual,
            ">" => Comparer::GreaterThan,
            "<" => Comparer::SmallerThan,
            _ => panic!("unknown comparer"),
        };

        Self {
            left: ExpressionVar::parse(left_str, line, ctx),
            right: ExpressionVar::parse(right_str, line, ctx),
            line,
            comparer,
        }
    }

    pub fn calc(&self, ctx: &mut Context) -> bool {
        ctx.add_points(COMPARISON_USAGE);
        let left = match &self.left {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            ExpressionVar::Kompas => {
                ctx.add_points(KOMPAS_USAGE);
                ctx.glade.griever.kompas()
            },
            ExpressionVar::KleurOog => {
                ctx.add_points(KLEUROOG_USAGE);
                ctx.glade.color_eye()
            },
            ExpressionVar::ZwOog => {
                ctx.add_points(ZWOOG_USAGE);
                ctx.glade.bw_eye()
            },
        };

        let right = match &self.right {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            ExpressionVar::Kompas => {
                ctx.add_points(KOMPAS_USAGE);
                ctx.glade.griever.kompas()
            },
            ExpressionVar::KleurOog => {
                ctx.add_points(KLEUROOG_USAGE);
                ctx.glade.color_eye()
            },
            ExpressionVar::ZwOog => {
                ctx.add_points(ZWOOG_USAGE);
                ctx.glade.bw_eye()
            },
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
    pub fn parse(text: &str, line: usize, ctx: &Context) -> Self {
        let c = INT_EXPRESSION.captures(&text).unwrap();
        let left_str = c.get(1).unwrap().as_str();
        let right_str = c.get(3).unwrap().as_str();

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

    pub fn calc(&self, ctx: &mut Context) -> i32 {
        ctx.add_points(OPERATION_USAGE);
        let left = match &self.left {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            ExpressionVar::Kompas => {
                ctx.add_points(KOMPAS_USAGE);
                ctx.glade.griever.kompas()
            },
            ExpressionVar::KleurOog => {
                ctx.add_points(KLEUROOG_USAGE);
                ctx.glade.color_eye()
            },
            ExpressionVar::ZwOog => {
                ctx.add_points(ZWOOG_USAGE);
                ctx.glade.bw_eye()
            },
        };

        let right = match &self.right {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Int(var) => *var,
            ExpressionVar::Expression(deeper) => deeper.calc(ctx),
            ExpressionVar::Kompas => {
                ctx.add_points(KOMPAS_USAGE);
                ctx.glade.griever.kompas()
            },
            ExpressionVar::KleurOog => {
                ctx.add_points(KLEUROOG_USAGE);
                ctx.glade.color_eye()
            },
            ExpressionVar::ZwOog => {
                ctx.add_points(ZWOOG_USAGE);
                ctx.glade.bw_eye()
            },
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
    pub fn parse(text: &str, line: usize, ctx: &mut Context) -> LangObject {
        ctx.add_points(ZOLANG_SOFTWARE);
        let c = ZOLANG_ID.captures(&text).unwrap_or_else(|| {
            panic!(
                "the zolang starting at line {} has a mistake in the syntax",
                line
            )
        });;
        let expr_str = c.get(1).unwrap().as_str().to_owned();

        let lines: Vec<&str> = text.split('\n').collect();
        let mut bracket_open = 0;
        let mut codeblock: Vec<&str> = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                bracket_open += 1;
                continue;
            }

            if line.contains('}') {
                bracket_open -= 1;
                if bracket_open == 0 {
                    break;
                }
            }
            if line.contains('{') {
                bracket_open += 1;
            }

            codeblock.push(line)
        }

        LangObject::Zolang(Self {
            expression: BoolExpression::parse(&expr_str, line, ctx),
            block: CodeBlock::parse(codeblock.join("\n"), line, ctx),
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
    pub fn parse(text: &str, line: usize, ctx: &mut Context) -> LangObject {
        ctx.add_points(ALS_SOFTWARE);
        let mut bracket_open = 0;
        let mut anders = false;

        let c = ALS_ID.captures(&text).unwrap_or_else(|| {
            panic!(
                "the als starting at line {} has a mistake in the syntax",
                line
            )
        });
        let expr_str = c.get(1).unwrap().as_str().to_owned();

        let lines: Vec<&str> = text.split('\n').collect();
        let mut if_codeblock_str: Vec<&str> = Vec::new();
        let mut else_block_str: Vec<&str> = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                bracket_open += 1;
                continue;
            }
            if line.trim() == "} anders {" && bracket_open == 1 {
                anders = true;
            } else {
                if line.contains('}') {
                    bracket_open -= 1;
                    if bracket_open == 0 {
                        break;
                    }
                }
                if line.contains('{') {
                    bracket_open += 1;
                }

                if anders {
                    else_block_str.push(line)
                } else {
                    if_codeblock_str.push(line)
                }
            }
        }

        let else_block = if else_block_str.is_empty() {
            None
        } else {
            Some(CodeBlock::parse(else_block_str.join("\n"), line + 1, ctx))
        };

        LangObject::Als(Self {
            expression: BoolExpression::parse(&expr_str, line, ctx),
            if_block: CodeBlock::parse(if_codeblock_str.join("\n"), line + 1, ctx),
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
    pub fn parse(text: &str, line: usize, ctx: &mut Context) -> LangObject {
        ctx.add_points(ASSIGNMENT_SOFTWARE);
        let c = ASSIGNMENT.captures(&text).unwrap_or_else(|| {
            panic!(
                "there is an error in the syntax of the assignment at line {}",
                line
            )
        });
        let name = c.get(1).unwrap().as_str().to_owned();
        let to_assign = c.get(2).unwrap().as_str();

        let expression = ExpressionVar::parse(to_assign, line, ctx);

        LangObject::Assignment(Self {
            var: name,
            line,
            expression,
        })
    }

    pub fn calc(&self, ctx: &mut Context) {
        ctx.add_points(ASSIGNMENT_USAGE);
        let value = match &self.expression {
            ExpressionVar::Variable(inner_var) => ctx.get_var(&inner_var),
            ExpressionVar::Expression(exp) => exp.calc(ctx),
            ExpressionVar::Int(i) => *i,
            ExpressionVar::Kompas => {
                ctx.add_points(KOMPAS_USAGE);
                ctx.glade.griever.kompas()
            },
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
    pub fn parse(text: &str, line: usize, ctx: &Context) -> LangObject {
        let c = PRINT.captures(&text).unwrap();
        let to_print = c.get(1).unwrap().as_str();

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
            ExpressionVar::Kompas => ctx.glade.griever.kompas(),
            ExpressionVar::KleurOog => ctx.glade.color_eye(),
            ExpressionVar::ZwOog => ctx.glade.bw_eye(),
        };

        println!("at line {} print: {}", self.line, value)
    }
}
