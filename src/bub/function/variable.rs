use mpl::symbols::Variable;

impl Variable for FunctionVariable {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FunctionVariable {
    // OrOr Expression
    OrOrExpression,
    OrOrExpression1,

    OrOr,

    // AndAnd Expression
    AndAndExpression,
    AndAndExpression1,

    AndAnd,

    // Comparsion Expression
    ComparisonExpression,
    ComparisonExpression1,

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

    // PlusOrMinusExpression
    PlusOrMinusExpression,
    PlusOrMinusExpression1,

    // Term
    Term,
    ZeroOrMoreStarOrSlashAndTerm,
    StarOrSlashAndTerm,

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

    UppercaseX,
    UppercaseY,
    UppercaseZ,
    LowercaseX,
    LowercaseY,
    LowercaseZ,
    UppercaseT,
    LowercaseT,
    UppercaseF,

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
    ExpressionInParentheses,
    ExpressionAndClose,

    // Integer
    IntegerLiteral,

    // Float
    FloatLiteral,
    PointAndDecLiteral,

    BytesF64Literal,

    DecLiteral,
    ZeroOrDecLiteral,

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
}
