use crate::bub::function::variable::{FunctionVariable, FunctionVariable::*};

use mpl::choices::{First, Second};
use mpl::rules::{RightRule, Rules};
use mpl::symbols::{Metasymbol::*, TerminalSymbol, U8SliceTerminal, U8SliceTerminal::*, E};

pub struct FunctionRules;

type Rule<'a> = RightRule<U8SliceTerminal<'a>, FunctionVariable>;

impl<'a> FunctionRules {
    // Expression
    const EXPRESSION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Term),
            rhs: E::V(Expression1),
        },
        second: Second(E::V(Term)),
    };
    const EXPRESSION1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(PlusOrMinus),
            rhs: E::V(Expression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Term
    const TERM_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Factor),
            rhs: E::V(Term1),
        },
        second: Second(E::V(Factor)),
    };
    const TERM1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(StarOrSlash),
            rhs: E::V(Term),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Factor
    const FACTOR_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(FloatLiteral),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Factor1)),
    };
    const FACTOR1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(PlusOrMinus),
            rhs: E::V(Factor),
        },
        second: Second(E::V(Factor2)),
    };
    const FACTOR2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Variable),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Factor3)),
    };
    const FACTOR3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Function),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Factor4)),
    };
    const FACTOR4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(ExpressionInParentheses),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Variable
    const VARIABLE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('X'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable1)),
    };
    const VARIABLE1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('Y'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable2)),
    };
    const VARIABLE2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('Z'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable3)),
    };
    const VARIABLE3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('x'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable4)),
    };
    const VARIABLE4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('y'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable5)),
    };
    const VARIABLE5_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('z'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable6)),
    };
    const VARIABLE6_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('T'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable7)),
    };
    const VARIABLE7_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('t'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable8)),
    };
    const VARIABLE8_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('F'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable9)),
    };
    const VARIABLE9_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("PI"))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable10)),
    };
    const VARIABLE10_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('E'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Function
    const FUNCTION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Sine),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function1)),
    };
    const FUNCTION1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Cosine),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function2)),
    };
    const FUNCTION2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Tangent),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function3)),
    };
    const FUNCTION3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Ln),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function4)),
    };
    const FUNCTION4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Lg),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// `Sine = "sin" Expression / f`
    const SINE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("sin"))),
            rhs: E::V(Expression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Cosine = "cos" Expression / f
    const COSINE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("cos"))),
            rhs: E::V(Expression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Tangent = "tan" Expression / f
    const TANGENT_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("tan"))),
            rhs: E::V(Expression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Ln = "ln" Expression / f
    const LN_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("ln"))),
            rhs: E::V(Expression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Lg = "lg" Expression / f
    const LG_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("lg"))),
            rhs: E::V(Expression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Delimiters
    /// ExpressionInParentheses = '(' ExpressionAndClose / f
    const EXPRESSION_IN_PARENTHESES_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('('))),
            rhs: E::V(ExpressionAndClose),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ExpressionAndClose = Expression ')' / f
    const EXPRESSION_AND_CLOSE_RULE: Rule<'a> = RightRule {
        first: First {
            rhs: E::V(Expression),
            lhs: E::T(TerminalSymbol::Original(Char(')'))),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Integer
    /// IntegerLiteral = DecLiteral () / f
    const INTEGER_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            rhs: E::V(DecLiteral),
            lhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Float
    /// FloatLiteral = DecLiteral PointAndDecLiteral / BytesF64Literal
    const FLOAT_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(DecLiteral),
            rhs: E::V(PointAndDecLiteral),
        },
        second: Second(E::V(BytesF64Literal)),
    };
    /// PointAndDecLiteral = '.' DecLiteral / f
    const POINT_AND_DEC_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('.'))),
            rhs: E::V(DecLiteral),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// BytesF64Literal = 'f' ???????? / f
    const BYTES_F64_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('f'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(8))),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Dec
    /// DecLiteral = DecDigit ZeroOrDecLiteral / f
    const DEC_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(DecDigit),
            rhs: E::V(ZeroOrDecLiteral),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ZeroOrDecLiteral = DecDigit ZeroOrDecLiteral / ()
    const ZERO_OR_DEC_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(DecDigit),
            rhs: E::V(ZeroOrDecLiteral),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Empty))),
    };

    /// DecDigit = '0' () / DecDigit1
    const DEC_DIGIT_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('0'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit1)),
    };
    /// DecDigit1 = '1' () / DecDigit2
    const DEC_DIGIT1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('1'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit2)),
    };
    /// DecDigit2 = '2' () / DecDigit3
    const DEC_DIGIT2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('2'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit3)),
    };
    /// DecDigit3 = '3' () / DecDigit4
    const DEC_DIGIT3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('3'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit4)),
    };
    /// DecDigit4 = '4' () / DecDigit5
    const DEC_DIGIT4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('4'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit5)),
    };
    /// DecDigit5 = '5' () / DecDigit6
    const DEC_DIGIT5_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('5'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit6)),
    };
    /// DecDigit6 = '6' () / DecDigit7
    const DEC_DIGIT6_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('6'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit7)),
    };
    /// DecDigit7 = '7' () / DecDigit8
    const DEC_DIGIT7_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('7'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit8)),
    };
    /// DecDigit8 = '8' () / '9'
    const DEC_DIGIT8_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('8'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Original(Char('9')))),
    };

    // Others
    /// PlusOrMinus = '+' () / '-'
    const PLUS_OR_MINUS_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('+'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Original(Char('-')))),
    };
    /// StarOrSlash = '*' () / '/'
    const STAR_OR_SLASH_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('*'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Original(Char('-')))),
    };
}

