use crate::bub::functions::rules::BubFnsRules;
use crate::bub::functions::variable::BubFnsVariable;
use crate::bub::functions::BubFnsAST;
use mpl::parse::Parse;
use mpl::span::StartAndLenSpan;

pub fn parse(input: &[u8], start_variable: &BubFnsVariable) -> Result<BubFnsAST, BubFnsAST> {
    let all_of_the_span = StartAndLenSpan::<u16, u16>::from_start_len(0, input.len() as u16);
    let rules = &BubFnsRules;
    input.minimal_parse(rules, start_variable, &all_of_the_span)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bub_fns() {
        // Bubble Function
        let input: &[u8] = "1 2 3 0!=1 sin(2*PI*440*n/S)".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFn);
        assert!(result.is_ok());

        // Bubble Functions
        let input: &[u8] = "".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns);
        assert!(result.is_err());

        let input: &[u8] = "    ".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns);
        assert!(result.is_err());

        let input: &[u8] = "1".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns);
        assert!(result.is_err());

        let input: &[u8] = "1 2 3 0!=1".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns);
        assert!(result.is_err());

        let input: &[u8] = "1 2 3 0!=1 sin(2*PI*440*n/S)".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns);
        assert!(result.is_ok());
        let input: &[u8] = "1 2 3 0!=1 sin(2*PI*440*n/S) 1 2 3 0!=1 sin(2*PI*440*n/S)".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns);
        assert!(result.is_ok());
        let input: &[u8] = "1 2 3 4!=5 6 1 2 3 4!=5 6 1 2 3 4!=5 6 1 2 3 4!=5 6".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns);
        assert!(result.is_ok());
    }

    #[test]
    fn expr() {
        let input: &[u8] = "".as_bytes();
        let result = parse(input, &BubFnsVariable::OrOrExpr);
        assert!(result.is_err());

        let input: &[u8] = "E".as_bytes();
        let result = parse(input, &BubFnsVariable::OrOrExpr);
        assert!(result.is_err());

        let input: &[u8] = "2.7<E".as_bytes();
        let result = parse(input, &BubFnsVariable::OrOrExpr);
        assert!(result.is_ok());

        let input: &[u8] = "PI>3.15&&2.7<E".as_bytes();
        let result = parse(input, &BubFnsVariable::OrOrExpr);
        assert!(result.is_ok());

        let input: &[u8] = "PI>3.15&&2.7<E&&2".as_bytes();
        let result = parse(input, &BubFnsVariable::OrOrExpr);
        assert!(result.is_err());

        let input: &[u8] = "X<=1.1&&Y!=1.0||N<3".as_bytes();
        let result = parse(input, &BubFnsVariable::OrOrExpr);
        assert!(result.is_ok());

        let input: &[u8] =
            "X<=1.1&&Y!=1.0||Z==0&&n<2*4||z+5*PI>9||y<=1.1&&sin2*cos(1/2*PI)!=sinPI*n&&tanS>=1.0"
                .as_bytes();
        let result = parse(input, &BubFnsVariable::OrOrExpr);
        assert!(result.is_ok());
    }

    #[test]
    fn sum() {
        let input: &[u8] = "".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_err());

        let input: &[u8] = "()".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_err());

        let input: &[u8] = "1**2".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_err());

        let input: &[u8] = "9".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "-1".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "E".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "1+2".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "1.0-2.0+3.5".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "1.0-2.0+3.5*4.2*lg5".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "sin(2*PI*440*n/S)".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "1.2*(5+-+98.76543210)/sin(n/S)/1000".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "X-Y+Z*x/y*(z)+tanPI-(cos((E+n-N)/S)+ln2+lg(5))+-1.0".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());

        let input: &[u8] = "4*(23-21)+54+(2343+n*(S-2))+sinPI".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());
        let input: &[u8] = "4*(23-21)+54+(2343+t*(S-2)+sinPI".as_bytes();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_err());

        let input = &[
            "1.2*(5+98.765432)/sin(n/S)*b".as_bytes(),
            &1.0_f64.to_le_bytes(),
            "+cos(PI)+1".as_bytes(),
        ]
        .concat();
        let result = parse(input, &BubFnsVariable::Sum);
        assert!(result.is_ok());
    }
}
