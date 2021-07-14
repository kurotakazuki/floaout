use crate::bub::function::variable::FunctionVariable;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{AST, CST};

impl<'input> Output<'input, [u8], FunctionVariable, StartAndLenSpan<u16, u16>> for f64 {
    fn output_ast(
        input: &'input [u8],
        cst: CST<FunctionVariable, StartAndLenSpan<u16, u16>, Self>,
    ) -> AST<FunctionVariable, StartAndLenSpan<u16, u16>, Self> {
        match cst.node.value {
            FunctionVariable::FloatLiteral | FunctionVariable::IntegerLiteral => {
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
