use crate::bub::functions::BubFnsRules;
use crate::bub::functions::{BubFnsVariable, BubFnsVariable::*};

use mpl::rules::{RightRule, Rules};
use mpl::symbols::U8SliceTerminal;

type Rule<'a> = RightRule<U8SliceTerminal<'a>, BubFnsVariable>;

impl<'a> Rules<U8SliceTerminal<'a>, BubFnsVariable> for BubFnsRules {
    fn get(&self, variable: &BubFnsVariable) -> Option<&Rule<'a>> {
        Some(match variable {
            // BubFns
            BubFns => &Self::BubFns_RULE,
            ZeroOrMoreBubFns => &Self::ZeroOrMoreBubFns_RULE,

            SpaceAndBubFn => &Self::SpaceAndBubFn_RULE,

            // BubFn
            BubFn => &Self::BubFn_RULE,
            BubFn1 => &Self::BubFn1_RULE,
            BubFn2 => &Self::BubFn2_RULE,
            BubFn3 => &Self::BubFn3_RULE,
            BubFn4 => &Self::BubFn4_RULE,

            SumAndSpace => &Self::SumAndSpace_RULE,
            OrOrExprAndSpace => &Self::OrOrExprAndSpace_RULE,

            // OrOr Expr
            OrOrExpr => &Self::OrOrExpr_RULE,
            OrOrExpr1 => &Self::OrOrExpr1_RULE,

            OrOr => &Self::OrOr_RULE,

            // AndAnd Expr
            AndAndExpr => &Self::AndAndExpr_RULE,
            AndAndExpr1 => &Self::AndAndExpr1_RULE,

            AndAnd => &Self::AndAnd_RULE,

            // Comparison Expr
            ComparisonExpr => &Self::ComparisonExpr_RULE,
            ComparisonExpr1 => &Self::ComparisonExpr1_RULE,

            Comparison => &Self::Comparison_RULE,
            Comparison1 => &Self::Comparison1_RULE,
            Comparison2 => &Self::Comparison2_RULE,
            Comparison3 => &Self::Comparison3_RULE,
            Comparison4 => &Self::Comparison4_RULE,
            Comparison5 => &Self::Comparison5_RULE,

            EqEq => &Self::EqEq_RULE,
            Ne => &Self::Ne_RULE,
            Ge => &Self::Ge_RULE,
            Le => &Self::Le_RULE,
            Gt => &Self::Gt_RULE,
            Lt => &Self::Lt_RULE,

            // Sum
            Sum => &Self::Sum_RULE,
            ZeroOrMorePlusOrMinusAndTerms => &Self::ZeroOrMorePlusOrMinusAndTerms_RULE,
            PlusOrMinusAndTerm => &Self::PlusOrMinusAndTerm_RULE,

            // Term
            Term => &Self::Term_RULE,
            ZeroOrMoreStarOrSlashAndFactors => &Self::ZeroOrMoreStarOrSlashAndFactors_RULE,
            StarOrSlashAndFactor => &Self::StarOrSlashAndFactor_RULE,

            // Factor
            Factor => &Self::Factor_RULE,

            // Power
            Power => &Self::Power_RULE,
            PowerAndFactor => &Self::PowerAndFactor_RULE,

            // Atom
            Atom => &Self::Atom_RULE,
            Atom1 => &Self::Atom1_RULE,
            Atom2 => &Self::Atom2_RULE,
            Atom3 => &Self::Atom3_RULE,
            Atom4 => &Self::Atom4_RULE,
            Atom5 => &Self::Atom5_RULE,

            // Variable
            Variable => &Self::Variable_RULE,
            Variable1 => &Self::Variable1_RULE,
            Variable2 => &Self::Variable2_RULE,
            Variable3 => &Self::Variable3_RULE,
            Variable4 => &Self::Variable4_RULE,
            Variable5 => &Self::Variable5_RULE,
            Variable6 => &Self::Variable6_RULE,
            Variable7 => &Self::Variable7_RULE,
            Variable8 => &Self::Variable8_RULE,
            Variable9 => &Self::Variable9_RULE,

            UppercaseX => &Self::UppercaseX_RULE,
            UppercaseY => &Self::UppercaseY_RULE,
            UppercaseZ => &Self::UppercaseZ_RULE,
            LowercaseX => &Self::LowercaseX_RULE,
            LowercaseY => &Self::LowercaseY_RULE,
            LowercaseZ => &Self::LowercaseZ_RULE,
            UppercaseN => &Self::UppercaseN_RULE,
            LowercaseN => &Self::LowercaseN_RULE,
            UppercaseF => &Self::UppercaseF_RULE,
            UppercaseS => &Self::UppercaseS_RULE,

            // Constant
            Constant => &Self::Constant_RULE,
            Constant1 => &Self::Constant1_RULE,

            E => &Self::E_RULE,
            Pi => &Self::Pi_RULE,

            // Function
            Function => &Self::Function_RULE,
            Function1 => &Self::Function1_RULE,
            Function2 => &Self::Function2_RULE,
            Function3 => &Self::Function3_RULE,
            Function4 => &Self::Function4_RULE,

            Sine => &Self::Sine_RULE,
            Cosine => &Self::Cosine_RULE,
            Tangent => &Self::Tangent_RULE,
            Ln => &Self::Ln_RULE,
            Lg => &Self::Lg_RULE,

            // Delimiters
            ExprInParentheses => &Self::ExprInParentheses_RULE,
            ExprAndClose => &Self::ExprAndClose_RULE,

            // Integer
            IntegerLiteral => &Self::IntegerLiteral_RULE,

            // Float
            FloatLiteral => &Self::FloatLiteral_RULE,
            PointAndDecLiteral => &Self::PointAndDecLiteral_RULE,

            BytesF64Literal => &Self::BytesF64Literal_RULE,

            DecLiteral => &Self::DecLiteral_RULE,
            ZeroOrMoreDecDigits => &Self::ZeroOrMoreDecDigits_RULE,

            DecDigit => &Self::DecDigit_RULE,
            DecDigit1 => &Self::DecDigit1_RULE,
            DecDigit2 => &Self::DecDigit2_RULE,
            DecDigit3 => &Self::DecDigit3_RULE,
            DecDigit4 => &Self::DecDigit4_RULE,
            DecDigit5 => &Self::DecDigit5_RULE,
            DecDigit6 => &Self::DecDigit6_RULE,
            DecDigit7 => &Self::DecDigit7_RULE,
            DecDigit8 => &Self::DecDigit8_RULE,

            // Other
            PlusOrMinus => &Self::PlusOrMinus_RULE,
            PlusOrMinus1 => &Self::PlusOrMinus1_RULE,
            Plus => &Self::Plus_RULE,
            Minus => &Self::Minus_RULE,

            StarOrSlash => &Self::StarOrSlash_RULE,
            StarOrSlash1 => &Self::StarOrSlash1_RULE,
            Star => &Self::Star_RULE,
            Slash => &Self::Slash_RULE,

            Space => &Self::Space_RULE,
        })
    }
}
