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
        let internal = ast.as_internal().expect("internal node");

        // TODO: Check whether variable is PlusOrMinusExpression

        match &*internal.equal {
            Choice::First(first) => {
                let lhs = self.eval_factor(&first.lhs)?;

                let star_or_slash_expr1 = first.rhs.as_first().unwrap();
                let star_or_slash_v = &star_or_slash_expr1.lhs;
                let rhs = self.eval_term(&star_or_slash_expr1.rhs)?;

                match star_or_slash_v
                    .as_internal()
                    .expect("star or slash")
                    .value
                    .0
                {
                    Star => Ok(lhs * rhs),
                    Slash => Ok(lhs / rhs),
                    _ => unreachable!(),
                }
            }
            Choice::Second(second) => self.eval_factor(&second.0),
        }
    }

    pub fn eval_factor(&self, ast: &FunctionAST) -> Result<f64, ()> {
        match &ast.node {
            Leaf(leaf) => leaf.as_original().map(|n| *n).ok_or(()),
            Internal(internal) => match internal {
                _ => todo!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bub::{function::parse, FunctionVariable};

    #[test]
    fn eval_plus_or_minus_expr() {
        let interpreter =
            FunctionInterpreter::new((1.0, 1.0, 0.0), (2.0, 2.0, 2.0), 12.0, 3.0, 44100.0);

        let input: &[u8] = "1+2*3".as_bytes();
        let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        let result = interpreter.eval_plus_or_minus_expr(&ast);
        assert_eq!(result, Ok(7.0));

        // let input: &[u8] = "1+2*3.0+4+5*6-8/8+9".as_bytes();
        // let ast = parse(&input, &FunctionVariable::PlusOrMinusExpression).unwrap();
        // let result = interpreter.eval_plus_or_minus_expr(&ast);
        // assert_eq!(result, Ok(49.0));
    }
}
