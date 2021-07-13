use mpl::symbols::Variable;

impl Variable for FunctionVariable {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FunctionVariable {
    // Expression
    Expression,
    Expression1,

    // Term
    Term,
    Term1,

    // Factor
    Factor,
    Factor1,
    Factor2,
    Factor3,
    Factor4,

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
    Variable10,

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
    StarOrSlash,
}
