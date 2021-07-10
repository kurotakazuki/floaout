# Floaout
Floaout is the next-generation audio format.





# Bubble File Format Specification

## Metadata
| Name | `Type` (Bytes) | Contents |
| ------------- | ------------- | ------------- |
| Bubble | `str` (3) | "bub" means Bubble |
| Version | `u8` (1) | Version of Bubble |
| Bubble ID | `u128` (16) | Bubble ID of this file. If undefined Bubble, than value is 0. |
| Frames | `u64` (8) | Number of Frames |
| Samples Per Sec | `f64` (8) | Samples Per Sec |
| Bits Per Sample | `u8` (1) | `SampleKind` |
| Name Size | `u8` (1) | Name Size |
| Name | `String` | Name (UTF-8) |
| CRC | `` () | Pending |

## Each Sample
|  | `` () |  |