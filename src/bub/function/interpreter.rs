use crate::bub::function::{FunctionAST, FunctionVariable::*};
use mpl::choices::Choice;
use mpl::trees::Node::*;

pub struct FunctionInterpreter {
    uppercase_x: f64,
    uppercase_y: f64,
    uppercase_z: f64,
    lowercase_x: f64,
    lowercase_y: f64,
    lowercase_z: f64,
    /// Number of frames starting from the file. Absolute Time
    uppercase_t: f64,
    /// Number of frames starting from the function. Relative Time
    lowercase_t: f64,
    uppercase_f: f64,
}

impl FunctionInterpreter {
    pub fn new(
        speaker_absolute_coordinates: (f64, f64, f64),
        bubble_absolute_coordinates: (f64, f64, f64),
        absolute_time: f64,
        relative_time: f64,
        samples_per_sec: f64,
    ) -> Self {
        Self {
            uppercase_x: speaker_absolute_coordinates.0,
            uppercase_y: speaker_absolute_coordinates.1,
            uppercase_z: speaker_absolute_coordinates.2,
            lowercase_x: speaker_absolute_coordinates.0 - bubble_absolute_coordinates.0,
            lowercase_y: speaker_absolute_coordinates.1 - bubble_absolute_coordinates.1,
            lowercase_z: speaker_absolute_coordinates.2 - bubble_absolute_coordinates.2,
            uppercase_t: absolute_time,
            lowercase_t: relative_time,
            uppercase_f: samples_per_sec,
        }
    }

    pub fn eval_or_or_expr(&self, ast: &FunctionAST) -> Result<bool, ()> {
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is OrOrExpression

        match &*internal.equal {
            Choice::First(first) => {
                let lhs = self.eval_and_and_expr(&first.lhs)?;

                let or_or_expr1 = first.rhs.as_first().unwrap();
                let rhs = self.eval_or_or_expr(&or_or_expr1.rhs)?;

                Ok(lhs || rhs)
            }
            Choice::Second(second) => self.eval_and_and_expr(&second.0),
        }
    }

    pub fn eval_and_and_expr(&self, ast: &FunctionAST) -> Result<bool, ()> {
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is AndAndExpression

        match &*internal.equal {
            Choice::First(first) => {
                let lhs = self.eval_comparison_expr(&first.lhs)?;

                let and_and_expr1 = first.rhs.as_first().unwrap();
                let rhs = self.eval_and_and_expr(&and_and_expr1.rhs)?;

                Ok(lhs && rhs)
            }
            Choice::Second(second) => self.eval_comparison_expr(&second.0),
        }
    }

    pub fn eval_comparison_expr(&self, ast: &FunctionAST) -> Result<bool, ()> {
        // TODO: Check whether variable is ComparisonExpression

        let first = ast.as_first().unwrap();

        let lhs = self.eval_plus_or_minus_expr(&first.lhs)?;

        let comparison_expr1 = first.rhs.as_first().unwrap();
        let comparison_v = &comparison_expr1.lhs;
        let rhs = self.eval_plus_or_minus_expr(&comparison_expr1.rhs)?;

        match comparison_v.as_internal().expect("plus or minus").value.0 {
            EqEq => Ok((lhs - rhs).abs() < f64::EPSILON),
            Ne => Ok((lhs - rhs).abs() > f64::EPSILON),
            Ge => Ok(lhs >= rhs),
            Le => Ok(lhs <= rhs),
            Gt => Ok(lhs > rhs),
            Lt => Ok(lhs < rhs),
            _ => unreachable!(),
        }
    }

    pub fn eval_plus_or_minus_expr(&self, ast: &FunctionAST) -> Result<f64, ()> {
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is PlusOrMinusExpression

        match &*internal.equal {
            Choice::First(first) => {
                let lhs = self.eval_term(&first.lhs)?;

                let plus_or_minus_expr1 = first.rhs.as_first().unwrap();
                let plus_or_minus_v = &plus_or_minus_expr1.lhs;
                let rhs = self.eval_plus_or_minus_expr(&plus_or_minus_expr1.rhs)?;

                match plus_or_minus_v
                    .as_internal()
                    .expect("plus or minus")
                    .value
                    .0
                {
                    Plus => Ok(lhs + rhs),
                    Minus => Ok(lhs - rhs),
                    _ => unreachable!(),
                }
            }
            Choice::Second(second) => self.eval_term(&second.0),
        }
    }

