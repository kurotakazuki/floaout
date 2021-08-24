use crate::Coord;
use mpl::span::StartAndLenSpan;
use mpl::trees::AST;

pub use self::interpreter::BubFnsInterpreter;
pub use self::output::BubFnsOutput;
pub use self::parse::parse;
pub use self::rules::BubFnsRules;
pub use self::variable::BubFnsVariable;

mod interpreter;
mod output;
mod parse;
mod rules;
mod variable;

pub type BubFnsAST = AST<BubFnsVariable, StartAndLenSpan<u16, u16>, BubFnsOutput>;

#[derive(Clone, Debug, PartialEq)]
pub struct BubFn {
    // The root node variable is `Sum`.
    pub bub_absolute_coord: (BubFnsAST, BubFnsAST, BubFnsAST),
    // The root node variable is `OrOrExpr`.
    pub domain: BubFnsAST,
    // The root node variable is `Sum`.
    pub volume: BubFnsAST,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BubFns(Vec<BubFn>);

impl From<Vec<BubFn>> for BubFns {
    fn from(v: Vec<BubFn>) -> Self {
        Self(v)
    }
}

impl From<BubFns> for Vec<BubFn> {
    fn from(v: BubFns) -> Self {
        v.0
    }
}

impl BubFns {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, bub_function: BubFn) {
        self.0.push(bub_function)
    }

    pub fn to_volume(
        &self,
        speaker_absolute_coord: Coord,
        absolute_frame: f64,
        relative_frame: f64,
        frames: f64,
        samples_per_sec: f64,
    ) -> Option<Vec<(f64, BubFnsInterpreter)>> {
        let mut volume_and_interpreter_vec = Vec::new();
        for bub_function in self.0.iter() {
            let bub_absolute_coord = Coord::default();
            let mut interpreter = BubFnsInterpreter::new(
                speaker_absolute_coord,
                bub_absolute_coord,
                absolute_frame,
                relative_frame,
                frames,
                samples_per_sec,
            );

            interpreter.lowercase.x = interpreter.uppercase.x
                - interpreter
                    .eval_sum(&bub_function.bub_absolute_coord.0)
                    .unwrap();
            interpreter.lowercase.y = interpreter.uppercase.y
                - interpreter
                    .eval_sum(&bub_function.bub_absolute_coord.1)
                    .unwrap();
            interpreter.lowercase.z = interpreter.uppercase.z
                - interpreter
                    .eval_sum(&bub_function.bub_absolute_coord.2)
                    .unwrap();

            let domain = interpreter.eval_or_or_expr(&bub_function.domain).unwrap();

            if domain {
                let volume = interpreter.eval_sum(&bub_function.volume).unwrap();
                if volume != 0.0 {
                    volume_and_interpreter_vec.push((volume, interpreter));
                }
            }
        }

        if volume_and_interpreter_vec.is_empty() {
            None
        } else {
            Some(volume_and_interpreter_vec)
        }
    }
}
