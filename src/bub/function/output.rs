use crate::bub::function::variable::{FunctionVariable, FunctionVariable::*};
use mpl::choices::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{AST, CST};
use std::convert::TryInto;

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
                        Choice::First(first) => {
                            cst.node.equal = first.lhs.into();
                            return AST::from_cst(cst);
                        }
                        Choice::Second(second) => {
                            equal = *second.0.node.into_internal().unwrap().equal;
                        }
                    }
                }
            }
            FloatLiteral | IntegerLiteral => {
                let n: f64 = match cst.node.equal {
                    Choice::First(_) => {
                        let lo = cst.span.start as usize;
                        let hi = cst.span.hi(input) as usize;

                        std::str::from_utf8(&input[lo..hi])
                            .unwrap()
                            .parse::<f64>()
                            .unwrap()
                    }
                    Choice::Second(second) => second.0.into_original().unwrap(),
                };

                AST::from_leaf(TerminalSymbol::from_original(n), cst.span)
            }
            BytesF64Literal => {
                let lo = cst.span.start as usize + 1;
                let hi = cst.span.hi(input) as usize;

                let n: f64 = f64::from_le_bytes(
                    input[lo..hi]
                        .try_into()
                        .expect("slice with incorrect length"),
                );

                AST::from_leaf(TerminalSymbol::from_original(n), cst.span)
            }
            OrOr | AndAnd | EqEq | Ne | Ge | Le | Gt | Lt | UppercaseX | UppercaseY
            | UppercaseZ | LowercaseX | LowercaseY | LowercaseZ | UppercaseT | LowercaseT
            | UppercaseF | Pi | E | Plus | Minus | Star | Slash => {
                if let Choice::First(first) = cst.node.equal {
                    cst.node.equal = first.lhs.into();
                    AST::from_cst(cst)
                } else {
                    AST::from_cst(cst)
                }
            }
            _ => AST::from_cst(cst),
        }
    }
}
