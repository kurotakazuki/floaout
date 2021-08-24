use crate::bub::functions::{BubFnsAST, BubFnsVariable::*};
use crate::Coord;
use mpl::choices::Choice;
use mpl::trees::Node::*;

pub struct BubFnsInterpreter {
    // Speaker absolute coordinates
    pub uppercase: Coord,
    // Speaker relative coordinates
    pub lowercase: Coord,
    /// Number of frames starting from the file. Absolute Time
    pub uppercase_n: f64,
    /// Number of frames starting from the function. Relative Time
    pub lowercase_n: f64,
    pub uppercase_f: f64,
    pub uppercase_s: f64,
}

impl BubFnsInterpreter {
    pub fn new(
        speaker_absolute_coord: Coord,
        bub_absolute_coord: Coord,
        absolute_frame: f64,
        relative_frame: f64,
        frames: f64,
        samples_per_sec: f64,
    ) -> Self {
        Self {
            uppercase: speaker_absolute_coord,
            lowercase: speaker_absolute_coord - bub_absolute_coord,
            uppercase_n: absolute_frame,
            lowercase_n: relative_frame,
            uppercase_f: frames,
            uppercase_s: samples_per_sec,
        }
    }

    pub fn eval_or_or_expr(&self, ast: &BubFnsAST) -> Result<bool, ()> {
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is OrOrExpr

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

    pub fn eval_and_and_expr(&self, ast: &BubFnsAST) -> Result<bool, ()> {
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is AndAndExpr

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

    pub fn eval_comparison_expr(&self, ast: &BubFnsAST) -> Result<bool, ()> {
        // TODO: Check whether variable is ComparisonExpr

        let first = ast.as_first().unwrap();

        let lhs = self.eval_sum(&first.lhs)?;

        let comparison_expr1 = first.rhs.as_first().unwrap();
        let comparison_v = &comparison_expr1.lhs;
        let rhs = self.eval_sum(&comparison_expr1.rhs)?;

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

    pub fn eval_sum(&self, ast: &BubFnsAST) -> Result<f64, ()> {
        let sum_v = ast.as_first().unwrap();

        // TODO: Check whether variable is Sum

        let mut lhs = self.eval_term(&sum_v.lhs)?;

        let mut zero_or_more = &sum_v.rhs;

        // zero or more plus or minus and term
        loop {
            match &zero_or_more.node {
                // PlusOrMinusAndTerm ZeroOrMorePlusOrMinusAndTerms
                Internal(internal) => {
                    let first = internal.as_first().unwrap();
                    let plus_or_minus_and_term_v = first.lhs.as_first().unwrap();
                    zero_or_more = &first.rhs;

                    let plus_or_minus_v = &plus_or_minus_and_term_v.lhs;
                    let rhs = self.eval_term(&plus_or_minus_and_term_v.rhs)?;

                    lhs = match plus_or_minus_v
                        .as_internal()
                        .expect("plus or minus")
                        .value
                        .0
                    {
                        Plus => lhs + rhs,
                        Minus => lhs - rhs,
                        _ => unreachable!(),
                    };
                }
                // ()
                Leaf(_) => return Ok(lhs),
            }
        }
    }

    pub fn eval_term(&self, ast: &BubFnsAST) -> Result<f64, ()> {
        let term_v = ast.as_first().unwrap();

        // TODO: Check whether variable is Term

        let mut lhs = self.eval_factor(&term_v.lhs)?;

        let mut zero_or_more = &term_v.rhs;

        // zero or more star or slash and factor
        loop {
            match &zero_or_more.node {
                // StarOrSlashAndFactor ZeroOrMoreStarOrSlashAndFactors
                Internal(internal) => {
                    let first = internal.as_first().unwrap();
                    let star_or_slash_and_factor_v = first.lhs.as_first().unwrap();
                    zero_or_more = &first.rhs;

                    let star_or_slash_v = &star_or_slash_and_factor_v.lhs;
                    let rhs = self.eval_factor(&star_or_slash_and_factor_v.rhs)?;

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

    pub fn eval_factor(&self, ast: &BubFnsAST) -> Result<f64, ()> {
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

    pub fn eval_power(&self, ast: &BubFnsAST) -> Result<f64, ()> {
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

    pub fn eval_atom(&self, ast: &BubFnsAST) -> Result<f64, ()> {
        match &ast.node {
            // FloatLiteral Or IntegerLiteral
            Leaf(leaf) => leaf.as_original().unwrap().as_f64().copied().ok_or(()),
            Internal(internal) => match internal.value.0 {
                // ExprInParentheses
                ExprInParentheses => {
                    let expression_and_close = ast.as_first().unwrap().rhs.as_first().unwrap();
                    self.eval_sum(&expression_and_close.lhs)
                }
                // Functions
                Sine => Ok(self.eval_factor(&ast.as_first().unwrap().rhs)?.sin()),
                Cosine => Ok(self.eval_factor(&ast.as_first().unwrap().rhs)?.cos()),
                Tangent => Ok(self.eval_factor(&ast.as_first().unwrap().rhs)?.tan()),
                Ln => Ok(self.eval_factor(&ast.as_first().unwrap().rhs)?.ln()),
                Lg => Ok(self.eval_factor(&ast.as_first().unwrap().rhs)?.log2()),
                // Variables
                UppercaseX => Ok(self.uppercase.x),
                UppercaseY => Ok(self.uppercase.y),
                UppercaseZ => Ok(self.uppercase.z),
                LowercaseX => Ok(self.lowercase.x),
                LowercaseY => Ok(self.lowercase.y),
                LowercaseZ => Ok(self.lowercase.z),
                UppercaseN => Ok(self.uppercase_n),
                LowercaseN => Ok(self.lowercase_n),
                UppercaseF => Ok(self.uppercase_f),
                UppercaseS => Ok(self.uppercase_s),
                _ => unreachable!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{functions::parse, BubFnsVariable};

    #[test]
    fn bub_fns() {
        let input: &[u8] = "1 2 3 0!=1 sin(2*PI*440*n/S) 1 2 3 0!=1 sin(2*PI*440*n/S) 1 2 3 0!=1 sin(2*PI*440*n/S)".as_bytes();
        let result = parse(&input, &BubFnsVariable::BubFns).unwrap();
        let bub_fns = result.into_original().unwrap().into_bub_fns().unwrap();

        for bub_fn in bub_fns.0 {
            let speaker_absolute_coord = (-1.0, 1.0, 0.0).into();
            let bub_absolute_coord = (0.0, 0.0, 0.0).into();
            let absolute_frame = 12.0;
            let relative_frame = 0.0;
            let frames = 88200.0;
            let samples_per_sec = 44100.0;
            let mut interpreter = BubFnsInterpreter::new(
                speaker_absolute_coord,
                bub_absolute_coord,
                absolute_frame,
                relative_frame,
                frames,
                samples_per_sec,
            );

            interpreter.lowercase.x = interpreter.uppercase.x
                - interpreter.eval_sum(&bub_fn.bub_absolute_coord.0).unwrap();
            interpreter.lowercase.y = interpreter.uppercase.y
                - interpreter.eval_sum(&bub_fn.bub_absolute_coord.1).unwrap();
            interpreter.lowercase.z = interpreter.uppercase.z
                - interpreter.eval_sum(&bub_fn.bub_absolute_coord.2).unwrap();

            let domain = interpreter.eval_or_or_expr(&bub_fn.domain).unwrap();
            let volume = interpreter.eval_sum(&bub_fn.volume).unwrap();

            assert_eq!(interpreter.lowercase.x, -2.0);
            assert_eq!(interpreter.lowercase.y, -1.0);
            assert_eq!(interpreter.lowercase.z, -3.0);
            assert_eq!(domain, true);
            assert_eq!(volume, 0.0);
        }
    }

    #[test]
    fn eval_or_or_expr() {
        let interpreter = BubFnsInterpreter::new(
            (-1.0, 1.0, 0.0).into(),
            (2.0, 3.0, 4.0).into(),
            12.0,
            3.0,
            88200.0,
            44100.0,
        );

        let input: &[u8] = "0.0<0.1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "F==88200".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1==0||0!=0".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(false));

        let input: &[u8] = "3.14<PI&&PI<3.15".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));

        let input: &[u8] = "2<E&&E<3&&3<PI&&PI<4".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));

        let input: &[u8] = "1==0||2<E&&E<3&&3<PI&&PI<4".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0||1==1&&1==0".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0||1==1&&1==0||1==1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0&&1==1||1==1&&0==1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0==0&&1!=1||1==1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::OrOrExpr).unwrap();
        let result = interpreter.eval_or_or_expr(&ast);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn eval_and_and_expr() {
        let interpreter = BubFnsInterpreter::new(
            (-1.0, 1.0, 0.0).into(),
            (2.0, 3.0, 4.0).into(),
            12.0,
            3.0,
            88200.0,
            44100.0,
        );

        let input: &[u8] = "0.0<0.1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::AndAndExpr).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0.0!=0.1&&1.0==tan(PI/4)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::AndAndExpr).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0.0<0.1&&X==X&&S==S&&n!=N".as_bytes();
        let ast = parse(&input, &BubFnsVariable::AndAndExpr).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "0.0<0.1&&X==X&&S==S&&n==N".as_bytes();
        let ast = parse(&input, &BubFnsVariable::AndAndExpr).unwrap();
        let result = interpreter.eval_and_and_expr(&ast);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn eval_comparison_expr() {
        let interpreter = BubFnsInterpreter::new(
            (-1.0, 1.0, 0.0).into(),
            (2.0, 3.0, 4.0).into(),
            12.0,
            3.0,
            88200.0,
            44100.0,
        );

        let input: &[u8] = "-1.0==-1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::ComparisonExpr).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));

        let input: &[u8] = "1.0==tan(PI/4)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::ComparisonExpr).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1.0!=-1.0".as_bytes();
        let ast = parse(&input, &BubFnsVariable::ComparisonExpr).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1.0>=1.0".as_bytes();
        let ast = parse(&input, &BubFnsVariable::ComparisonExpr).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "1.0<=(1.0*5-4)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::ComparisonExpr).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "-3<-1.0".as_bytes();
        let ast = parse(&input, &BubFnsVariable::ComparisonExpr).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(true));
        let input: &[u8] = "-3>-1.0".as_bytes();
        let ast = parse(&input, &BubFnsVariable::ComparisonExpr).unwrap();
        let result = interpreter.eval_comparison_expr(&ast);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn eval_sum() {
        let interpreter = BubFnsInterpreter::new(
            (-1.0, 1.0, 0.0).into(),
            (2.0, 3.0, 4.0).into(),
            12.0,
            3.0,
            88200.0,
            44100.0,
        );

        // PlusOrMinusFactor
        let input: &[u8] = "-3".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-3.0));
        let input: &[u8] = "++3".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(3.0));
        let input: &[u8] = "---3".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-3.0));
        let input: &[u8] = "2-----1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(1.0));

        // Functions
        let input: &[u8] = "sin(PI/2)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(1.0));
        let input: &[u8] = "cos(PI/4)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        let abs_difference = (result.unwrap() - 1.0 / 2.0_f64.sqrt()).abs();
        assert!(abs_difference < 1.0e-10);
        let input: &[u8] = "tan(PI/4)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        let abs_difference = (result.unwrap() - 1.0).abs();
        assert!(abs_difference < 1.0e-10);
        let input: &[u8] = "ln(E*E)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        let abs_difference = (result.unwrap() - 2.0).abs();
        assert!(abs_difference < 1.0e-10);
        let input: &[u8] = "lg8".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        let abs_difference = (result.unwrap() - 3.0).abs();
        assert!(abs_difference < 1.0e-10);

        // Variables
        let input: &[u8] = "X".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-1.0));
        let input: &[u8] = "Y".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(1.0));
        let input: &[u8] = "Z".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(0.0));
        let input: &[u8] = "x".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-3.0));
        let input: &[u8] = "y-z".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(2.0));
        let input: &[u8] = "44100+N/n".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(44104.0));
        let input: &[u8] = "E".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(2.71828182845904523536028747135266250));
        let input: &[u8] = "-PI".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-3.14159265358979323846264338327950288));

        // Paren
        let input: &[u8] = "1+2*((5)-4/(2))-(3*(9/(8-5)))".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-2.0));
        let input: &[u8] = "cos(2*PI)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(1.0));

        let input: &[u8] = "1+2*3".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(7.0));

        // Power
        let input: &[u8] = "-2^2".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-4.0));
        let input: &[u8] = "2^-2".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(0.25));
        let input: &[u8] = "(2+1)^(5-3)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(9.0));
        let input: &[u8] = "(lg2)^2".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(1.0));
        let input: &[u8] = "2^lnE^2".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(4.0));

        // Term
        let input: &[u8] = "4/2*2".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(4.0));
        let input: &[u8] = "32/2/2/2/2/2".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(1.0));

        // Sum
        let input: &[u8] = "1-8/8-9".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(-9.0));
        let input: &[u8] = "sin(1/2*PI)".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(1.0));
        let input: &[u8] = "1+2*3.0+4+5*6-8/8+9".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(49.0));

        // Sum
        let input: &[u8] = "lnE^2^3".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(7.999999999999999));

        let input: &[u8] = "lg2+1".as_bytes();
        let ast = parse(&input, &BubFnsVariable::Sum).unwrap();
        let result = interpreter.eval_sum(&ast);
        assert_eq!(result, Ok(2.0));
    }
}