impl<'a> Rules<U8SliceTerminal<'a>, FunctionVariable> for FunctionRules {
    fn get(&self, variable: &FunctionVariable) -> Option<&Rule<'a>> {
        Some(match variable {
            // Expression
            Expression => &Self::EXPRESSION_RULE,
            Expression1 => &Self::EXPRESSION1_RULE,

            // Term
            Term => &Self::TERM_RULE,
            Term1 => &Self::TERM1_RULE,

            // Factor
            Factor => &Self::FACTOR_RULE,
            Factor1 => &Self::FACTOR1_RULE,
            Factor2 => &Self::FACTOR2_RULE,
            Factor3 => &Self::FACTOR3_RULE,
            Factor4 => &Self::FACTOR4_RULE,

            // Variable
            Variable => &Self::VARIABLE_RULE,
            Variable1 => &Self::VARIABLE1_RULE,
            Variable2 => &Self::VARIABLE2_RULE,
            Variable3 => &Self::VARIABLE3_RULE,
            Variable4 => &Self::VARIABLE4_RULE,
            Variable5 => &Self::VARIABLE5_RULE,
            Variable6 => &Self::VARIABLE6_RULE,
            Variable7 => &Self::VARIABLE7_RULE,
            Variable8 => &Self::VARIABLE8_RULE,
            Variable9 => &Self::VARIABLE9_RULE,
            Variable10 => &Self::VARIABLE10_RULE,

            // Function
            Function => &Self::FUNCTION_RULE,
            Function1 => &Self::FUNCTION1_RULE,
            Function2 => &Self::FUNCTION2_RULE,
            Function3 => &Self::FUNCTION3_RULE,
            Function4 => &Self::FUNCTION4_RULE,

            Sine => &Self::SINE_RULE,
            Cosine => &Self::COSINE_RULE,
            Tangent => &Self::TANGENT_RULE,
            Ln => &Self::LN_RULE,
            Lg => &Self::LG_RULE,

            // Delimiters
            ExpressionInParentheses => &Self::EXPRESSION_IN_PARENTHESES_RULE,
            ExpressionAndClose => &Self::EXPRESSION_AND_CLOSE_RULE,

            // Integer
            IntegerLiteral => &Self::INTEGER_LITERAL_RULE,

            // Float
            FloatLiteral => &Self::FLOAT_LITERAL_RULE,
            PointAndDecLiteral => &Self::POINT_AND_DEC_LITERAL_RULE,

            BytesF64Literal => &Self::BYTES_F64_LITERAL_RULE,

            DecLiteral => &Self::DEC_LITERAL_RULE,
            ZeroOrDecLiteral => &Self::ZERO_OR_DEC_LITERAL_RULE,

            DecDigit => &Self::DEC_DIGIT_RULE,
            DecDigit1 => &Self::DEC_DIGIT1_RULE,
            DecDigit2 => &Self::DEC_DIGIT2_RULE,
            DecDigit3 => &Self::DEC_DIGIT3_RULE,
            DecDigit4 => &Self::DEC_DIGIT4_RULE,
            DecDigit5 => &Self::DEC_DIGIT5_RULE,
            DecDigit6 => &Self::DEC_DIGIT6_RULE,
            DecDigit7 => &Self::DEC_DIGIT7_RULE,
            DecDigit8 => &Self::DEC_DIGIT8_RULE,

            // Other
            PlusOrMinus => &Self::PLUS_OR_MINUS_RULE,
            StarOrSlash => &Self::STAR_OR_SLASH_RULE,
        })
    }
}
