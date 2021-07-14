use crate::bub::function::variable::{FunctionVariable, FunctionVariable::*};
use mpl::choices::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{AST, CST};

impl<'input> Output<'input, [u8], FunctionVariable, StartAndLenSpan<u16, u16>> for f64 {
    fn output_ast(
        input: &'input [u8],
        mut cst: CST<FunctionVariable, StartAndLenSpan<u16, u16>, Self>,
    ) -> AST<FunctionVariable, StartAndLenSpan<u16, u16>, Self> {
        match cst.node.value {
            Comparison | Factor | Variable | PlusOrMinus | StarOrSlash | Function => {
                let mut equal = cst.node.equal;
                loop {
                    match equal {
                        Choice::First(_) => {
                            cst.node.equal = equal;
                            return AST::from_cst(cst);
                        }
                        Choice::Second(second) => {
                            equal = *second.0.node.into_internal().unwrap().equal;
                        }
                    }
                }
            }
            FloatLiteral | IntegerLiteral => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;

                let n = std::str::from_utf8(&input[lo..hi])
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();

                AST::from_leaf(TerminalSymbol::from_original(n), cst.span)
            }
            _ => AST::from_cst(cst),
        }
    }
}
