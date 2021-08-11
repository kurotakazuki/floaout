# Floaout
Floaout is the next-generation audio format.

# TODO
- Add CRC
- Delete Metadata and &mut self to self in WavMetadata
- Rename BubbleMetadata.next_head_frame to next_head_absolute_frames and to `Option<u64>`
- Add read_bubble_functions_block
- Create Coordinates structure
- Add write_frames in BubbleFrameWriter
- error handling
- Clarify whether #[derive(Order)] is needed
- Add Functions like pow, sinh, ...

# Bubble File Format Specification

## Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Spec Version | `u8` (1) | Version of Bubble File Format Specification. |
| Bubble ID | `u128` (16) | Bubble ID of this file. If undefined Bubble, than value is 0. |
| Bubble Version | `u16` (2) | Version of Bubble |
| Frames | `u64` (8) | Number of frames |
| Samples Per Sec | `f64` (8) | Samples per sec |
| LpcmKind | `u8` (1) | `SampleKind` |
| BubbleSampleKind | `u8` (1) | `BubbleSampleKind` |
| Name Size | `u8` (1) | Name Size (0~255) |
| Name | `String` | Name (UTF-8) |
| CRC-32K/4.2 | `u32` (4) | Max length at Hamming Distance 4 is 2147483615 (bits). And max length at Hamming Distance 6 is 6167 (bits). |

### LpcmKind
| Variant  | Description | Value (`Type`) |
| ------------- | ------------- | ------------- |
| F32LE | `f32` Little Endian | 0 (`u8`) |
| F64LE | `f64` Little Endian | 1 (`u8`) |

### BubbleSampleKind
| Variant  | Description | Value |
| ------------- | ------------- | ------------- |
| LPCM | LPCM | 0 |
| Expression | Expression | 1 |

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
| Connected, Ended and Functions size | `u16` (1) | Connected, Ended and Functions size |
| Bubble's X coordinate | `Sum` | Bubble's X coordinate (X_0) |
| Space | `char` (1) | ' ' |
| Bubble's Y coordinate | `Sum` | Bubble's Y coordinate (Y_0) |
| Space | `char` (1) | ' ' |
| Bubble's Z coordinate | `Sum` | Bubble's Z coordinate (Z_0) |
| Space | `char` (1) | ' ' |
| Domain | `OrOrExpression` |  |
| Space | `char` (1) | ' ' |
| Volume | `Sum` |  |
| Space or Empty | `char` (1) | ' ' if there is another |
| Tail Relative Frame | `u64` (8) | Number of frames at the end of function. |
| Next Head Relative Frame | `u64` (8) | Number of frames at the start of the next function. Optional (!connected && !ended) |
| Sample Data |  | Sample Data |
| CRC-32K/4.2 | `u32` (4) | After every tail frames. From the previous CRC. |


### Connected, Ended and Functions size
| Name | `Type` (bits) | Description |
| ------------- | ------------- | ------------- |
| Functions Size | `u16` (14)  | Functions Size is 14 bits |

| ended \ connected | 0?????????????? | 1?????????????? |
| ------------- | ------------- | ------------- |
| ?0????????????? | Stopped (NST) | Head |
| ?1????????????? | Ended | x |

### Sample Data
#### LPCM
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Sample | `f32` or `f64` (4 or 8) | depends on `SampleKind` |

#### Expression
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Expression Size | `u16` (2) | Expression Size |
| Expression | `Sum` | Expression |


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
// BubbleFunctions
BubbleFunctions = BubbleFunction ZeroOrMoreBubbleFunctions / f
ZeroOrMoreBubbleFunctions = SpaceAndBubbleFunction ZeroOrMoreBubbleFunctions / ()

SpaceAndBubbleFunction = Space BubbleFunction / f

// BubbleFunction
// BubbleFunction = Sum Space Sum Space Sum Space OrOrExpression Space Sum
BubbleFunction = SumAndSpace BubbleFunction1 / f
BubbleFunction1 = SumAndSpace BubbleFunction2 / f
BubbleFunction2 = SumAndSpace BubbleFunction3 / f
BubbleFunction3 = OrOrExpressionAndSpace BubbleFunction4 / f
BubbleFunction4 = Sum () / f

SumAndSpace = Sum Space / f
OrOrExpressionAndSpace = OrOrExpression Space / f

// OrOr Expression
OrOrExpression = AndAndExpression OrOrExpression1 / AndAndExpression
OrOrExpression1 = OrOr OrOrExpression / f

OrOr = "||" () / f

// AndAnd Expression
AndAndExpression = ComparisonExpression AndAndExpression1 / ComparisonExpression
AndAndExpression1 = AndAnd AndAndExpression / f

AndAnd = "&&" () / f

// Comparison Expression
ComparisonExpression = Sum ComparisonExpression1 / f
ComparisonExpression1 = Comparison Sum / f

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
Atom = ExpressionInParentheses () / Atom1
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
ExpressionInParentheses = '(' ExpressionAndClose / f
ExpressionAndClose = Sum ')' / f

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