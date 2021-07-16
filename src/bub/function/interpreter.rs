// use crate::bub::function::{FunctionAST, FunctionVariable::*};
// use mpl::choices::Choice;
// use mpl::trees::Node;

// pub struct FunctionInterpreter {
//     uppercase_x: f64,
//     uppercase_y: f64,
//     uppercase_z: f64,
//     lowercase_x: f64,
//     lowercase_y: f64,
//     lowercase_z: f64,
//     /// Number of frames starting from the file. Absolute Time
//     uppercase_t: f64,
//     /// Number of frames starting from the function. Relative Time
//     lowercase_t: f64,
//     uppercase_f: f64,
// }

// impl FunctionInterpreter {
//     pub fn new(
//         speaker_absolute_coordinates: (f64, f64, f64),
//         bubble_absolute_coordinates: (f64, f64, f64),
//         absolute_time: f64,
//         relative_time: f64,
//         samples_per_sec: f64,
//     ) -> Self {
//         Self {
//             uppercase_x: speaker_absolute_coordinates.0,
//             uppercase_y: speaker_absolute_coordinates.1,
//             uppercase_z: speaker_absolute_coordinates.2,
//             lowercase_x: speaker_absolute_coordinates.0 - bubble_absolute_coordinates.0,
//             lowercase_y: speaker_absolute_coordinates.1 - bubble_absolute_coordinates.1,
//             lowercase_z: speaker_absolute_coordinates.2 - bubble_absolute_coordinates.2,
//             uppercase_t: absolute_time,
//             lowercase_t: relative_time,
//             uppercase_f: samples_per_sec,
//         }
//     }

//     pub fn eval_plus_or_minus_expr(&self, ast: FunctionAST) -> Result<f64, ()> {
//         let internal = ast.into_internal().expect("internal node");

//         // TODO: Check whether variable is PlusOrMinusExpression

//         match *internal.equal {
//             Choice::First(first) => {
//                 let lhs = self.eval_term(first.lhs)?;

//                 let plus_or_minus_expr1 = first.rhs.into_first().unwrap();
//                 let plus_or_minus_v = plus_or_minus_expr1.value.0;
//                 let plus_or_minus_expr1 = plus_or_minus_expr1.into_first().unwrap();

//                 let rhs = self.eval_term(plus_or_minus_expr1.rhs)?;

//                 match plus_or_minus_expr1_v {

//                 }

//                 Ok(lhs)
//             }
//             Choice::Second(second) => {
//                 self.eval_term(second.0)
//             }
//         }
//     }

//     pub fn eval_term(&self, ast: FunctionAST) -> Result<f64, ()> {
//         todo!()
//     }
// }
