use crate::bub::function::rules::FunctionRules;
use crate::bub::function::variable::FunctionVariable;
use mpl::parse::Parse;
use mpl::span::StartAndLenSpan;
use mpl::trees::AST;

pub struct FunctionParser;

impl FunctionParser {
    pub fn parse(
        input: &[u8],
        start_variable: &FunctionVariable,
    ) -> Result<
        AST<FunctionVariable, StartAndLenSpan<u16, u16>, ()>,
        AST<FunctionVariable, StartAndLenSpan<u16, u16>, ()>,
    > {
        let all_of_the_span = StartAndLenSpan::<u16, u16>::from_start_len(0, input.len() as u16);
        let rules = &FunctionRules;
        input.minimal_parse(rules, start_variable, &all_of_the_span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr() {
        let input: &[u8] = "1+2".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_err());
    }
}
