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
    pub bubble_absolute_coordinates: (FunctionAST, FunctionAST, FunctionAST),
    // The root node variable is `OrOrExpression`.
    pub domain: FunctionAST,
    // The root node variable is `Sum`.
    pub volume: FunctionAST,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BubbleFunctions(Vec<BubbleFunction>);

impl From<Vec<BubbleFunction>> for BubbleFunctions {
    fn from(v: Vec<BubbleFunction>) -> Self {
        Self(v)
    }
}

impl From<BubbleFunctions> for Vec<BubbleFunction> {
    fn from(v: BubbleFunctions) -> Self {
        v.0
    }
}

impl BubbleFunctions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, bubble_function: BubbleFunction) {
        self.0.push(bubble_function)
    }

    pub fn to_volume(
        &self,
        speaker_absolute_coordinates: (f64, f64, f64),
        absolute_frame: f64,
        relative_frame: f64,
        frames: f64,
        samples_per_sec: f64,
    ) -> Option<f64> {
        for bubble_function in self.0.iter() {
            let bubble_absolute_coordinates = (0.0, 0.0, 0.0);
            let mut interpreter = FunctionInterpreter::new(
                speaker_absolute_coordinates,
                bubble_absolute_coordinates,
                absolute_frame,
                relative_frame,
                frames,
                samples_per_sec,
            );

            interpreter.lowercase_x = interpreter.uppercase_x
                - interpreter
                    .eval_sum(&bubble_function.bubble_absolute_coordinates.0)
                    .unwrap();
            interpreter.lowercase_y = interpreter.uppercase_y
                - interpreter
                    .eval_sum(&bubble_function.bubble_absolute_coordinates.1)
                    .unwrap();
            interpreter.lowercase_z = interpreter.uppercase_z
                - interpreter
                    .eval_sum(&bubble_function.bubble_absolute_coordinates.2)
                    .unwrap();

            let domain = interpreter
                .eval_or_or_expr(&bubble_function.domain)
                .unwrap();

            if domain {
                let volume = interpreter.eval_sum(&bubble_function.volume).unwrap();
                return Some(volume);
            }
        }

        None
    }
}
