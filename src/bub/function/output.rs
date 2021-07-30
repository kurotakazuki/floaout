use crate::bub::function::{BubbleFunction, BubbleFunctions, FunctionVariable};
use mpl::choices::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{Node, AST, CST};
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionOutput {
    BubbleFunctions(BubbleFunctions),
    BubbleFunction(Box<BubbleFunction>),
    F64(f64),
}

impl From<BubbleFunctions> for FunctionOutput {
    fn from(value: BubbleFunctions) -> Self {
        Self::BubbleFunctions(value)
    }
}

impl From<BubbleFunction> for FunctionOutput {
    fn from(value: BubbleFunction) -> Self {
        Self::BubbleFunction(Box::new(value))
    }
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
            FunctionVariable::BubbleFunctions => {
                let span = cst.span;
                let mut bubble_functions = BubbleFunctions::new();
                let mut first = cst.node.equal.into_first().unwrap();
                loop {
                    bubble_functions.push(
                        *first
                            .lhs
                            .into_original()
                            .unwrap()
                            .into_bubble_function()
                            .unwrap(),
                    );
                    match first.rhs.node {
                        // ZeroOrMoreBubbleFunctions
                        Node::Internal(internal) => {
                            first = internal.into_first().unwrap();
                        }
                        // ()
                        Node::Leaf(_) => {
                            return AST::from_leaf(
                                TerminalSymbol::Original(bubble_functions.into()),
                                span,
                            );
                        }
                    }
                }
            }
            // Into First rhs Child Node
            FunctionVariable::SpaceAndBubbleFunction => {
                let span = cst.span;
                let mut rhs_child = cst.node.equal.into_first().unwrap().rhs;
                rhs_child.span = span;

                rhs_child
            }
            FunctionVariable::BubbleFunction => {
                let span = cst.span;
                let bubble_function_equal = cst.node.equal.into_first().unwrap();
                let x0 = bubble_function_equal.lhs;

                let bubble_function1_equal = bubble_function_equal.rhs.into_first().unwrap();
                let y0 = bubble_function1_equal.lhs;

                let bubble_function2_equal = bubble_function1_equal.rhs.into_first().unwrap();
                let z0 = bubble_function2_equal.lhs;

                let bubble_function3_equal = bubble_function2_equal.rhs.into_first().unwrap();
                let domain = bubble_function3_equal.lhs;

                let bubble_function4_equal = bubble_function3_equal.rhs.into_second().unwrap();
                let volume = bubble_function4_equal.0;

                let bubble_function = BubbleFunction {
                    bubble_absolute_coordinates: (x0, y0, z0),
                    domain,
                    volume,
                };

                AST::from_leaf(TerminalSymbol::Original(bubble_function.into()), span)
            }
            // Into First lhs Child Node
            FunctionVariable::SumAndSpace | FunctionVariable::OrOrExpressionAndSpace => {
                let span = cst.span;
                let mut lhs_child = cst.node.equal.into_first().unwrap().lhs;
                lhs_child.span = span;

                lhs_child
            }
            FunctionVariable::Comparison
            | FunctionVariable::Atom
            | FunctionVariable::PlusOrMinus
            | FunctionVariable::StarOrSlash
            | FunctionVariable::Function
            | FunctionVariable::Variable => {
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
            FunctionVariable::FloatLiteral | FunctionVariable::IntegerLiteral => {
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
            FunctionVariable::BytesF64Literal => {
                let lo = cst.span.start as usize + 1;
                let hi = cst.span.hi(input) as usize;

                let n: f64 = f64::from_le_bytes(
                    input[lo..hi]
                        .try_into()
                        .expect("slice with incorrect length"),
                );

                AST::from_leaf(TerminalSymbol::from_original(n.into()), cst.span)
            }
            FunctionVariable::Constant => match cst.node.equal {
                Choice::First(first) => first.lhs,
                Choice::Second(second) => second.0,
            },
            FunctionVariable::Constant1 => {
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
            FunctionVariable::E => AST::from_leaf(
                TerminalSymbol::from_original(std::f64::consts::E.into()),
                cst.span,
            ),
            FunctionVariable::Pi => AST::from_leaf(
                TerminalSymbol::from_original(std::f64::consts::PI.into()),
                cst.span,
            ),
            // First lhs into Second
            // A = B () / f
            // A = B
            FunctionVariable::BubbleFunction4
            | FunctionVariable::OrOr
            | FunctionVariable::AndAnd
            | FunctionVariable::EqEq
            | FunctionVariable::Ne
            | FunctionVariable::Ge
            | FunctionVariable::Le
            | FunctionVariable::Gt
            | FunctionVariable::Lt
            | FunctionVariable::UppercaseX
            | FunctionVariable::UppercaseY
            | FunctionVariable::UppercaseZ
            | FunctionVariable::LowercaseX
            | FunctionVariable::LowercaseY
            | FunctionVariable::LowercaseZ
            | FunctionVariable::UppercaseN
            | FunctionVariable::LowercaseN
            | FunctionVariable::UppercaseF
            | FunctionVariable::UppercaseS
            | FunctionVariable::Plus
            | FunctionVariable::Minus
            | FunctionVariable::Star
            | FunctionVariable::Slash
            | FunctionVariable::Space => {
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
