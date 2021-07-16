use mpl::span::StartAndLenSpan;
use mpl::trees::AST;

pub use self::interpreter::FunctionInterpreter;
pub use self::parse::parse;
pub use self::rules::FunctionRules;
pub use self::variable::FunctionVariable;

mod interpreter;
mod output;
mod parse;
mod rules;
mod variable;

type FunctionAST = AST<FunctionVariable, StartAndLenSpan<u16, u16>, f64>;
