use mpl::symbols::Variable;

impl Variable for BubFnsVariable {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BubFnsVariable {
    // BubFns
    BubFns,
    ZeroOrMoreBubFns,

    SpaceAndBubFn,

    // BubFn
    BubFn,
    BubFn1,
    BubFn2,
    BubFn3,
    BubFn4,

    SumAndSpace,
    OrOrExprAndSpace,

    // OrOr Expr
    OrOrExpr,
    OrOrExpr1,

    OrOr,

    // AndAnd Expr
    AndAndExpr,
    AndAndExpr1,

    AndAnd,

    // Comparison Expr
    ComparisonExpr,
    ComparisonExpr1,

    Comparison,
    Comparison1,
    Comparison2,
    Comparison3,
    Comparison4,
    Comparison5,

    EqEq,
    Ne,
    Ge,
    Le,
    Gt,
    Lt,

    // Sum
    Sum,
    ZeroOrMorePlusOrMinusAndTerms,
    PlusOrMinusAndTerm,

    // Term
    Term,
    ZeroOrMoreStarOrSlashAndFactors,
    StarOrSlashAndFactor,

    // Factor
    Factor,

    // Power
    Power,
    PowerAndFactor,

    // Atom
    Atom,
    Atom1,
    Atom2,
    Atom3,
    Atom4,
    Atom5,

    // Variable
    Variable,
    Variable1,
    Variable2,
    Variable3,
    Variable4,
    Variable5,
    Variable6,
    Variable7,
    Variable8,
    Variable9,

    UppercaseX,
    UppercaseY,
    UppercaseZ,
    LowercaseX,
    LowercaseY,
    LowercaseZ,
    UppercaseN,
    LowercaseN,
    UppercaseF,
    UppercaseS,

    // Constant
    Constant,
    Constant1,
    Pi,
    E,

    // Function
    Function,
    Function1,
    Function2,
    Function3,
    Function4,

    Sine,
    Cosine,
    Tangent,
    Ln,
    Lg,

    // Delimiters
    ExprInParentheses,
    ExprAndClose,

    // Integer
    IntegerLiteral,

    // Float
    FloatLiteral,
    PointAndDecLiteral,

    BytesF64Literal,

    DecLiteral,
    ZeroOrMoreDecDigits,

    DecDigit,
    DecDigit1,
    DecDigit2,
    DecDigit3,
    DecDigit4,
    DecDigit5,
    DecDigit6,
    DecDigit7,
    DecDigit8,

    // Other
    PlusOrMinus,
    PlusOrMinus1,
    Plus,
    Minus,

    StarOrSlash,
    StarOrSlash1,
    Star,
    Slash,

    Space,
}
