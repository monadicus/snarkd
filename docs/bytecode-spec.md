# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [ir.proto](#ir-proto)
    - [Function](#ir-Function)
    - [Header](#ir-Header)
    - [Input](#ir-Input)
    - [Program](#ir-Program)

- [opcode.proto](#opcode-proto)
    - [AssertData](#opcode-AssertData)
    - [BinaryData](#opcode-BinaryData)
    - [Instruction](#opcode-Instruction)
    - [TernaryData](#opcode-TernaryData)
    - [UnaryData](#opcode-UnaryData)

- [operand.proto](#operand-proto)
    - [Address](#operand-Address)
    - [Field](#operand-Field)
    - [Group](#operand-Group)
    - [GroupCoordinate](#operand-GroupCoordinate)
    - [Operand](#operand-Operand)
    - [Record](#operand-Record)
    - [RecordType](#operand-RecordType)
    - [RecordTypeEntry](#operand-RecordTypeEntry)
    - [Scalar](#operand-Scalar)
    - [SimpleType](#operand-SimpleType)
    - [Struct](#operand-Struct)
    - [StructType](#operand-StructType)
    - [StructTypeEntry](#operand-StructTypeEntry)
    - [TupleGroup](#operand-TupleGroup)
    - [Type](#operand-Type)
    - [VisibleData](#operand-VisibleData)

    - [Visibility](#operand-Visibility)

- [Scalar Value Types](#scalar-value-types)



<a name="ir-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## ir.proto
The structure of a Snarkd program


<a name="ir-Function"></a>

### Function
A function in a Snarkd program


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| argument_start_variable | [uint32](#uint32) |  | The ID of the function |
| instructions | [opcode.Instruction](#opcode-Instruction) | repeated | The instructions contained within the function |






<a name="ir-Header"></a>

### Header
The metadata of a Snarkd program


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snarkd_major | [uint32](#uint32) |  | The major release version of snarkd this programn was made for |
| snarkd_minor | [uint32](#uint32) |  | The minor release version of snarkd this programn was made for |
| snarkd_patch | [uint32](#uint32) |  | The patch release version of snarkd this programn was made for |
| main_inputs | [Input](#ir-Input) | repeated | A list of main input registers |
| constant_inputs | [Input](#ir-Input) | repeated | A list of constant input registers |
| register_inputs | [Input](#ir-Input) | repeated | A list of registers inputs |
| public_states | [Input](#ir-Input) | repeated | A list of public state inputs |
| private_record_states | [Input](#ir-Input) | repeated | A list of private record state inputs |
| private_leaf_states | [Input](#ir-Input) | repeated | A list of private leaf state inputs |






<a name="ir-Input"></a>

### Input
A register input for a program


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| variable | [uint32](#uint32) |  | The ID of the register |
| name | [string](#string) |  | The name of the register, used for debugging purposes |
| type | [operand.Type](#operand-Type) |  | The type of the register |






<a name="ir-Program"></a>

### Program
A Snarkd program


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [Header](#ir-Header) |  | The metadata of the program |
| functions | [Function](#ir-Function) | repeated | The functions within the program |















<a name="opcode-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## opcode.proto
Instructions supported by Snarkd


<a name="opcode-AssertData"></a>

### AssertData
The argumnents for an assertion operation


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lhs | [operand.Operand](#operand-Operand) |  | the lhs value of the operation |
| rhs | [operand.Operand](#operand-Operand) |  | the rhs value of the operation |






<a name="opcode-BinaryData"></a>

### BinaryData
The argumnents for a binary operation


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lhs | [operand.Operand](#operand-Operand) |  | the lhs value of the operation |
| rhs | [operand.Operand](#operand-Operand) |  | the rhs value of the operation |
| dest | [uint32](#uint32) |  | the register to store the result in |






<a name="opcode-Instruction"></a>

### Instruction
A Snarkd instruction


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| abs | [UnaryData](#opcode-UnaryData) |  | Computes the absolute value of the input, checking for overflow, storing the result in the destination register.<br>For integer types, a constraint is added to check for underflow. For cases where wrapping semantics are needed, see the abs.w instruction. This underflow happens when the input is the minimum value of a signed integer type. For example, abs -128i8 would result in underflow, since 128 cannot be represented as an i8. |
| abs_wrapped | [UnaryData](#opcode-UnaryData) |  | Compute the absolute value of the input, wrapping around at the boundary of the type, and storing the result in the destination register. |
| add | [BinaryData](#opcode-BinaryData) |  | Adds first with second, storing the outcome in destination.<br>For integer types, a constraint is added to check for overflow. For cases where wrapping semantics are needed for integer types, see the add.w instruction. |
| add_wrapped | [BinaryData](#opcode-BinaryData) |  | Adds first with second, wrapping around at the boundary of the type, and storing the outcome in destination. |
| and | [BinaryData](#opcode-BinaryData) |  | Performs an AND operation on integer (bitwise) or boolean first and second, storing the outcome in destination. |
| assert_eq | [AssertData](#opcode-AssertData) |  | Checks whether first and second are equal, halting if they are not equal. |
| assert_neq | [AssertData](#opcode-AssertData) |  | Checks whether first and second are not equal, halting if they are equal. |
| commit_bhp_256 | [BinaryData](#opcode-BinaryData) |  | Computes a BHP commitment on inputs of 256-bit chunks in first, and some randomness in second, storing the commitment in destination. Randomness should always be a Scalar value, and the produced commitment will always be a Field value.<br>The instruction will halt if the given input is smaller than 129 bits. |
| commit_bhp_512 | [BinaryData](#opcode-BinaryData) |  | Computes a BHP commitment on inputs of 512-bit chunks in first, and some randomness in second, storing the commitment in destination. Randomness should always be a Scalar value, and the produced commitment will always be a Field value.<br>The instruction will halt if the given input is smaller than 171 bits. |
| commit_bhp_768 | [BinaryData](#opcode-BinaryData) |  | Computes a BHP commitment on inputs of 768-bit chunks in first, and some randomness in second, storing the commitment in destination. Randomness should always be a Scalar value, and the produced commitment will always be a Field value.<br>The instruction will halt if the given input is smaller than 129 bits. |
| commit_bhp_1024 | [BinaryData](#opcode-BinaryData) |  | Computes a BHP commitment on inputs of 1024-bit chunks in first, and some randomness in second, storing the commitment in destination. Randomness should always be a Scalar value, and the produced commitment will always be a Field value.<br>The instruction will halt if the given input is smaller than 171 bits. |
| commit_ped_64 | [BinaryData](#opcode-BinaryData) |  | Computes a Pedersen commitment up to a 64-bit input in first, and some randomness in second, storing the commitment in destination. Randomness should always be a Scalar value, and the produced commitment will always be a Group value.<br>The instruction will halt if the given String or Interface value exceeds the 64-bit limit. |
| commit_ped_128 | [BinaryData](#opcode-BinaryData) |  | Computes a Pedersen commitment up to a 128-bit input in first, and some randomness in second, storing the commitment in destination. Randomness should always be a Scalar value, and the produced commitment will always be a Group value.<br>The instruction will halt if the given String or Interface value exceeds the 128-bit limit. |
| div | [BinaryData](#opcode-BinaryData) |  | Divides first by second, storing the outcome in destination. Halts on division by zero.<br>For integer types, this operation performs truncated division. Furthermore, a constraint is added to check for underflow. This underflow happens when dividing the minimum value of a signed integer type by -1. For example, div -128i8 -1i8 would result in underflow, since 128 cannot be represented as an i8.<br>For cases where wrapping semantics are needed for integer types, see the div.w instruction. |
| div_wrapped | [BinaryData](#opcode-BinaryData) |  | Divides first by second, wrapping around at the boundary of the type, and storing the outcome in destination. |
| double | [UnaryData](#opcode-UnaryData) |  | Doubles the input, storing the outcome in destination. |
| gt | [BinaryData](#opcode-BinaryData) |  | Checks if first is greater than second, storing the result in destination. |
| gte | [BinaryData](#opcode-BinaryData) |  | Checks if first is greater than or equal to second, storing the result in destination. |
| hash_bhp_256 | [UnaryData](#opcode-UnaryData) |  | Computes a BHP hash on inputs of 256-bit chunks in first, storing the hash in destination. The produced hash will always be a Field value.<br>The instruction will halt if the given input is smaller than 129 bits. |
| hash_bhp_512 | [UnaryData](#opcode-UnaryData) |  | Computes a BHP hash on inputs of 512-bit chunks in first, storing the hash in destination. The produced hash will always be a Field value.<br>The instruction will halt if the given input is smaller than 171 bits. |
| hash_bhp_768 | [UnaryData](#opcode-UnaryData) |  | Computes a BHP hash on inputs of 768-bit chunks in first, storing the hash in destination. The produced hash will always be a Field value.<br>The instruction will halt if the given input is smaller than 129 bits. |
| hash_bhp_1024 | [UnaryData](#opcode-UnaryData) |  | Computes a BHP hash on inputs of 1024-bit chunks in first, storing the hash in destination. The produced hash will always be a Field value.<br>The instruction will halt if the given input is smaller than 171 bits. |
| hash_ped_64 | [UnaryData](#opcode-UnaryData) |  | Computes a Pedersen hash up to a 64-bit input in first, storing the hash in destination. The produced hash will always be a Field value.<br>The instruction will halt if the given String or Interface value exceeds the 64-bit limit. |
| hash_ped_128 | [UnaryData](#opcode-UnaryData) |  | Computes a Pedersen hash up to a 128-bit input in first, storing the hash in destination. The produced hash will always be a Field value.<br>The instruction will halt if the given String or Interface value exceeds the 128-bit limit. |
| hash_psd_2 | [UnaryData](#opcode-UnaryData) |  | Calculates a Poseidon hash with an input rate of 2, from an input in first, storing the hash in destination. The produced hash will always be a Field value. |
| hash_psd_4 | [UnaryData](#opcode-UnaryData) |  | Calculates a Poseidon hash with an input rate of 4, from an input in first, storing the hash in destination. The produced hash will always be a Field value. |
| hash_psd_8 | [UnaryData](#opcode-UnaryData) |  | Calculates a Poseidon hash with an input rate of 8, from an input in first, storing the hash in destination. The produced hash will always be a Field value. |
| inv | [UnaryData](#opcode-UnaryData) |  | Computes the multiplicative inverse of the input, storing the outcome in destination. |
| is_eq | [BinaryData](#opcode-BinaryData) |  | Compares first and second, storing the result in destination. |
| is_neq | [BinaryData](#opcode-BinaryData) |  | Returns true if first is not equal to second, storing the result in destination. |
| lt | [BinaryData](#opcode-BinaryData) |  | Checks if first is less than second, storing the outcome in destination. |
| lte | [BinaryData](#opcode-BinaryData) |  | Checks if first is less than or equal to second, storing the outcome in destination. |
| mod | [BinaryData](#opcode-BinaryData) |  | Takes the modulus of first with respect to second, storing the outcome in destination. Halts if second is zero.<br>The semantics of this operation are consistent with the mathematical definition of modulo operation. |
| mul | [BinaryData](#opcode-BinaryData) |  | Multiplies first with second, storing the outcome in destination.<br>For integer types, a constraint is added to check for overflow/underflow. For cases where wrapping semantics are needed for integer types, see the mul.w instruction. |
| mul_wrapped | [BinaryData](#opcode-BinaryData) |  | Multiplies first with second, wrapping around at the boundary of the type, and storing the outcome in destination. |
| nand | [BinaryData](#opcode-BinaryData) |  | Returns false only if first and second are true, storing the outcome in destination. |
| neg | [UnaryData](#opcode-UnaryData) |  | Negates first, storing the outcome in destination.<br>For signed integer types, calling neg on the minimum value is an invalid operation. For example, the input -128i8 would not be valid since 128 cannot be represented as an i8. |
| nor | [BinaryData](#opcode-BinaryData) |  | Returns true when neither first nor second is true, storing the outcome in destination. |
| not | [UnaryData](#opcode-UnaryData) |  | Perform a NOT operation on an integer (bitwise) or boolean input, storing the outcome in destination. |
| or | [BinaryData](#opcode-BinaryData) |  | Performs an OR operation on integer (bitwise) or boolean first and second, storing the outcome in destination. |
| pow | [BinaryData](#opcode-BinaryData) |  | Raises first to the power of second, storing the outcome in destination.<br>For integer types, a constraint is added to check for overflow/underflow. For cases where wrapping semantics are needed for integer types, see the pow.w instruction. |
| pow_wrapped | [BinaryData](#opcode-BinaryData) |  | Raises first to the power of second, wrapping around at the boundary of the type, storing the outcome in destination. |
| rem | [BinaryData](#opcode-BinaryData) |  | Computes the truncated remainder of first divided by second, storing the outcome in destination. Halts on division by zero.<br>A constraint is added to check for underflow. This underflow happens when the associated division operation, div, underflows.<br>For cases where wrapping semantics are needed for integer types, see the rem.w instruction. |
| rem_wrapped | [BinaryData](#opcode-BinaryData) |  | Computes the truncated remainder of first divided by second, wrapping around at the boundary of the type, and storing the outcome in destination. |
| shl | [BinaryData](#opcode-BinaryData) |  | Shifts first left by second bits, storing the outcome in destination. |
| shl_wrapped | [BinaryData](#opcode-BinaryData) |  | Shifts first left by second bits, wrapping around at the boundary of the type, storing the outcome in destination. |
| shr | [BinaryData](#opcode-BinaryData) |  | Shifts first right by second bits, storing the outcome in destination. |
| shr_wrapped | [BinaryData](#opcode-BinaryData) |  | Shifts first right by second bits, wrapping around at the boundary of the type, storing the outcome in destination. |
| square | [UnaryData](#opcode-UnaryData) |  | Squares the input, storing the outcome in destination. |
| sqrt | [UnaryData](#opcode-UnaryData) |  | Computes the square root of the input, storing the outcome in destination. |
| sub | [BinaryData](#opcode-BinaryData) |  | Computes first - second, storing the outcome in destination. |
| sub_wrapped | [BinaryData](#opcode-BinaryData) |  | Computes first - second, wrapping around at the boundary of the type, and storing the outcome in destination. |
| ternary | [TernaryData](#opcode-TernaryData) |  | Selects first, if condition is true, otherwise selects second, storing the result in destination.<br>Example: ternary r0 r1 r2 into r3, where r0 is the condition, r1 is first, r2 is second, and r3 is the destination. |
| xor | [BinaryData](#opcode-BinaryData) |  | Performs an XOR operation on an integer (bitwise) or boolean first and second, storing the outcome in destination. |






<a name="opcode-TernaryData"></a>

### TernaryData
The argumnents for a ternary operation


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cond | [operand.Operand](#operand-Operand) |  | the condition that operation will use to make its selection |
| lhs | [operand.Operand](#operand-Operand) |  | the lhs value of the operation |
| rhs | [operand.Operand](#operand-Operand) |  | the rhs value of the operation |
| dest | [uint32](#uint32) |  | the register to store the result in |






<a name="opcode-UnaryData"></a>

### UnaryData
The arguments for a unary operation


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| operand | [operand.Operand](#operand-Operand) |  | the value to be operated on |
| dest | [uint32](#uint32) |  | the register to store the result in |















<a name="operand-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## operand.proto
Types and Values supported by snarkd


<a name="operand-Address"></a>

### Address
The address of an Aleo account


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  | TODO size? |






<a name="operand-Field"></a>

### Field
A native field element as an unsigned number up to
the modulus length of the field


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| negate | [bool](#bool) |  | TODO |
| values | [fixed64](#fixed64) | repeated | TODO |






<a name="operand-Group"></a>

### Group
The set of affine points on the elliptic curve passed into the
snarkd-vm forms a group. Snarkd supports this set as a primitive
data type. Group elements are special since their values can be
defined from the x-coordinate of a coordinate pair.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| single | [Field](#operand-Field) |  | TODO |
| tuple | [TupleGroup](#operand-TupleGroup) |  | TODO |






<a name="operand-GroupCoordinate"></a>

### GroupCoordinate
TODO


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| GroupField | [Field](#operand-Field) |  |  |
| SignHigh | [SimpleType](#operand-SimpleType) |  |  |
| SignLow | [SimpleType](#operand-SimpleType) |  |  |
| Inferred | [SimpleType](#operand-SimpleType) |  |  |






<a name="operand-Operand"></a>

### Operand
An instance of a Snarkd value


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [Address](#operand-Address) |  | The address of an Aleo account |
| boolean | [bool](#bool) |  | A true or false value |
| field | [Field](#operand-Field) |  | A native field element as an unsigned number up to the modulus length of the field |
| group | [Group](#operand-Group) |  | The set of affine points on the elliptic curve passed into the snarkd-vm forms a group. Snarkd supports this set as a primitive data type. Group elements are special since their values can be defined from the x-coordinate of a coordinate pair. |
| u8 | [uint32](#uint32) |  | An 8 bit unsigned integer |
| u16 | [uint32](#uint32) |  | A 16 bit unsigned integer |
| u32 | [uint32](#uint32) |  | A 32 bit unsigned integer |
| u64 | [uint64](#uint64) |  | A 64 bit unsigned integer |
| u128 | [bytes](#bytes) |  | A 128 bit unsigned integer |
| i8 | [int32](#int32) |  | An 8 bit signed integer |
| i16 | [int32](#int32) |  | A 16 bit signed integer |
| i32 | [int32](#int32) |  | A 32 bit signed integer |
| i64 | [int64](#int64) |  | A 64 bit signed integer |
| i128 | [bytes](#bytes) |  | A 128 bit signed integer |
| ref | [uint32](#uint32) |  | The ID of an input register containing the target value |
| scalar | [Scalar](#operand-Scalar) |  | Field elements in the scalar field. |
| string | [string](#string) |  | An array of characters. Snarkd currently only supports static strings for certian commit operations |
| struct | [Struct](#operand-Struct) |  | a complex data structure containing fields with other data. Internally represented as a tuple. |
| record | [Record](#operand-Record) |  | An arbitrary collection of user-owned state |






<a name="operand-Record"></a>

### Record
An arbitrary collection of user-owned state


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| owner | [VisibleData](#operand-VisibleData) |  | The owner of the record. This must always be an address |
| gates | [VisibleData](#operand-VisibleData) |  | TODO |
| data | [VisibleData](#operand-VisibleData) | repeated | The values held by the record |
| nonce | [VisibleData](#operand-VisibleData) |  | TODO |






<a name="operand-RecordType"></a>

### RecordType
The type and visibility information for a record


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| owner | [Visibility](#operand-Visibility) |  | The visibility status for the record owner |
| gates | [Visibility](#operand-Visibility) |  | The visibility status for the record gates |
| data | [RecordTypeEntry](#operand-RecordTypeEntry) | repeated | The fields within the record |
| nonce | [Visibility](#operand-Visibility) |  | The visibility status for the record nonce |






<a name="operand-RecordTypeEntry"></a>

### RecordTypeEntry
Type data for the field of a record


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | The name of the record entry |
| type | [Type](#operand-Type) |  | The data type of the record entry |
| visibility | [Visibility](#operand-Visibility) |  | The visibility status of the record entry |






<a name="operand-Scalar"></a>

### Scalar
Field elements in the scalar field.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [fixed64](#fixed64) | repeated | TODO size? |






<a name="operand-SimpleType"></a>

### SimpleType
An empty type used to get around the limitations of the `oneof` keyword






<a name="operand-Struct"></a>

### Struct
Acomplex data structure containing fields with
other data. Internally represented as a tuple.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [Operand](#operand-Operand) | repeated | a list of values held by the data structure |






<a name="operand-StructType"></a>

### StructType
The type information for a struct


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fields | [StructTypeEntry](#operand-StructTypeEntry) | repeated | The fields of the struct |






<a name="operand-StructTypeEntry"></a>

### StructTypeEntry
Type data for the field of a struct


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | The names of the field |
| type | [Type](#operand-Type) |  | The data type of the field |






<a name="operand-TupleGroup"></a>

### TupleGroup
TODO


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| left | [GroupCoordinate](#operand-GroupCoordinate) |  |  |
| right | [GroupCoordinate](#operand-GroupCoordinate) |  |  |






<a name="operand-Type"></a>

### Type
Type information for an unknown value


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [SimpleType](#operand-SimpleType) |  | The address of an Aleo account |
| boolean | [SimpleType](#operand-SimpleType) |  | A true or false value |
| field | [SimpleType](#operand-SimpleType) |  | A native field element as an unsigned number up to the modulus length of the field |
| group | [SimpleType](#operand-SimpleType) |  | The set of affine points on the elliptic curve passed into the snarkd-vm forms a group. Snarkd supports this set as a primitive data type. Group elements are special since their values can be defined from the x-coordinate of a coordinate pair. |
| u8 | [SimpleType](#operand-SimpleType) |  | An 8 bit unsigned integer |
| u16 | [SimpleType](#operand-SimpleType) |  | A 16 bit unsigned integer |
| u32 | [SimpleType](#operand-SimpleType) |  | A 32 bit unsigned integer |
| u64 | [SimpleType](#operand-SimpleType) |  | A 64 bit unsigned integer |
| u128 | [SimpleType](#operand-SimpleType) |  | A 128 bit unsigned integer |
| i8 | [SimpleType](#operand-SimpleType) |  | An 8 bit signed integer |
| i16 | [SimpleType](#operand-SimpleType) |  | A 16 bit signed integer |
| i32 | [SimpleType](#operand-SimpleType) |  | A 32 bit signed integer |
| i64 | [SimpleType](#operand-SimpleType) |  | A 64 bit signed integer |
| i128 | [SimpleType](#operand-SimpleType) |  | A 128 bit signed integer |
| scalar | [SimpleType](#operand-SimpleType) |  | Field elements in the scalar field. |
| string | [SimpleType](#operand-SimpleType) |  | An array of characters. Snarkd currently only supports static strings for certian commit operations |
| struct | [StructType](#operand-StructType) |  | a complex data structure containing fields with other data. Internally represented as a tuple. |
| record | [RecordType](#operand-RecordType) |  | An arbitrary collection of user-owned state |






<a name="operand-VisibleData"></a>

### VisibleData
A value and visibility pair used in a record


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [Operand](#operand-Operand) |  | the value held in the record |
| visibility | [Visibility](#operand-Visibility) |  | the visibility status of the value |








<a name="operand-Visibility"></a>

### Visibility
The possible visibility states for an input or record

| Name | Number | Description |
| ---- | ------ | ----------- |
| Constant | 0 | A public value, able to be seen but not set by others |
| Private | 1 | A private value, unable to be seen or set by others |
| Public | 2 | A public value, able to be seen and set by others |










## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

