use crate::bub::functions::{BubFn, BubFns, BubFnsVariable};
use mpl::choices::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{Node, AST, CST};
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq)]
pub enum BubFnsOutput {
    BubFns(BubFns),
    BubFn(Box<BubFn>),
    F64(f64),
}

impl From<BubFns> for BubFnsOutput {
    fn from(value: BubFns) -> Self {
        Self::BubFns(value)
    }
}

impl From<BubFn> for BubFnsOutput {
    fn from(value: BubFn) -> Self {
        Self::BubFn(Box::new(value))
    }
}

impl From<f64> for BubFnsOutput {
    fn from(n: f64) -> Self {
        Self::F64(n)
    }
}

impl BubFnsOutput {
    pub fn as_bub_fns(&self) -> Option<&BubFns> {
        match self {
            Self::BubFns(bub_fns) => Some(bub_fns),
            _ => None,
        }
    }

    pub fn as_bub_fn(&self) -> Option<&BubFn> {
        match self {
            Self::BubFn(bub_fn) => Some(bub_fn),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<&f64> {
        match self {
            Self::F64(n) => Some(n),
            _ => None,
        }
    }

    pub fn into_bub_fns(self) -> Option<BubFns> {
        match self {
            Self::BubFns(bub_fns) => Some(bub_fns),
            _ => None,
        }
    }

    pub fn into_bub_fn(self) -> Option<Box<BubFn>> {
        match self {
            Self::BubFn(bub_fn) => Some(bub_fn),
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

impl<'input> Output<'input, [u8], BubFnsVariable, StartAndLenSpan<u16, u16>> for BubFnsOutput {
    fn output_ast(
        input: &'input [u8],
        mut cst: CST<BubFnsVariable, StartAndLenSpan<u16, u16>, Self>,
    ) -> AST<BubFnsVariable, StartAndLenSpan<u16, u16>, Self> {
        match cst.node.value {
            BubFnsVariable::BubFns => {
                let span = cst.span;
                let mut bub_fns = BubFns::new();
                let mut first = cst.node.equal.into_first().unwrap();
                loop {
                    bub_fns.push(*first.lhs.into_original().unwrap().into_bub_fn().unwrap());
                    match first.rhs.node {
                        // ZeroOrMoreBubFns
                        Node::Internal(internal) => {
                            first = internal.into_first().unwrap();
                        }
                        // ()
                        Node::Leaf(_) => {
                            return AST::from_leaf(TerminalSymbol::Original(bub_fns.into()), span);
                        }
                    }
                }
            }
            // Into First rhs Child Node
            BubFnsVariable::SpaceAndBubFn => {
                let span = cst.span;
                let mut rhs_child = cst.node.equal.into_first().unwrap().rhs;
                rhs_child.span = span;

                rhs_child
            }
            BubFnsVariable::BubFn => {
                let span = cst.span;
                let bub_fn_equal = cst.node.equal.into_first().unwrap();
                let x0 = bub_fn_equal.lhs;

                let bub_fn1_equal = bub_fn_equal.rhs.into_first().unwrap();
                let y0 = bub_fn1_equal.lhs;

                let bub_fn2_equal = bub_fn1_equal.rhs.into_first().unwrap();
                let z0 = bub_fn2_equal.lhs;

                let bub_fn3_equal = bub_fn2_equal.rhs.into_first().unwrap();
                let domain = bub_fn3_equal.lhs;

                let bub_fn4_equal = bub_fn3_equal.rhs.into_second().unwrap();
                let volume = bub_fn4_equal.0;

                let bub_fn = BubFn {
                    bub_absolute_coord: (x0, y0, z0),
                    domain,
                    volume,
                };

                AST::from_leaf(TerminalSymbol::Original(bub_fn.into()), span)
            }
            // Into First lhs Child Node
            BubFnsVariable::SumAndSpace | BubFnsVariable::OrOrExprAndSpace => {
                let span = cst.span;
                let mut lhs_child = cst.node.equal.into_first().unwrap().lhs;
                lhs_child.span = span;

                lhs_child
            }
            BubFnsVariable::Comparison
            | BubFnsVariable::Atom
            | BubFnsVariable::PlusOrMinus
            | BubFnsVariable::StarOrSlash
            | BubFnsVariable::Function
            | BubFnsVariable::Variable => {
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
            BubFnsVariable::FloatLiteral | BubFnsVariable::IntegerLiteral => {
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
            BubFnsVariable::BytesF64Literal => {
                let lo = cst.span.start as usize + 1;
                let hi = cst.span.hi(input) as usize;

                let n: f64 = f64::from_le_bytes(
                    input[lo..hi]
                        .try_into()
                        .expect("slice with incorrect length"),
                );

                AST::from_leaf(TerminalSymbol::from_original(n.into()), cst.span)
            }
            BubFnsVariable::Constant => match cst.node.equal {
                Choice::First(first) => first.lhs,
                Choice::Second(second) => second.0,
            },
            BubFnsVariable::Constant1 => {
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
            BubFnsVariable::E => AST::from_leaf(
                TerminalSymbol::from_original(std::f64::consts::E.into()),
                cst.span,
            ),
            BubFnsVariable::Pi => AST::from_leaf(
                TerminalSymbol::from_original(std::f64::consts::PI.into()),
                cst.span,
            ),
            // First lhs into Second
            // A = B () / f
            // A = B
            BubFnsVariable::BubFn4
            | BubFnsVariable::OrOr
            | BubFnsVariable::AndAnd
            | BubFnsVariable::EqEq
            | BubFnsVariable::Ne
            | BubFnsVariable::Ge
            | BubFnsVariable::Le
            | BubFnsVariable::Gt
            | BubFnsVariable::Lt
            | BubFnsVariable::UppercaseX
            | BubFnsVariable::UppercaseY
            | BubFnsVariable::UppercaseZ
            | BubFnsVariable::LowercaseX
            | BubFnsVariable::LowercaseY
            | BubFnsVariable::LowercaseZ
            | BubFnsVariable::UppercaseN
            | BubFnsVariable::LowercaseN
            | BubFnsVariable::UppercaseF
            | BubFnsVariable::UppercaseS
            | BubFnsVariable::Plus
            | BubFnsVariable::Minus
            | BubFnsVariable::Star
            | BubFnsVariable::Slash
            | BubFnsVariable::Space => {
                if let Choice::First(first) = cst.node.equal {
                    cst.node.equal = first.lhs.into();
                }
                AST::from_cst(cst)
            }
            // TODO
            // ExprInParentheses => {
            //     let expression_and_close = cst.into_first().unwrap().rhs.into_first().unwrap();
            //     expression_and_close.lhs
            // }
            _ => AST::from_cst(cst),
        }
    }
}
