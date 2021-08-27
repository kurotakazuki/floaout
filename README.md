# Floaout
Floaout is the next-generation audio format.

# TODO
- Add `read_bub_fns_block` in BubFrameReader
- error handling
- Clarify whether #[derive(Order)] is needed
- Add Position field in BubfnsInterpreter
- Check file is supported version or not
- Parallel computing
- Add Functions like pow, sinh, ...

# Bubble File Format Specification

## Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Spec Version | `u8` (1) | Version of Bubble File Format Specification. |
| Bubble ID | `u128` (16) | Bubble ID of this file. The value is 0 if the Bubble is undefined. |
| Bubble Version | `u16` (2) | Version of Bubble |
| Frames | `u64` (8) | Number of frames |
| First Head Absolute Frame | `u64` (8) | First Head Absolute Frame |
| Samples Per Sec | `f64` (8) | Samples per sec |
| LpcmKind | `u8` (1) | `LpcmKind` |
| BubSampleKind | `u8` (1) | `BubSampleKind` |
| Name Size | `u8` (1) | Name Size (0~255) |
| Name | `String` | Name (UTF-8) |
| CRC-32K/4.2 | `u32` (4) | Max length at Hamming Distance 4 is 2147483615 (bits). And max length at Hamming Distance 6 is 6167 (bits). |

### LpcmKind
| Variant  | Description | Value (`Type`) |
| ------------- | ------------- | ------------- |
| F32LE | `f32` Little Endian | 0 (`u8`) |
| F64LE | `f64` Little Endian | 1 (`u8`) |

### BubSampleKind
| Variant  | Description | Value |
| ------------- | ------------- | ------------- |
| Lpcm | Lpcm | 0 |
| Expr | Expr | 1 |

### CRC
```rust ignore
Algorithm::<u32> {
    endian: Endian::Little,
    poly: 0x93a409eb, // CRC-32K/4.2
    init: 0xffffffff,
    refin: true,
    refout: true,
    xorout: 0xffffffff,
    residue: 0x76e908ce,
}
```

## Each Sample
| Bubble Sample |  | `BubbleSample` |


## Bubble Sample
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Functions size | `u16` (1) | Functions size |
| Bubble's X coordinate | `Sum` | Bubble's X coordinate (X_0) |
| Space | `char` (1) | ' ' |
| Bubble's Y coordinate | `Sum` | Bubble's Y coordinate (Y_0) |
| Space | `char` (1) | ' ' |
| Bubble's Z coordinate | `Sum` | Bubble's Z coordinate (Z_0) |
| Space | `char` (1) | ' ' |
| Domain | `OrOrExpr` |  |
| Space | `char` (1) | ' ' |
| Volume | `Sum` |  |
| Space or Empty | `char` (1) | ' ' if there is another |
| Foot Relative Frame | `u64` (8) | Number of frames at the end of function. |
| Next Head Relative Frame | `Option<u64>` (8) | Number of frames at the start of the next function. `None` if 0. |
| Sample Data |  | Sample Data |
| CRC-32K/4.2 | `u32` (4) | After every foot frames. From the previous CRC. |

### Sample Data
#### Lpcm
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Sample | `f32` or `f64` (4 or 8) | depends on `LpcmKind` |

#### Expr
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Expr Size | `u16` (2) | Expr Size |
| Expr | `Sum` | Expr |


### Keywords

