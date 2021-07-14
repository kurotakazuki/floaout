# Floaout
Floaout is the next-generation audio format.


# TODO
- Do not allow input like "2>1&&5".

# Bubble File Format Specification

## Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Bubble | `str` (3) | "bub" means Bubble |
| Version | `u8` (1) | Version of Bubble |
| Bubble ID | `u128` (16) | Bubble ID of this file. If undefined Bubble, than value is 0. |
| Frames | `u64` (8) | Number of frames |
| Samples Per Sec | `f64` (8) | Samples per sec |
| SampleKind | `u8` (1) | `SampleKind` |
| Name Size | `u8` (1) | Name Size |
| Name | `String` | Name (UTF-8) |
| CRC | `` () | Pending |

### SampleKind
| Variant  | Description | Value (`Type`) |
| ------------- | ------------- | ------------- |
| F32LE | `f32` Little Endian | 0 (`u8`) |
| F64LE | `f64` Little Endian | 1 (`u8`) |

## Each Sample
| Bubble Sample |  | `BubbleSample` |


## Bubble Sample
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Bubble's X coordinate | `f64` (8) | Bubble's X coordinate |
| Bubble's Y coordinate | `f64` (8) | Bubble's Y coordinate |
| Bubble's Z coordinate | `f64` (8) | Bubble's Z coordinate |
| Range | `` () |  |
| Comma | `char` (1) | ',' |
| Volume | `` () |  |
| Comma or Semicolon | `char` (1) | ',' if there is another |
| Connected, Ended, and FunctionKind | `u8` (1) | Connected, Ended, and `FunctionKind` |
| Ending (Relative) Time | `u64` (8) | Number of frames at the end of function. |
| Next Starting (Relative) Time | `u64` (8) | Number of frames at the start of the next function. Optional (!connected && !ended) |
| Sample Data |  | Sample Data |

### Connected and Ended
| ended \ connected | 0??????? | 1??????? |
| ------------- | ------------- | ------------- |
| ?0?????? | Stopped (NST) | Normal |
| ?1?????? | Ended | Ended |

### FunctionKind
| Variant  | Description | Value |
| ------------- | ------------- | ------------- |
| IEEEFloatingPoint | IEEEFloatingPoint | 0 |
| Expression | Expression | 1 |

### Sample Data
#### IEEEFloatingPoint
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| WavSample | `f32` or `f64` (4 or 8) | depends on `SampleKind` |

#### Expression
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Expression |  | Expression |


### Keywords

