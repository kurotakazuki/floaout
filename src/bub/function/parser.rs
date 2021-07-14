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
        let input: &[u8] = "".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_err());

        let input: &[u8] = "E".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());

        let input: &[u8] = "1+2".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());

        let input: &[u8] = "1.0-2.0+3.5".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());

        let input: &[u8] = "1.0-2.0+3.5*4.2*lg5".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());

        let input: &[u8] = "sin(2*PI*440*t/F)".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());

        let input: &[u8] = "1.2*(5+98.765432)/sin(t/F)/1000".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());

        let input: &[u8] = "X-Y+Z*x/y*(z)+tanPI-(cos((E+t-T)/F)+ln2+lg(5))+-1.0".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());

        let input: &[u8] = "4*(23-21)+54+(2343+t*(F-2))+sinPI".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_ok());
        let input: &[u8] = "4*(23-21)+54+(2343+t*(F-2)+sinPI".as_bytes();
        let result = FunctionParser::parse(input, &FunctionVariable::Expression);
        assert!(result.is_err());
    }
}