    pub fn eval_term(&self, ast: &FunctionAST) -> Result<f64, ()> {
        println!("{}", ast);
        let term_v = ast.as_first().unwrap();

        // TODO: Check whether variable is Term

        let mut lhs = self.eval_factor(&term_v.lhs)?;

        let mut zero_or_more = &term_v.rhs;

        // zero or more star or slash and term
        loop {
            match &zero_or_more.node {
                // StarOrSlashAndTerm ZeroOrMoreStarOrSlashAndTerm
                Internal(internal) => {
                    let first = internal.as_first().unwrap();
                    let star_or_slash_and_term_v = first.lhs.as_first().unwrap();
                    zero_or_more = &first.rhs;

                    let star_or_slash_v = &star_or_slash_and_term_v.lhs;
                    let rhs = self.eval_factor(&star_or_slash_and_term_v.rhs)?;

                    lhs = match star_or_slash_v
                        .as_internal()
                        .expect("star or slash")
                        .value
                        .0
                    {
                        Star => lhs * rhs,
                        Slash => lhs / rhs,
                        _ => unreachable!(),
                    };
                }
                // ()
                Leaf(_) => return Ok(lhs),
            }
        }
    }

    pub fn eval_factor(&self, ast: &FunctionAST) -> Result<f64, ()> {
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is Factor

        match &*internal.equal {
            Choice::First(first) => match first.lhs.as_internal().unwrap().value.0 {
                Plus => Ok(self.eval_factor(&first.rhs)?),
                Minus => Ok(-self.eval_factor(&first.rhs)?),
                _ => unreachable!(),
            },
            Choice::Second(second) => self.eval_power(&second.0),
        }
    }

    pub fn eval_power(&self, ast: &FunctionAST) -> Result<f64, ()> {
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is Power

        match &*internal.equal {
            Choice::First(first) => {
                let base = self.eval_atom(&first.lhs)?;

                let power_and_factor_v = first.rhs.as_first().unwrap();
                let exponent = self.eval_factor(&power_and_factor_v.rhs)?;

                Ok(base.powf(exponent))
            }
            Choice::Second(second) => self.eval_atom(&second.0),
        }
    }

    pub fn eval_atom(&self, ast: &FunctionAST) -> Result<f64, ()> {
        match &ast.node {
            // FloatLiteral Or IntegerLiteral
            Leaf(leaf) => leaf.as_original().map(|n| *n).ok_or(()),
            Internal(internal) => match internal.value.0 {
                // ExpressionInParentheses
                ExpressionInParentheses => {
                    let expression_and_close = ast.as_first().unwrap().rhs.as_first().unwrap();
                    self.eval_plus_or_minus_expr(&expression_and_close.lhs)
                }
                // Functions
                Sine => Ok(self
                    .eval_plus_or_minus_expr(&ast.as_first().unwrap().rhs)?
                    .sin()),
                Cosine => Ok(self
                    .eval_plus_or_minus_expr(&ast.as_first().unwrap().rhs)?
                    .cos()),
                Tangent => Ok(self
                    .eval_plus_or_minus_expr(&ast.as_first().unwrap().rhs)?
                    .tan()),
                Ln => Ok(self
                    .eval_plus_or_minus_expr(&ast.as_first().unwrap().rhs)?
                    .ln()),
                Lg => Ok(self
                    .eval_plus_or_minus_expr(&ast.as_first().unwrap().rhs)?
                    .log2()),
                // Variables
                UppercaseX => Ok(self.uppercase_x),
                UppercaseY => Ok(self.uppercase_y),
                UppercaseZ => Ok(self.uppercase_z),
                LowercaseX => Ok(self.lowercase_x),
                LowercaseY => Ok(self.lowercase_y),
                LowercaseZ => Ok(self.lowercase_z),
                UppercaseT => Ok(self.uppercase_t),
                LowercaseT => Ok(self.lowercase_t),
                UppercaseF => Ok(self.uppercase_f),
                _ => unreachable!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{function::parse, FunctionVariable};

    #[test]
    fn eval_or_or_expr() {
        let interpreter =
            FunctionInterpreter::new((-1.0, 1.0, 0.0), (2.0, 3.0, 4.0), 12.0, 3.0, 44100.0);

        let input: &[u8] = "0.0<0.1".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1==0||0!=0".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(false));

        let input: &[u8] = "3.14<PI&&PI<3.15".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));

        let input: &[u8] = "2<E&&E<3&&3<PI&&PI<4".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));