#### Variables
| Keyword | Description |
| ------------- | ------------- |
| X | Speaker's absolute X coordinate |
| Y | Speaker's absolute Y coordinate |
| Z | Speaker's absolute Z coordinate |
| x | x = X - X_0 (X_0 is Bubble's absolute X coordinate). Speaker's relative X coordinate. |
| y | y = Y - Y_0 (Y_0 is Bubble's absolute Y coordinate). Speaker's relative Y coordinate. |
| z | z = Z - Z_0 (Z_0 is Bubble's absolute Z coordinate). Speaker's relative Z coordinate. |
| N | Absolute frame n. Number of frames starting from the file. (`as f64`) |
| n | Relative frame n. Number of frames starting from the function.(`as f64`) |
| F | Frames (`as f64`) |
| S | Samples per sec |

##### Constants
| Keyword | Description |
| ------------- | ------------- |
| E | Euler's number |
| PI | Pi |

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
| b | `b????????` `f64` |

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
|   | Space |
| , | Comma |
| ( ) | Parentheses |

### Syntax
```rust ignore
// BubFns
BubFns = BubFn ZeroOrMoreBubFns / f
ZeroOrMoreBubFns = SpaceAndBubFn ZeroOrMoreBubFns / ()

SpaceAndBubFn = Space BubFn / f

// BubFn
// BubFn = Sum Space Sum Space Sum Space OrOrExpr Space Sum
BubFn = SumAndSpace BubFn1 / f
BubFn1 = SumAndSpace BubFn2 / f
BubFn2 = SumAndSpace BubFn3 / f
BubFn3 = OrOrExprAndSpace BubFn4 / f
BubFn4 = Sum () / f

SumAndSpace = Sum Space / f
OrOrExprAndSpace = OrOrExpr Space / f

// OrOr Expr
OrOrExpr = AndAndExpr OrOrExpr1 / AndAndExpr
OrOrExpr1 = OrOr OrOrExpr / f

OrOr = "||" () / f

// AndAnd Expr
AndAndExpr = ComparisonExpr AndAndExpr1 / ComparisonExpr
AndAndExpr1 = AndAnd AndAndExpr / f

AndAnd = "&&" () / f

// Comparison Expr
ComparisonExpr = Sum ComparisonExpr1 / f
ComparisonExpr1 = Comparison Sum / f

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

// Sum
Sum = Term ZeroOrMorePlusOrMinusAndTerms / f
ZeroOrMorePlusOrMinusAndTerms = PlusOrMinusAndTerm ZeroOrMorePlusOrMinusAndTerms / ()
PlusOrMinusAndTerm = PlusOrMinus Term / f

// Term
Term = Factor ZeroOrMoreStarOrSlashAndFactors / f
ZeroOrMoreStarOrSlashAndFactors = StarOrSlashAndFactor ZeroOrMoreStarOrSlashAndFactors / ()
StarOrSlashAndFactor = StarOrSlash Factor / f

// Factor
Factor = PlusOrMinus Factor / Power

// Power
Power = Atom PowerAndFactor / Atom
PowerAndFactor = '^' Factor / f

// Atom
Atom = ExprInParentheses () / Atom1
Atom1 = FloatLiteral () / Atom2
Atom2 = IntegerLiteral () / Atom3
Atom3 = Function () / Atom4
Atom4 = Variable () / Atom5
Atom5 = Constant () / f

// Variable
Variable = UppercaseX () / Variable1
Variable1 = UppercaseY () / Variable2
Variable2 = UppercaseZ () / Variable3
Variable3 = LowercaseX () / Variable4
Variable4 = LowercaseY () / Variable5
Variable5 = LowercaseZ () / Variable6
Variable6 = UppercaseN () / Variable7
Variable7 = LowercaseN () / Variable8
Variable8 = UppercaseF () / Variable9
Variable9 = UppercaseS () / f

UppercaseX = 'X' () / f
UppercaseY = 'Y' () / f
UppercaseZ = 'Z' () / f
LowercaseX = 'x' () / f
LowercaseY = 'y' () / f
LowercaseZ = 'z' () / f
UppercaseN = 'N' () / f
LowercaseN = 'n' () / f
UppercaseF = 'F' () / f
UppercaseS = 'S' () / f

// Constant
Constant = E () / Constant1
Constant1 = Pi () / f

E = 'E' () / f
Pi = "PI" () / f

// Function
Function = Sine () / Function1
Function1 = Cosine () / Function2
Function2 = Tangent () / Function3
Function3 = Ln () / Function4
Function4 = Lg () / f

Sine = "sin" Factor / f
Cosine = "cos" Factor / f
Tangent = "tan" Factor / f
Ln = "ln" Factor / f
Lg = "lg" Factor / f

// Delimiters
ExprInParentheses = '(' ExprAndClose / f
ExprAndClose = Sum ')' / f

// Integer
IntegerLiteral = DecLiteral () / f

// Float
FloatLiteral = DecLiteral PointAndDecLiteral / BytesF64Literal
PointAndDecLiteral = '.' DecLiteral / f

BytesF64Literal = 'b' ???????? / f

// Dec
DecLiteral = DecDigit ZeroOrMoreDecDigits / f
ZeroOrMoreDecDigits = DecDigit ZeroOrMoreDecDigits / ()

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

Space = ' ' () / f
```


# Floaout File Format Specification

## Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Spec Version | `u8` (1) | Version of Floaout File Format Specification. |
| Floaout ID | `u128` (16) | Floaout ID of this file. The value is 0 if the song is undefined. |
| Floaout Version | `u16` (2) | Version of Floaout |
| Bubbles | `u16` (2) | Number of Bubbles |
| Frames | `u64` (8) | Number of frames |
| Samples Per Sec | `f64` (8) | Samples per sec |
| LpcmKind | `u8` (1) | `LpcmKind` |
| Title Size | `u8` (1) | Title Size (0~255) |
| Title | `String` | Title (UTF-8) |
| Artist Size | `u8` (1) | Artist Size (0~255) |
| Artist | `String` | Artist (UTF-8) |
| CRC-32K/4.2 | `u32` (4) | Max length at Hamming Distance 4 is 2147483615 (bits). And max length at Hamming Distance 6 is 6167 (bits). |

## Each Bubble
Bubble Files will be 'i.bub' (i = 0, ... , Bubbles - 1)

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Name Size | `u8` (1) | Name Size (0~255) |
| File Name | `String` | Bubble File Name without ".bub" (UTF-8) |
| Bubble Starting Frames | `u16` (2) | Number of Bubble Starting Frames |
| Bubble Starting Frame | `u64` (8) | Bubble Starting Frame |
| ... | ... | ... |
| Bubble Starting Frame | `u64` (8) | Bubble Starting Frame |
| CRC-32K/4.2 | `u32` (4) | From the previous CRC. |