#### Variables
| Keyword | Description |
| ------------- | ------------- |
| X | X (Absolute) coordinate |
| Y | Y (Absolute) coordinate |
| Z | Z (Absolute) coordinate |
| x | x = X - X_0 (X_0 is Bubble's X coordinate). (Relative) coordinate. |
| y | y = Y - Y_0 (Y_0 is Bubble's Y coordinate). (Relative) coordinate. |
| z | z = Z - Z_0 (Z_0 is Bubble's Z coordinate). (Relative) coordinate. |
| T | Number of frames starting from the file. (Absolute) Time. (`as f64`) |
| t | Number of frames starting from the function. (Relative) Time. (`as f64`) |
| F | Samples per sec |

##### Constants
| Keyword | Description |
| ------------- | ------------- |
| PI | Pi |
| E | Euler's number |

#### Functions
| Keyword | Description |
| ------------- | ------------- |
| sin | Sine |
| cos | Cosine |
| tan | Tangent |
| ln | The natural logarithm of the number. |
| lg | The base 2 logarithm of the number. |

#### Others
| Keyword | Description |
| ------------- | ------------- |
| f | `f????????` `f64` |

### Punctuation
| Symbol | Name |
| ------------- | ------------- |
| + | Plus |
| - | Minus |
| * | Star |
| / | Slash |
| && | AndAnd |
| || | OrOr |
| == | EqEq |
| != | Ne |
| > | Gt |
| < | Lt |
| >= | Ge |
| <= | Le |

### Delimiters
| Symbol | Name |
| ------------- | ------------- |
| , | Comma |
| ; | Semicolon |
| ( ) | Parentheses |

### Syntax
```rust
// Expression
Expression = OrOrExpression () / f

// OrOr Expression
OrOrExpression = AndAndExpression OrOrExpression1 / AndAndExpression
OrOrExpression1 = OrOr OrOrExpression / f

OrOr = "||" () / f

// AndAnd Expression
AndAndExpression = ComparisonExpression AndAndExpression1 / ComparisonExpression
AndAndExpression1 = AndAnd AndAndExpression / f

AndAnd = "&&" () / f

// Comparsion Expression
ComparisonExpression = PlusOrMinusExpression ComparisonExpression1 / PlusOrMinusExpression
ComparisonExpression1 = Comparison ComparisonExpression / f

Comparison = EqEq () / Comparison1
Comparison1 = Ne () / Comparison2
Comparison2 = Ge () / Comparison3
Comparison3 = Le () / Comparison4
Comparison4 = Gt () / Comparison5
Comparison5 = Lt () / f

EqEq = "==" () / f
Ne = "!=" () / f
Ge = ">=" () / f
Le = "<=" () / f
Gt = '>' () / f
Lt = '<' () / f

// PlusOrMinusExpression
PlusOrMinusExpression = Term PlusOrMinusExpression1 / Term
PlusOrMinusExpression1 = PlusOrMinus PlusOrMinusExpression / f

// Term
Term = Factor Term1 / Factor
Term1 = StarOrSlash Term / f

// Factor
Factor = FloatLiteral () / Factor1
Factor1 = IntegerLiteral () / Factor2
Factor2 = PlusOrMinus Factor / Factor3
Factor3 = Function () / Factor4
Factor4 = Variable () / Factor5
Factor5 = ExpressionInParentheses () / f

// Variable
Variable = UppercaseX () / Variable1
Variable1 = UppercaseY () / Variable2
Variable2 = UppercaseZ () / Variable3
Variable3 = LowercaseX () / Variable4
Variable4 = LowercaseY () / Variable5
Variable5 = LowercaseZ () / Variable6
Variable6 = UppercaseT () / Variable7
Variable7 = LowercaseT () / Variable8
Variable8 = UppercaseF () / Variable9
Variable9 = Pi () / Variable10
Variable10 = E () / f

UppercaseX = 'X' () / f
UppercaseY = 'Y' () / f
UppercaseZ = 'Z' () / f
LowercaseX = 'x' () / f
LowercaseY = 'y' () / f
LowercaseZ = 'z' () / f
UppercaseT = 'T' () / f
LowercaseT = 't' () / f
UppercaseF = 'F' () / f
Pi = "PI" () / f
E = 'E' () / f

// Function
Function = Sine () / Function1
Function1 = Cosine () / Function2
Function2 = Tangent () / Function3
Function3 = Ln () / Function4
Function4 = Lg () / f

Sine = "sin" PlusOrMinusExpression / f
Cosine = "cos" PlusOrMinusExpression / f
Tangent = "tan" PlusOrMinusExpression / f
Ln = "ln" PlusOrMinusExpression / f
Lg = "lg" PlusOrMinusExpression / f

// Delimiters
ExpressionInParentheses = '(' ExpressionAndClose / f
ExpressionAndClose = PlusOrMinusExpression ')' / f

// Integer
IntegerLiteral = DecLiteral () / f

// Float
FloatLiteral = DecLiteral PointAndDecLiteral / BytesF64Literal
PointAndDecLiteral = '.' DecLiteral / f

BytesF64Literal = 'f' ???????? / f

// Dec
DecLiteral = DecDigit ZeroOrDecLiteral / f
ZeroOrDecLiteral = DecDigit ZeroOrDecLiteral / ()

DecDigit = '0' () / DecDigit1
DecDigit1 = '1' () / DecDigit2
DecDigit2 = '2' () / DecDigit3
DecDigit3 = '3' () / DecDigit4
DecDigit4 = '4' () / DecDigit5
DecDigit5 = '5' () / DecDigit6
DecDigit6 = '6' () / DecDigit7
DecDigit7 = '7' () / DecDigit8
DecDigit8 = '8' () / '9'

// Others
PlusOrMinus = Plus () / PlusOrMinus1
PlusOrMinus1 = Minus () / f
Plus = '+' () / f
Minus = '-' () / f

StarOrSlash = Star () / StarOrSlash1
StarOrSlash1 = Slash () / f
Star = '*' () / f
Slash = '/' () / f
```