        let input: &[u8] = "1==0||2<E&&E<3&&3<PI&&PI<4".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0||1==1&&1==0".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0||1==1&&1==0||1==1".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0&&1==1||1==1&&0==1".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0&&1!=1||1==1".as_bytes();
        let ast = parse(&input, &FunctionVariable::OrOrExpression).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn eval_and_and_expr() {
        let interpreter =
            FunctionInterpreter::new((-1.0, 1.0, 0.0), (2.0, 3.0, 4.0), 12.0, 3.0, 44100.0);

        let input: &[u8] = "0.0<0.1".as_bytes();
        let ast = parse(&input, &FunctionVariable::AndAndExpression).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0.0!=0.1&&1.0==tan(PI/4)".as_bytes();
        let ast = parse(&input, &FunctionVariable::AndAndExpression).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0.0<0.1&&X==X&&F==F&&t!=T".as_bytes();
        let ast = parse(&input, &FunctionVariable::AndAndExpression).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0.0<0.1&&X==X&&F==F&&t==T".as_bytes();
        let ast = parse(&input, &FunctionVariable::AndAndExpression).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn eval_comparison_expr() {
        let interpreter =
            FunctionInterpreter::new((-1.0, 1.0, 0.0), (2.0, 3.0, 4.0), 12.0, 3.0, 44100.0);

        let input: &[u8] = "-1.0==-1".as_bytes();
        let ast = parse(&input, &FunctionVariable::ComparisonExpression).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));

        let input: &[u8] = "1.0==tan(PI/4)".as_bytes();
        let ast = parse(&input, &FunctionVariable::ComparisonExpression).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1.0!=-1.0".as_bytes();
        let ast = parse(&input, &FunctionVariable::ComparisonExpression).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1.0>=1.0".as_bytes();
        let ast = parse(&input, &FunctionVariable::ComparisonExpression).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1.0<=(1.0*5-4)".as_bytes();
        let ast = parse(&input, &FunctionVariable::ComparisonExpression).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "-3<-1.0".as_bytes();
        let ast = parse(&input, &FunctionVariable::ComparisonExpression).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "-3>-1.0".as_bytes();
        let ast = parse(&input, &FunctionVariable::ComparisonExpression).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn eval_plus_or_minus_expr() {
        let interpreter =
            FunctionInterpreter::new((-1.0, 1.0, 0.0), (2.0, 3.0, 4.0), 12.0, 3.0, 44100.0);

        // PlusOrMinusFactor
        let input: &[u8] = "-3".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(-3.0));
        let input: &[u8] = "++3".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(3.0));
        let input: &[u8] = "---3".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(-3.0));
        let input: &[u8] = "2-----1".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(1.0));

        // Functions
        let input: &[u8] = "sin(PI/2)".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(1.0));
        let input: &[u8] = "cos(PI/4)".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        let abs_difference = (result.unwrap() - 1.0 / 2.0_f64.sqrt()).abs();
        assert!(abs_difference < 1.0e-10);
        let input: &[u8] = "tan(PI/4)".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        let abs_difference = (result.unwrap() - 1.0).abs();
        assert!(abs_difference < 1.0e-10);
        let input: &[u8] = "ln(E*E)".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        let abs_difference = (result.unwrap() - 2.0).abs();
        assert!(abs_difference < 1.0e-10);
        let input: &[u8] = "lg8".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        let abs_difference = (result.unwrap() - 3.0).abs();
        assert!(abs_difference < 1.0e-10);

        // Variables
        let input: &[u8] = "X".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(-1.0));
        let input: &[u8] = "Y".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(1.0));
        let input: &[u8] = "Z".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(0.0));
        let input: &[u8] = "x".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(-3.0));
        let input: &[u8] = "y-z".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(2.0));
        let input: &[u8] = "44100+T/t".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(44104.0));
        let input: &[u8] = "E".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(2.71828182845904523536028747135266250));
        let input: &[u8] = "-PI".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(-3.14159265358979323846264338327950288));

        // Paren
        let input: &[u8] = "1+2*((5)-4/(2))-(3*(9/(8-5)))".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(-2.0));
        let input: &[u8] = "cos(2*PI)".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(1.0));

        let input: &[u8] = "1+2*3".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(7.0));

        // Power
        let input: &[u8] = "-2^2".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(-4.0));
        let input: &[u8] = "2^-2".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(0.25));
        let input: &[u8] = "(2+1)^(5-3)".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(9.0));
        let input: &[u8] = "(lg2)^2".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(1.0));
        let input: &[u8] = "2^lnE^2".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(4.0));

        // Term
        let input: &[u8] = "4/2*2".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(4.0));
        let input: &[u8] = "32/2/2/2/2/2".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(1.0));
        // TODO
        // let input: &[u8] = "1+2*3.0+4+5*6-8/8+9".as_bytes();
        // let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        // let result = interpreter.eval_plus_or_minus_expr(&ast);
        // assert_eq!(result, Ok(49.0));
    }
}
