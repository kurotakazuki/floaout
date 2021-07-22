use crate::bub::function::{
    BubbleFunction, BubbleFunctions, FunctionVariable, FunctionVariable::*,
};
use mpl::choices::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{AST, CST};
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionOutput {
    BubbleFunctions(BubbleFunctions),
    BubbleFunction(Box<BubbleFunction>),
    F64(f64),
}

impl From<f64> for FunctionOutput {
    fn from(n: f64) -> Self {
        Self::F64(n)
    }
}

impl FunctionOutput {
    pub fn as_bubble_functions(&self) -> Option<&BubbleFunctions> {
        match self {
            Self::BubbleFunctions(bubble_functions) => Some(bubble_functions),
            _ => None,
        }
    }

    pub fn as_bubble_function(&self) -> Option<&BubbleFunction> {
        match self {
            Self::BubbleFunction(bubble_function) => Some(bubble_function),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<&f64> {
        match self {
            Self::F64(n) => Some(n),
            _ => None,
        }
    }

    pub fn into_bubble_functions(self) -> Option<BubbleFunctions> {
        match self {
            Self::BubbleFunctions(bubble_functions) => Some(bubble_functions),
            _ => None,
        }
    }

    pub fn into_bubble_function(self) -> Option<Box<BubbleFunction>> {
        match self {
            Self::BubbleFunction(bubble_function) => Some(bubble_function),
            _ => None,
        }
    }

    pub fn into_f64(self) -> Option<f64> {
        match self {
            Self::F64(n) => Some(n),
            _ => None,
        }
    }
}

impl<'input> Output<'input, [u8], FunctionVariable, StartAndLenSpan<u16, u16>> for FunctionOutput {
    fn output_ast(
        input: &'input [u8],
        mut cst: CST<FunctionVariable, StartAndLenSpan<u16, u16>, Self>,
    ) -> AST<FunctionVariable, StartAndLenSpan<u16, u16>, Self> {
        match cst.node.value {
            Comparison | Atom | PlusOrMinus | StarOrSlash | Function | Variable => {
                let mut equal = cst.node.equal;
                loop {
                    match equal {
                        Choice::First(first) => {
                            return first.lhs;
                        }
                        Choice::Second(second) => {
                            equal = *second.0.node.into_internal().unwrap().equal;
                        }
                    }
                }
            }
            FloatLiteral | IntegerLiteral => {
                let n = match cst.node.equal {
                    Choice::First(_) => {
                        let lo = cst.span.start as usize;
                        let hi = cst.span.hi(input) as usize;

                        std::str::from_utf8(&input[lo..hi])
                            .unwrap()
                            .parse::<f64>()
                            .unwrap()
                            .into()
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

                AST::from_leaf(TerminalSymbol::from_original(n.into()), cst.span)
            }
            Constant => match cst.node.equal {
                Choice::First(first) => first.lhs,
                Choice::Second(second) => second.0,
            },
            Constant1 => {
                let o = cst
                    .node
                    .equal
                    .into_first()
                    .unwrap()
                    .lhs
                    .into_original()
                    .unwrap();
                AST::from_leaf(TerminalSymbol::from_original(o), cst.span)
            }
            E => AST::from_leaf(
                TerminalSymbol::from_original(std::f64::consts::E.into()),
                cst.span,
            ),
            Pi => AST::from_leaf(
                TerminalSymbol::from_original(std::f64::consts::PI.into()),
                cst.span,
            ),
            // Into Second
            OrOr | AndAnd | EqEq | Ne | Ge | Le | Gt | Lt | UppercaseX | UppercaseY
            | UppercaseZ | LowercaseX | LowercaseY | LowercaseZ | UppercaseT | LowercaseT
            | UppercaseF | Plus | Minus | Star | Slash | Semicolon | Space => {
                if let Choice::First(first) = cst.node.equal {
                    cst.node.equal = first.lhs.into();
                    AST::from_cst(cst)
                } else {
                    AST::from_cst(cst)
                }
            }
            // TODO
            // ExpressionInParentheses => {
            //     let expression_and_close = cst.into_first().unwrap().rhs.into_first().unwrap();
            //     expression_and_close.lhs
            // }
            _ => AST::from_cst(cst),
        }
    }
}
