use mpl::span::StartAndLenSpan;
use mpl::trees::AST;

pub use self::interpreter::FunctionInterpreter;
pub use self::output::FunctionOutput;
pub use self::parse::parse;
pub use self::rules::FunctionRules;
pub use self::variable::FunctionVariable;

mod interpreter;
mod output;
mod parse;
mod rules;
mod variable;

pub type FunctionAST = AST<FunctionVariable, StartAndLenSpan<u16, u16>, FunctionOutput>;

#[derive(Clone, Debug, PartialEq)]
pub struct BubbleFunction {
    // The root node variable is `Sum`.
    bubble_absolute_coordinates: (FunctionAST, FunctionAST, FunctionAST),
    // The root node variable is `OrOrExpression`.
    domain: FunctionAST,
    // The root node variable is `Sum`.
    volume: FunctionAST,
}

pub type BubbleFunctions = Vec<BubbleFunction>;
