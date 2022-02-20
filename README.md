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
| Number of Child Nodes | `u8` (1) | The value is `3`. |
| Optional Metadata (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Audio Data (Child_1 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_1 Node`. |
| Required Metadata |  | Binary data. |
| Optional Metadata (Child_0 Node) | {`Optional Metadata (Child_0 Node) Size`} | Optional Metadata (Child_0 Node). (Details in Optional Metadata table.) |
| Audio Data (Child_1 Node) | {`Audio Data (Child_1 Node) Size`} | Audio Data (Child_1 Node). (Details in Audio Data table.) |

#### Required Metadata
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Spec Version | `u8` (1) | Version of Bubble File Format Specification. The value is `0`. |
| Bubble ID | `ULEB128` | Bubble ID of this file. The value is 0 if the Bubble is undefined. If it is not 0, it must be based on the ID managed by bkbkb.net. |
| Minor Spec Version | `u8` (1) | Minor Version of Bubble File Format Specification. Will be deleted in Spec Version 1.  The value is `0`. |
| Frames | `ULEB128` | Number of frames. |
| Frames Per Sec | `ULEB128` | Frames per second. |

#### Optional Metadata


#### Audio Data
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Scope || Scope. (Details in Scope table.) |

##### Scope
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. The value is 1 (1 Formula) + { Number of Scopes and BubFnsBlocks }. |
| Formula (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Scope or BubFnsBlock (Child_i Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_i Node`. |
| Formula (Child_0 Node) | {`Formula (Child_0 Node) Size`} | Formula (Child_0 Node). |
| Scope or BubFnsBlock (Child_i Node) | {`Scope or BubFnsBlock (Child_i Node) Size`} | Child_i Scope or BubFnsBlock (Child_i Node). |


##### Formula
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. |
| Array, Assignment, Function, or Variable  (Child_i Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_i Node`. |
| Array, Assignment, Function, or Variable  (Child_i Node) | {`Array, Assignment, Function, or Variable  (Child_i Node) Size`} | Child_i Array, Assignment, Function, or Variable  (Child_i Node). |

###### Formula Kind
| Variant  | Description | Value (`Type`) |
| ------------- | ------------- | ------------- |
| Array |  | 0 (`u8`) |
| Assignment |  | 1 (`u8`) |
| Function |  | 2 (`u8`) |
| Variable |  | 3 (`u8`) |

###### Array (Leaf Node)
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| FormulaKind | `u8` (1) | `FormulaKind::Array` |
| Array Ident Size | `ULEB128` | An unsigned value identifying the size of Array Ident. |
| Array Ident | `String` | Ident of Array |
| ArrayKind | `u8` (1) | `ArrayKind` |
| Array |  | Data of Array |

###### Assignment (Leaf Node)
Assign after the BubFnsBlock outpout is generated at each frame.

| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| FormulaKind | `u8` (1) | `FormulaKind::Assignment` |
| AsgmtExpr | `AsgmtExpr` | Assignment Expression  |

###### Function (Leaf Node)
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| FormulaKind | `u8` (1) | `FormulaKind::Function` |
| Function Ident Size | `ULEB128` | An unsigned value identifying the size of Function Ident. |
| Function Ident | `String` | Ident of Function |
| Arguments | ` | Arguments  // TODO |
| Function | `Sum` | Function |

###### Variable (Leaf Node)
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `u8` (1) | The value is `0`. |
| FormulaKind | `u8` (1) | `FormulaKind::Variable` |
| Variable Ident Size | `ULEB128` | An unsigned value identifying the size of Variable Ident. |
| Variable Ident | `String` | Ident of Variable |
| Variable | `Sum` | Variable |

##### BubFnsBlock
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. |
| BubFn (Child_i Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_i Node`. |

| Head Absolute Frame | `ULEB128` | Number of frames at the start of `BubFnsBlock`. |
| Tail Absolute Frame | `ULEB128` | Number of frames at the end of `BubFnsBlock`. |

| BubFn (Child_i Node) | {`BubFn (Child_i Node) Size`} | Child_i BubFn (Child_i Node). |



##### BubFn
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | `ULEB128` | An unsigned value identifying the size of `Number of Child Nodes`. The value is `5`. |
| Bubble's absolute X coordinate (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Bubble's absolute Y coordinate (Child_1 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_1 Node`. |
| Bubble's absolute X coordinate (Child_2 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_2 Node`. |
| Domain (Child_3 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_3 Node`. |
| Output (Child_4 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_4 Node`. |

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



## TODO
- [ ] Add summation
- [ ] Add CRC