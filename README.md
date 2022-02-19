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
| Required Metadata (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Optional Metadata (Child_1 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_1 Node`. |
| Audio Data (Child_2 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_2 Node`. |
| Required Metadata (Child_0 Node) | {`Required Metadata (Child_0 Node) Size`} | Required Metadata (Child_0 Node).  (Details in the Required Metadata table.)|
| Optional Metadata (Child_1 Node) | {`Optional Metadata (Child_1 Node) Size`} | Optional Metadata (Child_1 Node).  (Details in the Optional Metadata table.)|
| Audio Data (Child_2 Node) | {`Audio Data (Child_2 Node) Size`} | Audio Data (Child_2 Node).  (Details in the Audio Data table.)|

#### Required Metadata

#### Optional Metadata


#### Audio Data
| Name | `Type` (Bytes) | Description |
| ------------- | ------------- | ------------- |
| Number of Child Nodes | ULEB128 | An unsigned value identifying the size of `Number of Child Nodes`. 1 (1 Formula) + { Number of Scopes and Blocks } |
| Formula (Child_0 Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_0 Node`. |
| Scope or Block (Child_i Node) Size | `ULEB128` | An unsigned value identifying the size of `Child_i Node`. |
| Formula (Child_0 Node) | {`Formula (Child_0 Node) Size`} | Formula (Child_0 Node). |
| Scope or Block (Child_i Node) | {`Scope or Block (Child_i Node) Size`} | Child_i Scope or Block (Child_i Node). |


##### Formula


###### Function

###### Array

###### Variable

###### Assign
Assign after the Block outpout is generated at each frame.

##### Scope

##### Block
