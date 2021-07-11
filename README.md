# Floaout
Floaout is the next-generation audio format.





# Bubble File Format Specification

## Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Bubble | `str` (3) | "bub" means Bubble |
| Version | `u8` (1) | Version of Bubble |
| Bubble ID | `u128` (16) | Bubble ID of this file. If undefined Bubble, than value is 0. |
| Frames | `u64` (8) | Number of frames |
| Samples Per Sec | `f64` (8) | Samples per sec |
| Bits Per Sample | `u8` (1) | `SampleKind` |
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
| Comma or Period | `char` (1) | ',' if there is another |
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
| Keyword | Description |
| ------------- | ------------- |
| X | X (Absolute) coordinate |
| Y | Y (Absolute) coordinate |
| Z | Z (Absolute) coordinate |
| x | x = X - X_0 (X_0 is Bubble's X coordinate). (Relative) coordinate. |
| y | y = Y - Y_0 (Y_0 is Bubble's Y coordinate). (Relative) coordinate. |
| z | z = Z - Z_0 (Z_0 is Bubble's Z coordinate). (Relative) coordinate. |
| T | Number of frames starting from the file. (Absolute) Time. |
| t | Number of frames starting from the function. (Relative) Time. |
| F | Samples per sec |
|  |  |
| PI | Pi |
| E | Euler's number |
|  |  |
| f | `f64` |
|  |  |
| sin | Sine |
| cos | Cosine |
| tan | Tangent |
| ln | The natural logarithm of the number. |
| lg | The base 2 logarithm of the number. |