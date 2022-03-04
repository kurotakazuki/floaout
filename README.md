# Floaout

Floaout is the next-generation audio format.

## Bubble File Format Specification


### Root Node
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Tree Size | `ULEB128` | An unsigned value identifying the size of `Tree`. This size value doesn't include the size on or before the `Node ID` field. |
| Metadata |  | Metadata. |

### Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `2`. |
| Optional Metadata (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Audio Data (Child_1 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_1 Node`. |
| Required Metadata |  | Binary data. |
| Optional Metadata (Child_0 Node) | {`Optional Metadata (Child_0 Node) Size`} | Optional Metadata (Child_0 Node). (Details in Optional Metadata table.) |
| Audio Data (Child_1 Node) | {`Audio Data (Child_1 Node) Size`} | Audio Data (Child_1 Node). (Details in Audio Data table.) |

#### Required Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Spec Version | `u8` (1) | Version of Bubble File Format Specification. The value is `0`. |
| Minor Spec Version | `u8` (1) | Minor Version of Bubble File Format Specification. Will be deleted in Spec Version 1.  The value is `0`. |
| Bubble ID | `ULEB128` | Bubble ID of this file. The value is 0 if the Bubble is undefined. If it is not 0, it must be based on the ID managed by bkbkb.net. |
| Frames | `ULEB128` | Number of frames. |
| Frames Per Sec | `ULEB128` | Frames per second. |

#### Optional Metadata

- Instruments Name

- Ident


#### Audio Data
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Scope || Scope. (Details in Scope table.) |

##### Scope
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. The value is 1 (1 UDF) + { Number of Scopes and BubFnsBlocks }. |
| UDF (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Scope or BubFnsBlock (Child_i Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_i Node`. |
| UDF (Child_0 Node) | {`UDF (Child_0 Node) Size`} | UDF (Child_0 Node). |
| Scope or BubFnsBlock (Child_i Node) | {`Scope or BubFnsBlock (Child_i Node) Size`} | Child_i Scope or BubFnsBlock (Child_i Node). |


##### User-defined Formula (UDF)
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. |
| Array, Assignment, Map, or Variable  (Child_i Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_i Node`. |
| Array, Assignment, Map, or Variable  (Child_i Node) | {`Array, Assignment, Map, or Variable  (Child_i Node) Size`} | Child_i Array, Assignment, Map, or Variable  (Child_i Node). |

###### UDF Kind
| Variant | Description | Value (`Type`) |
| ------------- | ------------- | ------------- |
| Array `a` |  | 0 (`u8`) |
| Assignment |  | 1 (`u8`) |
| Constant |  | 2 (`u8`) |
| Map `m` |  | 3 (`u8`) |
| Variable `v` |  | 4 (`u8`) |

###### Array (Leaf Node)
Ident is `a`.

```
a[1]
```

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| UDFKind | `u8` (1) | `UDFKind::Array` |
| Array ID | `ULEB128` | An unsigned value identifying the size of Array Ident. |
| ArrayKind | `u8` (1) | `ArrayKind` |
| Array |  | Data of Array |

###### Array Kind
| Variant | Description | Value (`Type`) |
| ------------- | ------------- | ------------- |
| LEu8 |  | 0 (`[u8]`) |
| LEu16 |  | 1 (`[u16]`) |
| LEu32 |  | 2 (`[u32]`) |
| LEu64 |  | 3 (`[u64]`) |
| LEu128 |  | 4 (`[u128]`) |
| LEi8 |  | 5 (`[i8]`) |
| LEi16 |  | 6 (`[i16]`) |
| LEi32 |  | 7 (`[i32]`) |
| LEi64 |  | 8 (`[i64]`) |
| LEi128 |  | 9 (`[i128]`) |
| LEf32 |  | 10 (`[f32]`) |
| LEf64 |  | 11 (`[f64]`) |

###### Assignment (Leaf Node)
```
A[n-1]=n+1
```

Assign after the BubFnsBlock output is generated at each frame.

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| UDFKind | `u8` (1) | `UDFKind::Assignment` |
| AsgmtExpr | `AsgmtExpr` | Assignment Expression |

###### User-defined Map (Function) (Leaf Node)
Ident is `m`.

Parameter (Argument) ident is `p`.

```
m(p0, p1)=p0+p1+1
```

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| UDFKind | `u8` (1) | `UDFKind::Map` |
| Map ID | `ULEB128` | Map ID |
| Number of Arguments | `ULEB128` | Number of Arguments |
| Map | `Sum` | Map |

###### User-defined Variable (Leaf Node)
Ident is `v`.

```
v=a+1
```

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| UDFKind | `u8` (1) | `UDFKind::Variable` |
| Variable ID | `ULEB128` | Variable ID |
| Variable | `Sum` | Variable |

##### BubFnsBlock
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. |
| BubFn (Child_i Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_i Node`. |

| Head Absolute Frame | `ULEB128` | Number of frames at the start of `BubFnsBlock`. |
| BubFnsBlock Frames | `ULEB128` | Number of frames in `BubFnsBlock`. `Tail Absolute Frame` - `Head Absolute Frame` + 1 |

| BubFn (Child_i Node) | {`BubFn (Child_i Node) Size`} | Child_i BubFn (Child_i Node). |



##### BubFn
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. The value is `5`. |
| Bubble's absolute X coordinate (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Bubble's absolute Y coordinate (Child_1 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_1 Node`. |
| Bubble's absolute Z coordinate (Child_2 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_2 Node`. |
| Domain (Child_3 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_3 Node`. |
| Output (Child_4 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_4 Node`. |
| Bubble's absolute X coordinate |  | Bubble's absolute X coordinate. |
| Bubble's absolute Y coordinate |  | Bubble's absolute Y coordinate. |
| Bubble's absolute Z coordinate |  | Bubble's absolute Z coordinate. |
| Domain |  | Domain. |
| Output |  | Output. |

###### Bubble's absolute (X|Y|Z) coordinate (Leaf Node)
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| Bubble's absolute (X|Y|Z) coordinate | `Sum` | Bubble's absolute (X|Y|Z) coordinate ((X|Y|Z)_b) |

###### Domain (Leaf Node)
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| Domain | `OrOrExpr` | Domain |

###### Output (Leaf Node)
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| Output | `Sum` | Output |



### Keywords

#### Built-in Variables
Ident is `V`.

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- |
| V | `Char` (1) | Ident is `V`. |
| BVariableKind | `ULEB128` | Built-in Variable Kind |

##### Built-in Variable Kind (BVariableKind)
This data is `ULEB128`.

| Variant | Description | Value |
| ------------- | ------------- | ------------- |
| X | Speaker's absolute X coordinate | 0 |
| Y | Speaker's absolute Y coordinate | 1 |
| Z | Speaker's absolute Z coordinate | 2 |
| x | x = X - X_0 (X_0 is Bubble's absolute X coordinate). Speaker's relative X coordinate. | 3 |
| y | y = Y - Y_0 (Y_0 is Bubble's absolute Y coordinate). Speaker's relative Y coordinate. | 4 |
| z | z = Z - Z_0 (Z_0 is Bubble's absolute Z coordinate). Speaker's relative Z coordinate. | 5 |
| N | Absolute frame n. Number of frames starting from the file. (`as f64`) | 6 |
| n | Relative frame n. Number of frames starting from at the start of `BubFnsBlock`.(`as f64`) | 7 |
| F | Absolute Frames (`as f64`) | 8 |
| f | Relative Frames (`as f64`) | 9 |
| T | Absolute Time. T = N / F | 10 |
| t | Relative Time. t = n / f | 11 |
| S | Frames per sec. (`as f64`) f_s | 12 |

#### Built-in Constants
Ident is `C`.

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- |
| C | `Char` (1) | Ident is `C`. |
| BCKind | `ULEB128` | Built-in Constant Kind |

##### Built-in Constant Kind (BCKind)
This data is `ULEB128`.

| Variant | Description | Value |
| ------------- | ------------- | ------------- |
| E | Euler’s number (e) | 0 |
| Frac1Pi | 1/π | 1 |
| Frac1Sqrt2 | 1/sqrt(2) | 2 |
| Frac2Pi | 2/π | 3 |
| Frac2SqrtPi | 2/sqrt(π) | 4 |
| FracPi2 | π/2 | 5 |
| FracPi3 | π/3 | 6 |
| FracPi4 | π/4 | 7 |
| FracPi6 | π/6 | 8 |
| FracPi8 | π/8 | 9 |
| Ln2 | ln(2) | 10 |
| Ln10 | ln(10) | 11 |
| Log210 | log2(10) | 12 |
| Log2E | log2(e) | 13 |
| Log102 | log10(2) | 14 |
| Log10E | log10(e) | 15 |
| Pi | Archimedes’ constant (π) | 16 |
| Sqrt2 | sqrt(2) | 17 |
| Tau | The full circle constant (τ) | 18 |

#### Built-in Maps (Functions)
Ident is `M`.

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- |
| M | `Char` (1) | Ident is `M`. |
| BMapKind | `ULEB128` | Built-in Map Kind |
| Open paren |  | `(` |
| Arguments |  | Arguments |
| Close paren |  | `)` |

##### Built-in Map Kind (BMapKind)
This data is `ULEB128`.

| Variant | Description | Arguments | Value |
| ------------- | ------------- | ------------- |
| Sin | Sine | n | 0 |
| Cos | Cosine | n | 1 |
| Tan | Tangent | n | 2 |
| Log | The logarithm. | base, n | 3 |
| Ln | The natural logarithm of the number. | n | 4 |
| Log2 | The base 2 logarithm of the number. | n | 5 |
| Log10 | The base 2 logarithm of the number. | n | 6 |

<!-- #### Others
| Keyword | Description |
| ------------- | ------------- |
| b | `b????????` `f64` | -->

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
| ( ) | Parentheses |

### Syntax
```rust ignore
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
Atom3 = Map () / Atom4
Atom4 = Variable () / Atom5
Atom5 = Constant () / f

// Variable
Variable = BIOrUDVariable ? / f
BIOrUDVariable = 'V' () / 'v'

// Constant
Constant = 'C' ? / f

// Map (Function)
Map = BIOrUDMap Map1 / f
BIOrUDMap = 'M' () / 'm'
// TODO Change into ULEB128.
Map1 = ? Map2 / f
Map2 = '(' Map3 / f
Map3 = MapParams ')' / f

MapParams = Sum ZeroOrMoreMapParams / f
ZeroOrMoreMapParams = CommaAndMapParam ZeroOrMoreMapParams / ()
CommaAndMapParam = ',' Sum / f

// Delimiters
ExprInParentheses = '(' ExprAndClose / f
ExprAndClose = Sum ')' / f

// Integer
IntegerLiteral = DecLiteral () / f

// Float
FloatLiteral = DecLiteral PointAndDecLiteral / BytesF64Literal
PointAndDecLiteral = '.' DecLiteral / f

// BytesF64Literal = 'b' ???????? / f

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
```



## TODO
- [ ] Add summation
- [ ] Add CRC
- [ ] Add assignment in BubFn
- [ ] Add Others
- [ ] Add Output array or variable
- [ ] Add f64::consts::EPSILON
- [ ] Change Number Literal to `u{ULEB128}`, `i{Signed LEB128}`, `f????`, and `d????????`