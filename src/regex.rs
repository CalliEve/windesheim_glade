use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref INSTANTIATOR: Regex =
        Regex::new(r"[ \t]*gebruik (kleurOog|zwOog|kompas|[a-z]) *").unwrap();
    pub static ref PRINT: Regex =
        Regex::new(r"[ \t]*print ([0-9]+|kompas|zwOog|kleurOog|[a-z]) *").unwrap();
    pub static ref LONE_VAR: Regex = Regex::new(r"[ \t]+([a-z])[ \n$]").unwrap();
    pub static ref INT_EXPRESSION: Regex =
        Regex::new(r"[ \t]*(.*) (\+|-|\*|/|%) ([0-9]+|kompas|zwOog|kleurOog|[a-z])").unwrap();
    pub static ref BOOL_EXPRESSION: Regex =
        Regex::new(r"[ \t]*([0-9]+|kompas|zwOog|kleurOog|[a-z]) (==|!=|>|<) (.*)").unwrap();
    pub static ref ASSIGNMENT: Regex = Regex::new(r"[ \t]*([a-z]) = (.*)").unwrap();
    pub static ref ALS_ID: Regex = Regex::new(r"[ \t]*als (.*?) \{").unwrap();
    pub static ref ZOLANG_ID: Regex = Regex::new(r"[ \t]*zolang (.*?) \{").unwrap();
    pub static ref END_BLOCK: Regex = Regex::new(r"[ \t]*\} *$").unwrap();
    pub static ref FORBIDDEN_END_BLOCK: Regex =
        Regex::new(r"[ \t]*[^\n \t]+[ \t]*\}[ \t]*$").unwrap();
    pub static ref HANGING_EXPRESSION: Regex =
        Regex::new(r".*(==|!=|>|<|\+|-|\*|/|%)[ \t}{]*$").unwrap();
}
